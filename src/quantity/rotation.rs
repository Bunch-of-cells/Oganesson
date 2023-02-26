use std::ops::Mul;

use crate::{Float, Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    pub w: Float,
    pub v: Vector<3>,
}

impl Quaternion {
    pub fn new(θ: Float, v: Vector<3>) -> Quaternion {
        Quaternion {
            w: (θ / 2.0).cos(),
            v: (θ / 2.0).sin() * v.normalized(),
        }
    }

    pub fn inverse(&self) -> Quaternion {
        Quaternion {
            w: self.w,
            v: -self.v,
        }
    }
}

impl Mul for Quaternion {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Quaternion {
            w: self.w * rhs.w,
            v: self.v * rhs.w + rhs.v * self.w + self.v.cross(&rhs.v),
        }
    }
}
