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
        let gravitational_field = self.gravitational_field();
        self.resolve_collisions(&collisions, dt);
        for object in self.objects.iter_mut() {
            object
                .apply_force(
                    electric_field(object.position()) * object.charge()
                        + gravitational_field(object.position()) * object.mass(),
                )
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
                    Vector::zero() * units::of_electric_field_strength / constants::k_e.unit(),
                    |acc, &(charge, pos)| {
                        let diff = x - pos;
                        if diff.is_zero() {
                            acc
                        } else {
                            acc + charge / diff.magnitude().squared() * diff.normalized()
                        }
                    },
                )
        }
    }

    /// Classical Newtonian Gravitation
    pub fn gravitational_field(&mut self) -> impl Fn(Vector<N>) -> Vector<N> {
        let mass_pos = self
            .objects
            .iter()
            .map(|object| (object.mass(), object.position()))
            .collect::<Vec<_>>();
        move |x| {
            constants::G
                * mass_pos.iter().fold(
                    Vector::zero() * units::kg / units::m.pow(2),
                    |acc, &(mass, pos)| {
                        let diff = x - pos;
                        if diff.is_zero() {
                            acc
                        } else {
                            acc - mass / diff.magnitude().squared() * diff.normalized()
                        }
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
            if let Some(normal) = obj_a.is_collision(obj_b) {
                collisions.push(Collision {
                    obj_a: a,
                    obj_b: b,
                    normal,
                });
            }
        }
        collisions
    }

    fn resolve_collisions(&mut self, collisions: &[Collision<N>], _dt: Scalar) {
        for &Collision {
            obj_a,
            obj_b,
            normal,
        } in collisions
        {
            let a = &self.objects[obj_a];
            let b = &self.objects[obj_b];

            if a.attributes().is_static && b.attributes().is_static {
                continue;
            }

            let u_a = a.velocity();
            let u_b = b.velocity();
            let m_a = a.mass();
            let m_b = b.mass();
            let e = a
                .attributes()
                .restitution_coefficient
                .max(b.attributes().restitution_coefficient);
            let n = normal.normalized();

            let j = -(1.0 + e) * (u_a - u_b).dot(&n) / (m_a.recip() + m_b.recip());
            let n_j = j * n;

            match (a.attributes().is_static, b.attributes().is_static) {
                (true, true) => (),
                (false, false) => {
                    let v_a = u_a + n_j / m_a;
                    let v_b = u_b - n_j / m_b;

                    self.objects[obj_a].set_velocity(v_a);
                    self.objects[obj_b].set_velocity(v_b);
                }
                (false, true) => {
                    let v_a = u_a + n_j / m_a;
                    self.objects[obj_a].set_velocity(v_a);
                }
                (true, false) => {
                    let v_b = u_b - n_j / m_b;
                    self.objects[obj_b].set_velocity(v_b);
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
