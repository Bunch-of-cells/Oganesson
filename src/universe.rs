use crate::{collision::possible_collisions, constants, units, Float, Object, ObjectID, Vector};

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

    pub fn add_object(&mut self, object: Object<N>) -> ObjectID {
        self.objects.push(object);
        ObjectID(self.objects.len() - 1)
    }

    pub fn delete_object(&mut self, object: ObjectID) -> Object<N> {
        self.objects.remove(object.0)
    }

    pub fn with_objects(&mut self, objects: impl IntoIterator<Item = Object<N>>) -> &mut Self {
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
        let mut f = Vec::new();
        for obj in self.objects.iter() {
            f.push((obj.mass(), obj.position()))
        }
        for (i, object) in self.objects.iter_mut().enumerate() {
            if object.velocity().magnitude() < 0.1 {
                let p = object.position();
                println!("{i} : {p:?}");
            }
            let mut force = Vector::zero() * units::N;
            for (j, (m, r2)) in f.iter().enumerate() {
                if j == i {
                    continue;
                }
                let r1 = object.position();
                let r = *r2 - r1;
                force += r.normalized() * constants::G * object.mass() * *m / r.squared()
            }
            object.update(dt, force);
        }
        self.resolve_collisions();
    }

    fn resolve_collisions(&mut self) {
        let possible_collisions = possible_collisions(&self.objects);

        for (obj_a, obj_b) in possible_collisions {
            let a = &self.objects[obj_a];
            let b = &self.objects[obj_b];
            if let Some(normal) = a.collider().collides(&b.collider()) {
                let u_a = a.velocity();
                let u_b = b.velocity();
                let m_a = a.mass();
                let m_b = b.mass();
                let x_a = a.position();
                let x_b = b.position();

                let e = a
                    .attributes()
                    .restitution_coefficient
                    .max(b.attributes().restitution_coefficient);

                let n = normal.normalized();
                let j = -(1.0 + e) * (u_a - u_b).dot(n) / (m_a.recip() + m_b.recip()) * n;

                let v_a = u_a + j / m_a;
                let v_b = u_b - j / m_b;
                self.objects[obj_a].set_velocity(v_a);
                self.objects[obj_b].set_velocity(v_b);
                self.objects[obj_a].set_position(x_a + normal / 2.0);
                self.objects[obj_b].set_position(x_b - normal / 2.0);
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
        world.with_objects(objects);
        world
    }
}
