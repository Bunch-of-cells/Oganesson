use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub, SubAssign},
    rc::Rc,
};

use crate::{
    dimension::{Dimension, DimensionError},
    Float, Scalar,
};

#[derive(Clone, PartialEq)]
pub struct Tensor {
    rank: usize,
    dim: u32,
    arr: Rc<Vec<Float>>,
    dim_: Dimension,
    start: usize,
}

impl Tensor {
    pub const fn zero(dim: u32, rank: usize) -> Tensor {
        Tensor {
            rank,
            dim,
            arr: Rc::new(Vec::from_iter((0..rank.pow(dim)).map(|_| 0.0))),
            dim_: Dimension::NONE,
            start: 0,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.arr().iter().all(|&x| x.abs() <= Float::EPSILON)
    }

    fn check_struct(&self, other: &Tensor) -> bool {
        self.rank == other.rank && self.dim == other.dim
    }

    pub fn checked_add(self, other: Tensor) -> Option<Tensor> {
        if self.dim_ != other.dim_ || self.check_struct(&other) {
            None
        } else {
            let arr = Rc::new(
                self.arr()
                    .iter()
                    .zip(other.arr().iter())
                    .map(|(&x, &y)| x + y)
                    .collect(),
            );
            Some(Tensor { arr, ..self })
        }
    }

    fn arr(&self) -> &[Float] {
        &self.arr[self.start..self.start + self.rank.pow(self.dim)]
    }

    pub fn checked_sub(self, other: Tensor) -> Option<Tensor> {
        if self.dim_ != other.dim_ || self.check_struct(&other) {
            None
        } else {
            let arr = Rc::new(
                self.arr()
                    .iter()
                    .zip(other.arr().iter())
                    .map(|(&x, &y)| x - y)
                    .collect(),
            );
            Some(Tensor { arr, ..self })
        }
    }

    pub fn dimension_err(
        &self,
        dim: impl Into<Dimension>,
        var: &str,
    ) -> Result<(), DimensionError> {
        let dim = dim.into();
        if self.dim_ != dim {
            Err(DimensionError::expected_dimension_of(dim, self.dim_, var))
        } else {
            Ok(())
        }
    }

    pub const fn dim(&self) -> Dimension {
        self.dim_
    }

    pub fn rankdim(&self) -> (usize, u32) {
        (self.rank, self.dim)
    }
}

// impl Debug for Tensor {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut iter = self.0.iter();
//         if let Some(dim) = iter.next() {
//             write!(f, "({}", dim)?;
//         }
//         for dim in iter {
//             write!(f, ", {}", dim)?;
//         }
//         write!(f, ") {}", self.1,)
//     }
// }

impl Add for Tensor {
    type Output = Tensor;
    #[track_caller]
    fn add(self, other: Tensor) -> Tensor {
        match self.checked_add(other) {
            Some(v) => v,
            None => panic!(
                "Cannot add tensors objects: [{}, {}] ({}) and [{}, {}] ({})",
                self.rank, self.dim, self.dim_, other.rank, other.dim, other.dim_
            ),
        }
    }
}

impl AddAssign for Tensor {
    #[track_caller]
    fn add_assign(&mut self, other: Tensor) {
        *self = *self + other;
    }
}

impl Sub for Tensor {
    type Output = Tensor;
    #[track_caller]
    fn sub(self, other: Tensor) -> Tensor {
        match self.checked_sub(other) {
            Some(v) => v,
            None => panic!(
                "Cannot subtract tensors objects: [{}, {}] ({}) and [{}, {}] ({})",
                self.rank, self.dim, self.dim_, other.rank, other.dim, other.dim_
            ),
        }
    }
}

impl SubAssign for Tensor {
    #[track_caller]
    fn sub_assign(&mut self, other: Tensor) {
        *self = *self - other;
    }
}

impl Mul<Float> for Tensor {
    type Output = Tensor;
    fn mul(self, other: Float) -> Tensor {
        let arr = Rc::new(self.arr().iter().map(|&x| x * other).collect());
        Tensor { arr, ..self }
    }
}

impl Mul<Tensor> for Float {
    type Output = Tensor;
    fn mul(self, other: Tensor) -> Tensor {
        other * self
    }
}

impl Div<Float> for Tensor {
    type Output = Tensor;
    fn div(self, other: Float) -> Tensor {
        let arr = Rc::new(self.arr().iter().map(|&x| x / other).collect());
        Tensor { arr, ..self }
    }
}

impl Mul<Scalar> for Tensor {
    type Output = Tensor;
    fn mul(self, other: Scalar) -> Tensor {
        let arr = Rc::new(self.arr().iter().map(|&x| x * other.0).collect());
        Tensor {
            arr,
            dim_: self.dim_ * other.dim(),
            ..self
        }
    }
}

impl Mul<Tensor> for Scalar {
    type Output = Tensor;
    fn mul(self, other: Tensor) -> Tensor {
        other * self
    }
}

impl Div<Scalar> for Tensor {
    type Output = Tensor;
    fn div(self, other: Scalar) -> Tensor {
        let arr = Rc::new(self.arr().iter().map(|&x| x / other.0).collect());
        Tensor {
            arr,
            dim_: self.dim_ / other.dim(),
            ..self
        }
    }
}

impl Neg for Tensor {
    type Output = Tensor;
    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl Index<usize> for Tensor {
    type Output = Tensor;
    fn index(&self, index: usize) -> &Self::Output {
        if index > self.rank || self.dim == 0 {
            panic!()
        }
        let arr = Rc::clone(&self.arr);
        &Tensor {
            arr,
            dim: self.dim - 1,
            rank: self.rank,
            dim_: self.dim_,
            start: self.start + self.rank * index,
        }
    }
}

impl IndexMut<usize> for Tensor {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index > self.rank || self.dim == 0 {
            panic!()
        }
        let arr = Rc::clone(&self.arr);
        &mut Tensor {
            arr,
            dim: self.dim - 1,
            rank: self.rank,
            dim_: self.dim_,
            start: self.start + self.rank * index,
        }
    }
}

impl Mul<Dimension> for Tensor {
    type Output = Tensor;
    fn mul(self, rhs: Dimension) -> Self::Output {
        Tensor {
            dim_: self.dim_ * rhs,
            ..self
        }
    }
}

impl Div<Dimension> for Tensor {
    type Output = Tensor;
    fn div(self, rhs: Dimension) -> Self::Output {
        Tensor {
            dim_: self.dim_ * rhs,
            ..self
        }
    }
}

impl From<Tensor> for Dimension {
    fn from(val: Tensor) -> Dimension {
        val.dim_
    }
}
