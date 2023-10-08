use crate::{constants, units, Float, Object, ObjectID, Scalar, Vector, VectorField};

pub struct Universe<const N: usize> {
    objects: Vec<Object<N>>,
    electric_fields: Vec<(Scalar, VectorField<'static, N>)>,
    gravitational_field: VectorField<'static, N>,
}

impl<const N: usize> Universe<N> {
    pub fn new() -> Universe<N> {
        Universe {
            objects: Vec::new(),
            electric_fields: vec![],
            gravitational_field: (
                |_| Vector::zero() * units::N / units::kg,
                units::N / units::kg,
            )
                .into(),
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
        // self.update_electric_field(dt);
        // self.update_gravitational_field(dt);
        let field = self.electric_field();

        for object in self.objects.iter_mut() {
            let force = field.at(object.position()).unwrap() * object.charge();
            object.update(dt, force);
        }
    }

    fn update_electric_field(&mut self, dt: Scalar) {
        let charge_pos = self
            .objects
            .iter()
            .map(|object| (object.charge(), object.position()))
            .collect::<Vec<_>>();
        for (t, _) in &mut self.electric_fields {
            *t += dt;
        }
        self.electric_fields.push((
            dt,
            (
                move |x: Vector<N>| {
                    constants::k_e()
                        * charge_pos.iter().fold(
                            Vector::zero() * units::N / units::C / constants::k_e().dim(),
                            |field, &(charge, pos)| {
                                let r = x - pos;
                                if r.is_zero() {
                                    field
                                } else {
                                    field + charge / r.dot(r) * r.normalized()
                                }
                            },
                        )
                },
                units::N / units::C,
            )
                .into(),
        ));
    }

    fn update_gravitational_field(&mut self, dt: Scalar) {
        let mass_pos = self
            .objects
            .iter()
            .map(|object| (object.mass(), object.position()))
            .collect::<Vec<_>>();
        self.gravitational_field
            .impose(
                dt * constants::c,
                (
                    move |x: Vector<N>| {
                        constants::G
                            * mass_pos.iter().fold(
                                Vector::zero() * units::N / units::kg / constants::G.dim(),
                                |field, &(mass, pos)| {
                                    let r = x - pos;
                                    if r.is_zero() {
                                        field
                                    } else {
                                        field + mass / r.dot(r) * r.normalized()
                                    }
                                },
                            )
                    },
                    units::N / units::kg,
                )
                    .into(),
            )
            .unwrap();
    }

    pub(crate) fn electric_field(&self) -> VectorField<'static, N> {
        let mut field: VectorField<'static, N> = (
            |_| Vector::zero() * units::N / units::C,
            units::N / units::C,
        )
            .into();
        for (t, new) in &self.electric_fields {
            field.impose(*t * constants::c, new.clone()).unwrap();
        }
        field
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
