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
    pub fn new(
        position: Vector<N>,
        shape: ObjectShape<N>,
        rotation: Rotation,
        collider: bool,
    ) -> Transform<N> {
        Transform {
            position,
            rotation,
            size: 1.0.into(),
            collider: collider
                .then(|| shape.clone().into_collider())
                .unwrap_or_default(),
            shape,
        }
    }

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
    pub fn new_2d() -> Rotation {
        Rotation::Dim2(0.0)
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
