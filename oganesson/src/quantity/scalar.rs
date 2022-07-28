use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

use crate::{
    unit::{Unit, UnitError},
    units::Null,
};

#[derive(Clone, Copy, PartialEq)]
pub struct Scalar(pub f64, pub Unit);
impl Scalar {
    pub fn value(&self) -> f64 {
        self.0
    }

    pub fn zero() -> Scalar {
        Scalar(0.0, Null)
    }

    pub fn checked_add(self, other: Scalar) -> Option<Scalar> {
        if self.1 != other.1 {
            None
        } else {
            Some(self + other)
        }
    }

    pub fn checked_sub(self, other: Scalar) -> Option<Scalar> {
        if self.1 != other.1 {
            None
        } else {
            Some(self - other)
        }
    }

    pub fn is_of_unit(&self, unit: Unit) -> Result<(), UnitError> {
        if self.1 != unit {
            Err(UnitError::expected_unit(unit, self.1))
        } else {
            Ok(())
        }
    }

    pub fn unit(&self) -> Unit {
        self.1
    }
}

impl Default for Scalar {
    fn default() -> Self {
        Self::zero()
    }
}

impl Debug for Scalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1,)
    }
}

impl From<f64> for Scalar {
    fn from(a: f64) -> Self {
        Scalar(a, Null)
    }
}

impl Add for Scalar {
    type Output = Scalar;
    fn add(self, other: Scalar) -> Scalar {
        if self.1 != other.1 {
            panic!(
                "Cannot add vectors with different units: {} and {}",
                self.1, other.1
            );
        }
        Scalar(self.0 + other.0, self.1)
    }
}

impl AddAssign for Scalar {
    fn add_assign(&mut self, other: Scalar) {
        *self = *self + other;
    }
}

impl Sub for Scalar {
    type Output = Scalar;
    fn sub(self, other: Scalar) -> Scalar {
        if self.1 != other.1 {
            panic!(
                "Cannot add vectors with different units: {} and {}",
                self.1, other.1
            );
        }
        Scalar(self.0 - other.0, self.1)
    }
}

impl SubAssign for Scalar {
    fn sub_assign(&mut self, other: Scalar) {
        *self = *self - other;
    }
}

impl Mul<f64> for Scalar {
    type Output = Scalar;
    fn mul(self, other: f64) -> Scalar {
        Scalar(self.0 * other, self.1)
    }
}

impl Mul<Scalar> for f64 {
    type Output = Scalar;
    fn mul(self, other: Scalar) -> Scalar {
        other * self
    }
}

impl Div<f64> for Scalar {
    type Output = Scalar;
    fn div(self, other: f64) -> Scalar {
        Scalar(self.0 / other, self.1)
    }
}

impl Div<Scalar> for f64 {
    type Output = Scalar;
    fn div(self, other: Scalar) -> Scalar {
        Scalar(self / other.0, Null.div(other.1))
    }
}

impl Mul for Scalar {
    type Output = Scalar;
    fn mul(self, other: Scalar) -> Scalar {
        Scalar(self.0 * other.0, self.1 * other.1)
    }
}

impl Div for Scalar {
    type Output = Scalar;
    fn div(self, other: Scalar) -> Scalar {
        Scalar(self.0 / other.0, self.1 / other.1)
    }
}

impl Neg for Scalar {
    type Output = Scalar;
    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl Mul<Unit> for Scalar {
    type Output = Scalar;
    fn mul(self, rhs: Unit) -> Self::Output {
        Scalar(self.0, self.1 * rhs)
    }
}
