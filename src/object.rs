use std::fmt::Debug;

use crate::{dimension::DimensionError, units, Collider, Float, Scalar, Vector};
use macroquad::color::{Color, WHITE};

pub struct ObjectBuilder<const N: usize> {
    velocity: Vector<N>,
    mass: Scalar,
    position: Vector<N>,
    charge: Scalar,
    color: Color,
    size: Scalar,
    attributes: ObjectAttributes,
}

impl<const N: usize> ObjectBuilder<N> {
    pub fn new_at(position: Vector<N>) -> Self {
        ObjectBuilder {
            position,
            velocity: Vector::zero() * units::m / units::s,
            mass: 1.0 * units::kg,
            charge: 0.0 * units::C,
            size: 1.0 * units::m,
            attributes: ObjectAttributes::default(),
            color: WHITE,
        }
    }

    pub fn build(self) -> Result<Object<N>, DimensionError> {
        self.position.dimension_err(units::m, "position")?;
        self.velocity
            .dimension_err(units::m / units::s, "velocity")?;
        self.mass.dimension_err(units::kg, "mass")?;
        self.charge.dimension_err(units::C, "charge")?;
        self.size.dimension_err(units::m, "size")?;

        let intrinsic = IntrinsicProperty {
            mass: self.mass,
            charge: self.charge,
            color: self.color,
            size: self.size,
            attributes: self.attributes,
        };

        let object = Object {
            intrinsic,
            position: self.position,
            velocity: [self.velocity; 4],
        };

        Ok(object)
    }

    #[inline(always)]
    pub fn with_velocity(mut self, velocity: Vector<N>) -> Self {
        self.velocity = velocity;
        self
    }

    #[inline(always)]
    pub fn with_mass(mut self, mass: Scalar) -> Self {
        self.mass = mass;
        self
    }

    #[inline(always)]
    pub fn with_charge(mut self, charge: Scalar) -> Self {
        self.charge = charge;
        self
    }

    #[inline(always)]
    pub fn with_size(mut self, size: Scalar) -> Self {
        self.size = size;
        self
    }

    #[inline(always)]
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    #[inline(always)]
    pub fn with_attributes(mut self, attributes: ObjectAttributes) -> Self {
        self.attributes = attributes;
        self
    }
}

#[derive(Clone)]
pub struct Object<const N: usize> {
    velocity: [Vector<N>; 4],
    position: Vector<N>,
    intrinsic: IntrinsicProperty,
}

impl<const N: usize> Object<N> {
    pub(crate) fn update(&mut self, dt: Scalar, force: Vector<N>) {
        let velocity = self.acceleration(force) * dt
            + (self.velocity[0]
                + 3.0 * self.velocity[1]
                + 3.0 * self.velocity[2]
                + self.velocity[3])
                / 8.0;

        self.position += velocity * dt;
        self.velocity.rotate_left(1);
        self.velocity[3] = velocity;
    }

    #[inline(always)]
    fn acceleration(&mut self, force: Vector<N>) -> Vector<N> {
        self.inv_lorentz_factor() / self.mass()
            * (force - force.dot(self.velocity()) * self.velocity() / crate::constants::c2())
    }

    pub(crate) fn set_velocity(&mut self, velocity: Vector<N>) {
        self.velocity = [velocity; 4];
    }

    pub(crate) fn set_position(&mut self, position: Vector<N>) {
        self.position = position;
    }

    pub fn collider(&self) -> Collider<N> {
        Collider {
            position: self.position,
            size: self.intrinsic.size,
        }
    }

    // Getters

    #[inline(always)]
    pub fn velocity(&self) -> Vector<N> {
        self.velocity[3]
    }

    #[inline(always)]
    pub fn mass(&self) -> Scalar {
        self.intrinsic.mass
    }

    pub fn charge(&self) -> Scalar {
        self.intrinsic.charge
    }

    #[inline(always)]
    pub fn position(&self) -> Vector<N> {
        self.position
    }

    #[inline(always)]
    pub fn size(&self) -> Scalar {
        self.intrinsic.size
    }

    #[inline(always)]
    pub fn intrinsic_properties(&self) -> &IntrinsicProperty {
        &self.intrinsic
    }

    #[inline(always)]
    pub fn attributes(&self) -> &ObjectAttributes {
        &self.intrinsic.attributes
    }

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

    #[inline(always)]
    /// Calculate the inverse of lorentz factor (1/γ)
    pub fn inv_lorentz_factor(&self) -> Scalar {
        if self.velocity[3].is_zero() {
            return 1.0.into();
        }
        (1.0 - (self.velocity[3].squared() / crate::constants::c2())).powf(0.5)
    }

    #[inline(always)]
    /// E = KE + rest energy = γmc2
    pub fn internal_energy(&self) -> Scalar {
        self.intrinsic.mass * crate::constants::c2() * self.lorentz_factor()
    }

    #[inline(always)]
    pub fn color(&self) -> Color {
        self.intrinsic.color
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectID(pub(crate) usize);

#[derive(Clone, Debug)]
pub struct IntrinsicProperty {
    pub mass: Scalar,
    pub charge: Scalar,
    pub attributes: ObjectAttributes,
    pub size: Scalar,
    pub color: Color,
}

#[derive(Clone, Debug)]
pub struct ObjectAttributes {
    pub restitution_coefficient: Float,
}

impl Default for ObjectAttributes {
    fn default() -> Self {
        Self {
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
            .field("attrs", &self.attributes())
            .field("color", &self.color());

        s.finish()
    }
}
