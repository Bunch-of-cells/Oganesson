use crate::{unit::UnitError, units, Collider, Scalar, Transform, Vector};

#[derive(Clone, Debug)]
pub struct Object<const N: usize> {
    pub(crate) velocity: Vector<N>,
    pub(crate) mass: Scalar,
    pub(crate) transform: Transform<N>,
    pub(crate) acceleration: Vector<N>,
    pub(crate) collider: Collider<N>,
    pub(crate) properties: ObjectProperty,
}

impl<const N: usize> Object<N> {
    pub fn new(
        position: Vector<N>,
        velocity: Vector<N>,
        mass: Scalar,
        collider: Collider<N>,
    ) -> Result<Object<N>, UnitError> {
        position.get_uniterror(units::m, "position")?;
        velocity.get_uniterror(units::m / units::s, "velocity")?;
        mass.get_uniterror(units::kg, "mass")?;

        match collider {
            Collider::Sphere { radius } => {
                radius.get_uniterror(units::m, "collider::sphere::radius")?;
                assert!(radius.value() >= 0.0);
            }
            Collider::Quad {
                dimentions: distance,
            } => {
                distance.get_uniterror(units::m, "collider::plane::distance")?;
            }
            Collider::Polyline { points } => {
                assert!(!points.is_empty());
                todo!()
            }
        }

        Ok(Object {
            velocity,
            acceleration: Vector::zero() * units::of_acceleration,
            mass,
            transform: Transform::new(position),
            collider,
            properties: ObjectProperty::default(),
        })
    }

    pub fn with_property(mut self, property: ObjectProperty) -> Self {
        self.properties = property;
        self
    }

    pub(crate) fn update(&mut self, dt: Scalar) {
        if !self.properties.is_static {
            self.velocity += self.acceleration * dt;
        }
        self.transform.position += self.velocity * dt;
        self.acceleration = Vector::zero() * units::of_acceleration;
    }

    pub fn velocity(&self) -> Vector<N> {
        self.velocity
    }

    pub fn force(&self) -> Vector<N> {
        self.acceleration
    }

    pub fn mass(&self) -> Scalar {
        self.mass
    }

    pub fn transform(&self) -> &Transform<N> {
        &self.transform
    }

    pub fn position(&self) -> Vector<N> {
        self.transform.position
    }

    pub fn collider(&self) -> &Collider<N> {
        &self.collider
    }

    pub fn apply_force(&mut self, force: Vector<N>) -> Result<(), UnitError> {
        force.get_uniterror(units::N, "force")?;
        self.acceleration += force / self.mass;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct ObjectProperty {
    pub is_static: bool,
}

impl Default for ObjectProperty {
    fn default() -> Self {
        ObjectProperty { is_static: false }
    }
}
