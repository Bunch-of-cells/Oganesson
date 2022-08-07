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

    pub fn add_objects(&mut self, objects: impl IntoIterator<Item = Object<N>>) -> &mut Self {
        self.objects.extend(objects);
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
            println!("Collision: {:?}", collision);

            let a = &self.objects[collision.a];

            let m1 = a.mass;
            let v1 = a.velocity;
            // let x1 = a.collider.get_bounding_box(&a.transform).center();

            let b = &self.objects[collision.b];
            let m2 = b.mass;
            let v2 = b.velocity;
            // let x2 = b.collider.get_bounding_box(&b.transform).center();

            match (a.properties.is_static, b.properties.is_static) {
                (true, true) => (),
                (false, false) => {
                    let a1 = 2.0 * m1 * (v2 - v1) / (m1 + m2) / dt;
                    let a2 = 2.0 * m2 * (v1 - v2) / (m1 + m2) / dt;

                    let a = &mut self.objects[collision.a];
                    a.acceleration += a1;

                    let b = &mut self.objects[collision.b];
                    b.acceleration += a2;
                }
                (true, false) => {
                    todo!()
                }
                (false, true) => {
                    todo!()
                    // let x1_x2_diff = x1 - x2;
                    // let a = &mut self.objects[collision.a];
                    // let v1_prime = (v1 - v2).dot(&x1_x2_diff) / x1_x2_diff.magnitude()
                    //     * x1_x2_diff.normalized();
                    // a.acceleration += (v1_prime - v1) / dt
                }
            }
        }
    }
}

impl<const N: usize> Default for PhysicsWorld<N> {
    fn default() -> Self {
        Self::new(Vector::zero() * units::of_acceleration).unwrap()
    }
}

impl<const N: usize, const T: usize> From<(Vector<N>, [Object<N>; T])> for PhysicsWorld<N> {
    fn from(args: (Vector<N>, [Object<N>; T])) -> Self {
        let mut world = Self::new(args.0).unwrap();
        world.add_objects(args.1);
        world
    }
}

impl<const N: usize, const T: usize> From<[Object<N>; T]> for PhysicsWorld<N> {
    fn from(objects: [Object<N>; T]) -> Self {
        let mut world = Self::default();
        world.add_objects(objects);
        world
    }
}
