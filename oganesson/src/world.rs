use crate::{
    collision::{possible_collisions, Collision},
    scalar::Scalar,
    unit::UnitError,
    units, Float, Object, Vector,
};

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
            object.acceleration += self.gravity;
            object.update(dt);
            println!("{:?}", object);
        }
    }

    fn find_collisions(&self) -> Vec<Collision<N>> {
        let mut collisions = Vec::new();
        let possible_collisions = possible_collisions(&self.objects);

        for (a, b) in possible_collisions {
            let obj_a = &self.objects[a];
            let obj_b = &self.objects[b];
            if let Some(direction) =
                obj_a
                    .collider
                    .is_collision(&obj_a.transform, &obj_b.collider, &obj_b.transform)
            {
                collisions.push(Collision::new(a, b, direction));
            }
        }

        collisions
    }

    fn resolve_collisions(&mut self, collisions: &[Collision<N>], dt: Scalar) {
        for collision in collisions {
            let a = &self.objects[collision.a];
            if a.properties.is_static {

            }
            let m1 = a.mass;
            let m2 = a.mass;

            let b = &self.objects[collision.b];
            let v1 = b.velocity;
            let v2 = b.velocity;

            let v1_prime = (2.0 * m2 * v2 + (m1 - m2) * v1) / (m2 + m1);
            let v2_prime = (2.0 * m1 * v1 + (m2 - m1) * v2) / (m2 + m1);

            let a = &mut self.objects[collision.a];
            a.acceleration += (v1_prime - v1) / dt;

            let b = &mut self.objects[collision.b];
            b.acceleration += (v2_prime - v2) / dt;
        }
    }
}

impl<const N: usize> Default for PhysicsWorld<N> {
    fn default() -> Self {
        Self::new(Vector::zero() * units::of_acceleration).unwrap()
    }
}
