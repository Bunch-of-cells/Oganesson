use crate::{Scalar, Vector};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone)]
pub struct Transform<const N: usize> {
    pub(crate) position: Vector<N>,
    pub(crate) scale: Vector<N>,
    pub(crate) rotation: Quaternion,
}

impl<const N: usize> Transform<N> {
    pub fn position(&self) -> Vector<N> {
        self.position
    }

    pub fn scale(&self) -> Vector<N> {
        self.scale
    }

    pub fn rotation(&self) -> Quaternion {
        self.rotation
    }
}

#[derive(Debug, Clone)]
pub enum Collider<const N: usize> {
    Sphere { radius: Scalar },
    Plane { dimentions: Vector<N> },
}

impl<const N: usize> Collider<N> {
    pub fn is_collision(
        &self,
        transform: &Transform<N>,
        collider: &Collider<N>,
        collider_transform: &Transform<N>,
    ) -> bool {
        match (self, collider) {
            (&Self::Sphere { radius: r1 }, &Self::Sphere { radius: r2 }) => {
                let displacement = transform.position - collider_transform.position;
                let distance = displacement.magnitude().abs();
                if distance >= r1 + r2 {
                    false
                } else {
                    true
                }
            }
            (&Self::Sphere { radius: _r }, &Self::Plane { dimentions: _d }) => {
                
                todo!()
            }
            (Self::Plane { .. }, Self::Sphere { .. }) => {
                collider.is_collision(collider_transform, self, transform)
            }
            (Self::Plane { .. }, Self::Plane { .. }) => {
                todo!()
            }
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct Collision<const N: usize> {
    pub a: usize,
    pub b: usize,
    pub A: Vector<N>,
    pub B: Vector<N>,
}

impl<const N: usize> Collision<N> {
    #[allow(non_snake_case)]
    pub fn new(a: usize, b: usize, A: Vector<N>, B: Vector<N>) -> Collision<N> {
        Collision { a, b, A, B }
    }
}
