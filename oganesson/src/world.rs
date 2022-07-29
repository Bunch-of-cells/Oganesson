use crate::{
    collision::Collision, scalar::Scalar, unit::UnitError, units, Collider, Float, Quaternion,
    Transform, Vector,
};

#[derive(Clone, Debug)]
pub struct Object<const N: usize> {
    velocity: Vector<N>,
    force: Vector<N>,
    mass: Scalar,
    transform: Transform<N>,
    collider: Collider<N>,
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
            }
            Collider::Plane {
                dimentions: distance,
            } => {
                distance.get_uniterror(units::m, "collider::plane::distance")?;
            }
        }

        Ok(Object {
            velocity,
            force: Vector::zero() * units::N,
            mass,
            transform: Transform {
                position,
                scale: Vector([1.0; N], units::Null),
                rotation: Quaternion {
                    w: 0.0,
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            collider,
        })
    }

    fn update(&mut self, dt: Scalar) {
        self.velocity += self.force * dt / self.mass;
        self.transform.position += self.velocity * dt;
    }

    pub fn velocity(&self) -> Vector<N> {
        self.velocity
    }

    pub fn force(&self) -> Vector<N> {
        self.force
    }

    pub fn mass(&self) -> Scalar {
        self.mass
    }

    pub fn transform(&self) -> &Transform<N> {
        &self.transform
    }

    pub fn collider(&self) -> &Collider<N> {
        &self.collider
    }
}

pub struct PhysicsWorld<const N: usize> {
    objects: Vec<Object<N>>,
    gravity: Vector<N>,
}

impl<const N: usize> PhysicsWorld<N> {
    pub fn new(gravity: Vector<N>) -> Result<PhysicsWorld<N>, UnitError> {
        gravity.get_uniterror(units::of_acceleration, "gravity")?;

        Ok(PhysicsWorld {
            objects: Vec::new(),
            gravity,
        })
    }

    pub fn objects(&self) -> &[Object<N>] {
        &self.objects
    }

    pub fn add_object(&mut self, object: Object<N>) -> &mut Self {
        self.objects.push(object);
        self
    }

    pub fn remove_objects<F>(&mut self, f: F)
    where
        F: FnMut(&Object<N>) -> bool,
    {
        self.objects.retain(f);
    }

    pub fn step(&mut self, dt: Float) {
        let dt = dt * units::s;
        let collisions = self.find_collisions();
        self.resolve_collisions(&collisions, dt);
        for object in self.objects.iter_mut() {
            object.force += self.gravity * object.mass;
            object.update(dt);
            println!("{:?}", object);
            object.force = Vector::zero() * units::N;
        }
    }

    fn find_collisions(&self) -> Vec<Collision<N>> {
        let mut collisions = Vec::new();

        for (a, obj_a) in self.objects.iter().enumerate() {
            for (b, obj_b) in self.objects.iter().enumerate() {
                if a == b {
                    break;
                }

                if obj_a
                    .collider
                    .is_collision(&obj_a.transform, &obj_b.collider, &obj_b.transform)
                {
                    collisions.push(Collision::new(
                        a,
                        b,
                        obj_a.transform.position,
                        obj_b.transform.position,
                    ));
                }
            }
        }

        collisions
    }

    pub fn resolve_collisions(&mut self, collisions: &[Collision<N>], _dt: Scalar) {
        for collision in collisions {
            let m1 = self.objects[collision.a].mass;
            let m2 = self.objects[collision.b].mass;
            let v1 = self.objects[collision.a].velocity;
            let v2 = self.objects[collision.b].velocity;

            let v2_prime = (2.0 * m1 * v1 + (m2 - m1) * v2) / (m2 + m1);
            let v1_prime = (2.0 * m2 * v2 + (m1 - m2) * v1) / (m2 + m1);

            let a = &mut self.objects[collision.a];
            a.velocity = v1_prime;

            let b = &mut self.objects[collision.b];
            b.velocity = v2_prime;
        }
    }
}

impl<const N: usize> Default for PhysicsWorld<N> {
    fn default() -> Self {
        Self::new(Vector::zero() * units::N).unwrap()
    }
}
