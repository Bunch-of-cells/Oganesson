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

    /// **This does not raise the units to the given power, use it at your own risk**
    pub fn powf(self, n: f64) -> Scalar {
        Scalar(self.0.powf(n), self.1)
    }

    pub fn powi(self, n: i32) -> Scalar {
        Scalar(self.0.powi(n), self.1.pow(n))
    }

    pub fn try_radical(self, n: i32) -> Option<Scalar> {
        Some(Scalar(self.0.powf(1.0 / n as f64), self.1.try_radical(n)?))
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
    #[track_caller]
    fn add(self, other: Scalar) -> Scalar {
        if self.1 != other.1 {
            panic!(
                "Cannot add scalars with different units: {} and {}",
                self.1, other.1
            );
        }
        Scalar(self.0 + other.0, self.1)
    }
}

impl AddAssign for Scalar {
    #[track_caller]
    fn add_assign(&mut self, other: Scalar) {
        *self = *self + other;
    }
}

impl Add<f64> for Scalar {
    type Output = Scalar;
    fn add(self, other: f64) -> Scalar {
        Scalar(self.0 + other, self.1)
    }
}

impl AddAssign<f64> for Scalar {
    fn add_assign(&mut self, other: f64) {
        *self = *self + other;
    }
}

impl Add<Scalar> for f64 {
    type Output = Scalar;
    fn add(self, other: Scalar) -> Scalar {
        other + self
    }
}

impl Sub for Scalar {
    type Output = Scalar;
    #[track_caller]
    fn sub(self, other: Scalar) -> Scalar {
        if self.1 != other.1 {
            panic!(
                "Cannot add scalars with different units: {} and {}",
                self.1, other.1
            );
        }
        Scalar(self.0 - other.0, self.1)
    }
}

impl SubAssign for Scalar {
    #[track_caller]
    fn sub_assign(&mut self, other: Scalar) {
        *self = *self - other;
    }
}

impl Sub<f64> for Scalar {
    type Output = Scalar;
    fn sub(self, other: f64) -> Scalar {
        Scalar(self.0 - other, self.1)
    }
}

impl SubAssign<f64> for Scalar {
    fn sub_assign(&mut self, other: f64) {
        *self = *self - other;
    }
}

impl Sub<Scalar> for f64 {
    type Output = Scalar;
    fn sub(self, other: Scalar) -> Scalar {
        Scalar(self - other.0, other.1)
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
