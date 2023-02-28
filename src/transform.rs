use crate::{Collider, Float, Quaternion, Scalar, Vector};

#[derive(Debug, Clone)]
pub struct Transform<const N: usize> {
    pub(crate) position: Vector<N>,
    pub(crate) rotation: Rotation,
    pub(crate) shape: ObjectShape<N>,
    pub(crate) size: Scalar,
    pub(crate) collider: Collider<N>,
}

impl<const N: usize> Transform<N> {
    pub fn position(&self) -> Vector<N> {
        self.position
    }

    pub fn size(&self) -> Scalar {
        self.size
    }

    pub fn shape(&self) -> &ObjectShape<N> {
        &self.shape
    }

    pub fn rotation(&self) -> Rotation {
        self.rotation
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Rotation {
    Dim2(Float),
    Dim3(Quaternion),
}

impl Rotation {
    pub fn as_2d(&self) -> Option<Float> {
        match *self {
            Self::Dim2(θ) => Some(θ),
            _ => None,
        }
    }

    pub fn as_3d(&self) -> Option<Quaternion> {
        match *self {
            Self::Dim3(q) => Some(q),
            _ => None,
        }
    }

    pub fn rotate_vec<const N: usize>(self, v: Vector<N>) -> Vector<N> {
        match self {
            Rotation::Dim2(θ) if N == 2 => v.resize::<2>().rotate(θ).resize(),
            Rotation::Dim3(q) if N == 3 => v.resize::<3>().rotate(q).resize(),
            _ => panic!(),
        }
    }

    pub fn rotate(&mut self, other: Rotation) {
        match (self, other) {
            (Rotation::Dim2(θ), Rotation::Dim2(θ1)) => *θ += θ1,
            (Rotation::Dim3(q), Rotation::Dim3(q1)) => *q = *q * q1,
            _ => panic!(),
        }
    }

    pub fn new<const N: usize>() -> Rotation {
        match N {
            2 => Rotation::Dim2(0.0),
            3 => Rotation::Dim3(Quaternion::default()),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum ObjectShape<const N: usize> {
    Sphere {
        radius: Scalar,
    },
    Polygon {
        points: Vec<Vector<N>>,
    },
    #[default]
    Point,
}

impl<const N: usize> ObjectShape<N> {
    pub(crate) fn into_collider(self) -> Collider<N> {
        match self {
            ObjectShape::Sphere { radius } => Collider::Sphere { radius },
            ObjectShape::Point => Collider::Point,
            ObjectShape::Polygon { points } => Collider::Polygon { points },
        }
    }
}
