use std::fmt::Debug;

use crate::{
    field::ScalarField, unit::UnitError, units, Collider, Float, Scalar, Transform, Vector,
};

#[cfg(feature = "simulation")]
use ggez::graphics::Color;

#[derive(Clone)]
pub struct Object<const N: usize> {
    velocity: [Vector<N>; 4],
    intrinsic: IntrinsicProperty<N>,
    transform: [Transform<N>; 2],
}

impl<const N: usize> Object<N> {
    pub fn new(
        position: Vector<N>,
        velocity: Vector<N>,
        intrinsic: IntrinsicProperty<N>,
    ) -> Result<Object<N>, UnitError> {
        position.get_uniterror(units::m, "position")?;
        velocity.get_uniterror(units::m / units::s, "velocity")?;
        intrinsic.mass.get_uniterror(units::kg, "mass")?;
        intrinsic.charge.get_uniterror(units::C, "charge")?;

        match intrinsic.collider {
            Collider::Sphere { radius } => {
                radius.get_uniterror(units::m, "collider::sphere::radius")?;
                assert!(radius.value() > 0.0);
            }

            Collider::Polygon { ref points } => {
                assert!(points.len() > N);
                for point in points {
                    point.get_uniterror(units::Null, "collider::polygon::points")?;
                }
            }
            Collider::Plane { normal } => {
                normal.get_uniterror(units::Null, "collider::line::normal")?;
                assert!(normal.magnitude() > 0.0);
            }
            Collider::Triangle { a, b, c } => {
                a.get_uniterror(units::Null, "collider::triange::a")?;
                b.get_uniterror(units::Null, "collider::triange::a")?;
                c.get_uniterror(units::Null, "collider::triange::a")?;
            }
        }

        #[cfg(feature = "relativistic")]
        match velocity
            .magnitude()
            .partial_cmp(&crate::constants::c.value())
        {
            Some(std::cmp::Ordering::Equal) => assert!(
                mass.value() == 0.0,
                "Cannot have non zero mass for a particle travelling at light speed"
            ),
            Some(std::cmp::Ordering::Greater) => panic!("Cannot travel faster than light"),
            _ => assert!(
                mass.value() != 0.0,
                "Cannot have zero mass for a particle below light speed"
            ),
        }

        Ok(Object {
            velocity: [velocity; 4],
            transform: [Transform::new(position), Transform::new(position)],
            intrinsic,
        })
    }

    pub(crate) fn update(&mut self, dt: Scalar, fields: &ScalarField<'_, N>) {
        let acceleration = self.acceleration(fields);
        let velocity = acceleration * dt
            + (self.velocity[0]
                + 3.0 * self.velocity[1]
                + 3.0 * self.velocity[2]
                + self.velocity[3])
                / 8.0;
        self.transform[0] = self.transform[1].clone();
        self.transform[1].position += velocity * dt;
        self.velocity.rotate_left(1);
        self.velocity[3] = velocity;
    }

    pub(crate) fn set_velocity(&mut self, velocity: Vector<N>) {
        self.velocity = [velocity; 4];
    }

    pub fn is_collision(&self, other: &Object<N>) -> Option<Vector<N>> {
        match (self.collider(), other.collider()) {
            (&Collider::Sphere { radius: r1 }, &Collider::Sphere { radius: r2 }) => {
                let distance = self.position() - other.position();
                let direction = distance.normalized();
                let distance = distance.magnitude().abs();
                if distance >= r1 + r2 {
                    None
                } else {
                    let mut normal = direction * (r1 + r2 - distance);
                    normal.1 = units::Null;
                    Some(normal)
                }
            }

            (Collider::Polygon { .. }, Collider::Sphere { .. }) => None,
            (Collider::Sphere { .. }, Collider::Polygon { .. }) => {
                other.is_collision(self).map(|v| -v)
            }

            (&Collider::Sphere { radius: _r }, &Collider::Plane { normal: n }) => {
                let c = other.position();
                let v = self.velocity[2]; // previous velocity (of last frame)
                let x = self.transform[0].position; // previous velocity (of last frame)
                let i = x + (x - c).dot(&n) / v.dot(&n) * v;
                todo!("{i:?}")
                // Some(i)
            }
            (Collider::Plane { .. }, Collider::Sphere { .. }) => {
                other.is_collision(self).map(|v| -v)
            }

            _ => todo!(),
        }
    }

