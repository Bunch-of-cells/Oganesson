#![allow(non_snake_case)]
use crate::{
    collision::possible_collisions, constants, h, units, Float, Object, ObjectID, Vector, STEP,
};

pub struct Universe<const N: usize> {
    objects: Vec<Object<N>>,
    field_g: Vector<N>,
    field_E: Vector<N>,
    field_B: Vector<N>,
}

impl<const N: usize> Universe<N> {
    pub fn new() -> Universe<N> {
        Universe {
            objects: Vec::new(),
            field_g: Vector::zero() * units::N / units::kg,
            field_E: Vector::zero() * units::N / units::C,
            field_B: Vector::zero() * units::T,
        }
    }

    pub fn add_gravitational_field(&mut self, g: Vector<N>) {
        self.field_g = g;
    }

    pub fn add_electric_field(&mut self, E: Vector<N>) {
        self.field_E = E;
    }

    pub fn add_magnetic_field(&mut self, B: Vector<N>) {
        self.field_B = B;
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
        for _ in 0..(dt / STEP) as usize {
            let f = self.objects.clone();
            for (i, object) in self.objects.iter_mut().enumerate() {
                let v = object.velocity + 0.5 * h() * object.acc;
                object.position += v * h();

                // Calculate force
                let mut g = f.clone();
                g[i].position = object.position;
                let force = Self::force(&g, i, object, self.field_g, self.field_E, self.field_B);
                object.acc = object.acceleration(force);

                object.velocity = v + object.acc * h() * 0.5;
            }
            self.resolve_collisions();
        }
    }

    fn force(
        f: &[Object<N>],
        i: usize,
        object: &Object<N>,
        g: Vector<N>,
        E: Vector<N>,
        B: Vector<N>,
    ) -> Vector<N> {
        let mut force = Vector::zero() * units::N;
        for (j, obj) in f.iter().enumerate() {
            if j == i {
                continue;
            }
            let r1 = object.position();
            let r = obj.position() - r1;
            force += r.normalized()
                * (constants::G * object.mass() * obj.mass()
                    - constants::k_e() * object.charge() * obj.charge())
                / r.squared()
        }
        force += object.charge() * E + object.mass() * g;
        let vB = if N == 3 {
            (object.velocity[1] * B[2] - object.velocity[2] * B[1]) * Vector::basis(0)
                - (object.velocity[0] * B[2] - object.velocity[2] * B[0]) * Vector::basis(1)
                + (object.velocity[0] * B[1] - object.velocity[1] * B[0]) * Vector::basis(2)
        } else {
            panic!("B field in non 3D space");
        };
        force += object.charge() * vB * units::N / units::C;
        force
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

                let e = 0.5
                    * (a.attributes().restitution_coefficient
                        + b.attributes().restitution_coefficient);

                let n = normal.normalized();
                let j = -(1.0 + e) * (u_a - u_b).dot(n) / (m_a.recip() + m_b.recip()) * n;
                self.objects[obj_a].acc = 2.0 * j / (m_a * h());
                self.objects[obj_b].acc = -2.0 * j / (m_b * h());
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
