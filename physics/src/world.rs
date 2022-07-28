use crate::{
    scalar::Scalar, unit::UnitError, units, Collider, Collision, Solver, Transform, Vector,
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
        transform: Transform<N>,
        velocity: Vector<N>,
        force: Vector<N>,
        mass: Scalar,
        collider: Collider<N>,
    ) -> Result<Object<N>, UnitError> {
        velocity.is_of_unit(units::m / units::s)?;
        force.is_of_unit(units::N)?;
        mass.is_of_unit(units::kg)?;

        Ok(Object {
            velocity,
            force,
            mass,
            transform,
            collider,
        })
    }

    fn update(&mut self, dt: f64) {
        self.velocity += self.force * dt / self.mass;
        self.transform.position += self.velocity * dt;
    }
}

pub struct PhysicsWorld<const N: usize> {
    objects: Vec<Object<N>>,
    solvers: Vec<Box<dyn Solver<N>>>,
    gravity: Vector<N>,
}

impl<const N: usize> PhysicsWorld<N> {
    pub fn new(gravity: Vector<N>) -> Result<PhysicsWorld<N>, UnitError> {
        gravity.is_of_unit(units::N)?;

        Ok(PhysicsWorld {
            objects: Vec::new(),
            solvers: Vec::new(),
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

    pub fn add_solver(&mut self, solver: Box<dyn Solver<N>>) -> &mut Self {
        self.solvers.push(solver);
        self
    }

    pub fn remove_solvers<F>(&mut self, f: F)
    where
        F: FnMut(&Box<dyn Solver<N>>) -> bool,
    {
        self.solvers.retain(f);
    }

    pub fn step(&mut self, dt: f64) {
        self.resolve_collisions(dt);
        for object in self.objects.iter_mut() {
            object.force += self.gravity * object.mass;
            object.update(dt);
            object.force = Vector::<N>::zero();
        }
    }

    fn resolve_collisions(&self, dt: f64) {
        let mut collisions = Vec::new();

        for obj_a in self.objects.iter() {
            for obj_b in self.objects.iter() {
                if std::ptr::eq(obj_a as _, obj_b as _) {
                    break;
                }

                let collision_points = obj_a.collider.test_collision(
                    &obj_a.transform,
                    &obj_b.collider,
                    &obj_b.transform,
                );

                if collision_points.has_collision {
                    collisions.push(Collision::new(
                        obj_a.clone(),
                        obj_b.clone(),
                        collision_points,
                    ));
                }
            }
        }

        for solver in self.solvers.iter() {
            solver.solve(&collisions, dt);
        }
    }
}

impl<const N: usize> Default for PhysicsWorld<N> {
    fn default() -> Self {
        Self::new(Vector::<N>::zero() * units::N).unwrap()
    }
}