    #[cfg(not(feature = "relativistic"))]
    #[inline(always)]
    pub fn acceleration(&mut self, fields: &ScalarField<'_, N>) -> Vector<N> {
        let force = -fields.gradient().at(self.position()).unwrap();
        dbg!(force);
        force / self.mass()
    }

    #[cfg(feature = "relativistic")]
    #[inline(always)]
    pub fn acceleration(&mut self, fields: &ScalarField<'_, N>) -> Vector<N> {
        let force = -fields.gradient().at(self.position());
        self.inv_lorentz_factor() / self.mass()
            * (force - force.dot(&self.velocity()) * self.velocity() / crate::constants::c2)
    }

    #[inline(always)]
    pub fn velocity(&self) -> Vector<N> {
        self.velocity[3]
    }

    #[cfg(not(feature = "relativistic"))]
    #[inline(always)]
    pub fn mass(&self) -> Scalar {
        self.intrinsic.mass
    }

    #[cfg(feature = "relativistic")]
    #[inline(always)]
    pub fn mass(&self) -> Scalar {
        self.intrinsic.mass * self.lorentz_factor()
    }

    #[cfg(feature = "relativistic")]
    #[inline(always)]
    pub fn rest_mass(&self) -> Scalar {
        self.intrinsic.mass
    }

    pub fn charge(&self) -> Scalar {
        self.intrinsic.charge
    }

    #[inline(always)]
    pub fn transform(&self) -> &Transform<N> {
        &self.transform[1]
    }

    #[inline(always)]
    pub fn position(&self) -> Vector<N> {
        self.transform[1].position
    }

    #[inline(always)]
    pub fn collider(&self) -> &Collider<N> {
        &self.intrinsic.collider
    }

    #[inline(always)]
    pub fn intrinsic_properties(&self) -> &IntrinsicProperty<N> {
        &self.intrinsic
    }

    #[inline(always)]
    pub fn attributes(&self) -> &ObjectAttributes {
        &self.intrinsic.attributes
    }

    #[cfg(not(feature = "relativistic"))]
    #[inline(always)]
    /// KE = 1/2 m v^2
    pub fn kinetic_energy(&self) -> Scalar {
        self.intrinsic.mass * self.velocity[3].squared() * 0.5
    }

    #[cfg(feature = "relativistic")]
    #[inline(always)]
    /// KE = E - mc2
    pub fn kinetic_energy(&self) -> Scalar {
        self.intrinsic.mass * crate::constants::c2 * (self.lorentz_factor() - 1.0)
    }

    #[cfg(not(feature = "relativistic"))]
    #[inline(always)]
    /// p = mv
    pub fn momentum(&self) -> Vector<N> {
        self.intrinsic.mass * self.velocity[3]
    }

    #[cfg(feature = "relativistic")]
    #[inline(always)]
    /// p = γmv
    pub fn momentum(&self) -> Vector<N> {
        self.intrinsic.mass * self.velocity[3] * self.lorentz_factor()
    }

    #[cfg(feature = "relativistic")]
    #[inline(always)]
    /// Calculate the lorentz factor (γ)
    pub fn lorentz_factor(&self) -> Scalar {
        if self.velocity[3].is_zero() {
            return 1.0.into();
        }
        let den = self.inv_lorentz_factor();
        assert!(den > 0.0, "The object is travelling at light speed");
        1.0 / den
    }

    #[cfg(feature = "relativistic")]
    #[inline(always)]
    /// Calculate the inverse of lorentz factor (1/γ)
    pub fn inv_lorentz_factor(&self) -> Scalar {
        if self.velocity[3].is_zero() {
            return 1.0.into();
        }
        (1.0 - (self.velocity[3].squared() / crate::constants::c2)).powf(0.5)
    }

