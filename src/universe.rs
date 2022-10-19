use crate::{
    collision::{possible_collisions, Collision},
    constants,
    scalar::Scalar,
    units, Float, Object, Vector,
};

pub struct Universe<const N: usize> {
    objects: Vec<Object<N>>,
}

impl<const N: usize> Universe<N> {
    pub fn new() -> Universe<N> {
        Universe {
            objects: Vec::new(),
        }
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
        let electric_field = self.electric_field();
        self.resolve_collisions(&collisions, dt);
        for object in self.objects.iter_mut() {
            object
                .apply_force(electric_field(object.position()))
                .unwrap();
            object.update(dt);
        }
    }

    pub fn electric_field(&mut self) -> impl Fn(Vector<N>) -> Vector<N> {
        let charge_pos = self
            .objects
            .iter()
            .map(|object| (object.charge(), object.position()))
            .collect::<Vec<_>>();
        move |x| {
            constants::k_e
                * charge_pos.iter().fold(
                    Vector::zero() * units::of_electric_field_strength,
                    |acc, &(charge, pos)| {
                        acc + charge / (pos - x).magnitude().squared() * (pos - x).normalized()
                    },
                )
        }
    }

    fn find_collisions(&self) -> Vec<Collision<N>> {
        let mut collisions = Vec::new();
        let possible_collisions = possible_collisions(&self.objects);

        for (a, b) in possible_collisions {
            let obj_a = &self.objects[a];
            let obj_b = &self.objects[b];
            if let Some(direction) = obj_a.is_collision(obj_b) {
                collisions.push(Collision {
                    obj_a: a,
                    obj_b: b,
                    direction,
                });
            }
        }
        collisions
    }

    fn resolve_collisions(&mut self, collisions: &[Collision<N>], _dt: Scalar) {
        for collision in collisions {
            println!("Collision: {:?}", collision);

            let a = &self.objects[collision.obj_a];

            // let m1 = a.mass();
            // let v1 = a.velocity();
            // // let x1 = a.collider.get_bounding_box(&a.transform).center();

            let b = &self.objects[collision.obj_b];
            // let m2 = b.mass();
            // let v2 = b.velocity();
            // let x2 = b.collider.get_bounding_box(&b.transform).center();

            match (a.attributes().is_static, b.attributes().is_static) {
                (true, true) => (),
                (false, false) => {
                    // let a1 = 2.0 * m1 * (v2 - v1) / (m1 + m2) / dt;
                    // let a2 = 2.0 * m2 * (v1 - v2) / (m1 + m2) / dt;

                    // let a = &mut self.objects[collision.a];
                    // // a.force += a1;

                    // let b = &mut self.objects[collision.b];
                    // b.force += a2;

                    todo!()
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

impl<const N: usize> Default for Universe<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize, const T: usize> From<[Object<N>; T]> for Universe<N> {
    fn from(objects: [Object<N>; T]) -> Self {
        let mut world = Self::new();
        world.add_objects(objects);
        world
    }
}
