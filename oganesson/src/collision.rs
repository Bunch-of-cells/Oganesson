use crate::{unit::UnitError, units::Null, Object, Vector};

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct CollisionPoints<const N: usize> {
    pub A: Vector<N>,
    pub B: Vector<N>,
    pub normal: Vector<N>,
    pub depth: f32,
    pub has_collision: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Quaternion {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}

#[derive(Debug, Clone)]
pub struct Transform<const N: usize> {
    pub position: Vector<N>,
    pub scale: Vector<N>,
    pub rotation: Quaternion,
}

impl<const N: usize> Transform<N> {
    pub fn new(
        position: Vector<N>,
        scale: Vector<N>,
        rotation: Quaternion,
    ) -> Result<Transform<N>, UnitError> {
        position.is_of_unit(Null)?;
        scale.is_of_unit(Null)?;
        Ok(Transform {
            position,
            scale,
            rotation,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Collider<const N: usize> {
    Sphere { radius: f32, center: Vector<N> },
    Plane { plane: Vector<N>, distance: f32 },
}

impl<const N: usize> Collider<N> {
    pub fn test_collision(
        &self,
        transform: &Transform<N>,
        collider: &Collider<N>,
        collider_transform: &Transform<N>,
    ) -> CollisionPoints<N> {
        match (self, collider) {
            (Self::Sphere { .. }, Self::Sphere { .. }) => {
                todo!()
            }
            (Self::Sphere { .. }, Self::Plane { .. }) => {
                todo!()
            }
            (Self::Plane { .. }, Self::Sphere { .. }) => {
                let mut points = collider.test_collision(collider_transform, self, transform);
                std::mem::swap(&mut points.A, &mut points.B);

                points.normal = -points.normal;
                points
            }
            (Self::Plane { .. }, Self::Plane { .. }) => {
                unreachable!()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Collision<const N: usize> {
    pub obj_a: Object<N>,
    pub obj_b: Object<N>,
    pub points: CollisionPoints<N>,
}

impl<const N: usize> Collision<N> {
    pub fn new(obj_a: Object<N>, obj_b: Object<N>, points: CollisionPoints<N>) -> Collision<N> {
        Collision {
            obj_a,
            obj_b,
            points,
        }
    }
}

pub trait Solver<const N: usize> {
    fn solve(&self, collisions: &[Collision<N>], dt: f64);
}