    #[cfg(feature = "relativistic")]
    #[inline(always)]
    /// E = γmc2
    pub fn energy(&self) -> Scalar {
        self.intrinsic.mass * crate::constants::c2 * self.lorentz_factor()
    }

    #[cfg(feature = "simulation")]
    #[inline(always)]
    pub fn color(&self) -> Color {
        self.intrinsic.color
    }
}

#[derive(Clone, Debug)]
pub struct IntrinsicProperty<const N: usize> {
    pub mass: Scalar,
    pub charge: Scalar,
    pub collider: Collider<N>,
    pub attributes: ObjectAttributes,
    #[cfg(feature = "simulation")]
    pub color: Color,
}

impl<const N: usize> IntrinsicProperty<N> {
    #[cfg(not(feature = "simulation"))]
    pub fn new(mass: Scalar, collider: Collider<N>) -> Result<IntrinsicProperty<N>, UnitError> {
        mass.get_uniterror(units::kg, "mass")?;

        match collider {
            Collider::Sphere { radius } => {
                radius.get_uniterror(units::m, "collider::sphere::radius")?;
                assert!(radius.value() > 0.0);
            }

            Collider::Polygon { ref points } => {
                assert!(points.len() > N);
                for point in points {
                    point.get_uniterror(units::Null, "collider::polygon::points")?;
                }
            }
            Collider::Plane { normal } => {
                normal.get_uniterror(units::Null, "collider::line::normal")?;
                assert!(normal.magnitude() > 0.0);
            }
            Collider::Triangle { a, b, c } => {
                a.get_uniterror(units::Null, "collider::triange::a")?;
                b.get_uniterror(units::Null, "collider::triange::a")?;
                c.get_uniterror(units::Null, "collider::triange::a")?;
            }
        }
        Ok(Self {
            mass,
            collider,
            attributes: ObjectAttributes::default(),
            charge: Scalar::zero() * units::C,
        })
    }

    #[cfg(feature = "simulation")]
    pub fn new(
        mass: Scalar,
        collider: Collider<N>,
        color: Color,
    ) -> Result<IntrinsicProperty<N>, UnitError> {
        mass.get_uniterror(units::kg, "mass")?;

        match collider {
            Collider::Sphere { radius } => {
                radius.get_uniterror(units::m, "collider::sphere::radius")?;
                assert!(radius.value() > 0.0);
            }

            Collider::Polygon { ref points } => {
                assert!(points.len() > N);
                for point in points {
                    point.get_uniterror(units::Null, "collider::polygon::points")?;
                }
            }
            Collider::Plane { normal } => {
                normal.get_uniterror(units::Null, "collider::line::normal")?;
                assert!(normal.magnitude() > 0.0);
            }
            Collider::Triangle { a, b, c } => {
                a.get_uniterror(units::Null, "collider::triange::a")?;
                b.get_uniterror(units::Null, "collider::triange::a")?;
                c.get_uniterror(units::Null, "collider::triange::a")?;
            }
        }
        Ok(Self {
            mass,
            collider,
            attributes: ObjectAttributes::default(),
            charge: Scalar::zero() * units::C,
            color,
        })
    }

    pub fn with_charge(mut self, charge: Scalar) -> Result<IntrinsicProperty<N>, UnitError> {
        charge.get_uniterror(units::C, "charge")?;
        self.charge = charge;
        Ok(self)
    }

    pub fn with_attributes(mut self, attributes: ObjectAttributes) -> IntrinsicProperty<N> {
        self.attributes = attributes;
        self
    }
}

#[derive(Clone, Debug)]
pub struct ObjectAttributes {
    pub is_static: bool,
    pub restitution_coefficient: Float,
}

impl Default for ObjectAttributes {
    fn default() -> Self {
        Self {
            is_static: false,
            restitution_coefficient: 1.0,
        }
    }
}

impl<const N: usize> Debug for Object<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Object");

        s.field("position", &self.position())
            .field("velocity", &self.velocity())
            .field("mass", &self.mass())
            .field("charge", &self.charge())
            .field("collider", &self.collider())
            .field("attrs", &self.attributes());

        #[cfg(feature = "simulation")]
        s.field("color", &self.color());

        s.finish()
    }
}
