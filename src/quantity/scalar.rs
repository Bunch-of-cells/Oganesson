use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, Neg, Sub, SubAssign},
};

use crate::{
    unit::{Unit, UnitError},
    units::Null,
    Float,
};

#[derive(Clone, Copy, PartialEq)]
pub struct Scalar(pub Float, pub Unit);

impl Scalar {
    pub fn value(&self) -> Float {
        self.0
    }

    pub fn zero() -> Scalar {
        Scalar(0.0, Null)
    }

    pub fn checked_add(self, other: Scalar) -> Option<Scalar> {
        if self.1 != other.1 {
            None
        } else {
            Some(Scalar(self.0 + other.0, self.1))
        }
    }

    pub fn checked_sub(self, other: Scalar) -> Option<Scalar> {
        if self.1 != other.1 {
            None
        } else {
            Some(Scalar(self.0 - other.0, self.1))
        }
    }

    pub fn get_uniterror(&self, unit: Unit, var: &str) -> Result<(), UnitError> {
        if self.1 != unit {
            Err(UnitError::expected_unit_of(unit, self.1, var))
        } else {
            Ok(())
        }
    }

    pub fn unit(&self) -> Unit {
        self.1
    }

    /// **This does not raise the units to the given power, use it at your own risk**
    pub fn powf(self, n: Float) -> Scalar {
        Scalar(self.0.powf(n), self.1)
    }

    pub fn powi(self, n: i32) -> Scalar {
        Scalar(self.0.powi(n), self.1.pow(n))
    }

    pub fn try_radical(self, n: i32) -> Option<Scalar> {
        Some(Scalar(
            self.0.powf(1.0 / n as Float),
            self.1.try_radical(n)?,
        ))
    }

    pub fn abs(self) -> Scalar {
        Scalar(self.0.abs(), self.1)
    }

    pub fn squared(self) -> Scalar {
        Scalar(self.0.powi(2), self.1.pow(2))
    }
}

impl Default for Scalar {
    fn default() -> Self {
        Self::zero()
    }
}

impl Debug for Scalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2?} {}", self.0, self.1,)
    }
}

impl From<Float> for Scalar {
    fn from(a: Float) -> Self {
        Scalar(a, Null)
    }
}

impl Add for Scalar {
    type Output = Scalar;
    #[track_caller]
    fn add(self, other: Scalar) -> Scalar {
        self.checked_add(other).unwrap_or_else(|| {
            panic!(
                "Cannot add scalars with different units: {} and {}",
                self.1, other.1
            )
        })
    }
}

impl AddAssign for Scalar {
    #[track_caller]
    fn add_assign(&mut self, other: Scalar) {
        *self = *self + other;
    }
}

impl Add<Float> for Scalar {
    type Output = Scalar;
    fn add(self, other: Float) -> Scalar {
        Scalar(self.0 + other, self.1)
    }
}

impl AddAssign<Float> for Scalar {
    fn add_assign(&mut self, other: Float) {
        *self = *self + other;
    }
}

impl Add<Scalar> for Float {
    type Output = Scalar;
    fn add(self, other: Scalar) -> Scalar {
        other + self
    }
}

impl Sub for Scalar {
    type Output = Scalar;
    #[track_caller]
    fn sub(self, other: Scalar) -> Scalar {
        self.checked_add(other).unwrap_or_else(|| {
            panic!(
                "Cannot subtract scalars with different units: {} and {}",
                self.1, other.1
            )
        })
    }
}

impl SubAssign for Scalar {
    #[track_caller]
    fn sub_assign(&mut self, other: Scalar) {
        *self = *self - other;
    }
}

impl Sub<Float> for Scalar {
    type Output = Scalar;
    fn sub(self, other: Float) -> Scalar {
        Scalar(self.0 - other, self.1)
    }
}

impl SubAssign<Float> for Scalar {
    fn sub_assign(&mut self, other: Float) {
        *self = *self - other;
    }
}

impl Sub<Scalar> for Float {
    type Output = Scalar;
    fn sub(self, other: Scalar) -> Scalar {
        Scalar(self - other.0, other.1)
    }
}

impl Mul<Float> for Scalar {
    type Output = Scalar;
    fn mul(self, other: Float) -> Scalar {
        Scalar(self.0 * other, self.1)
    }
}

impl Mul<Scalar> for Float {
    type Output = Scalar;
    fn mul(self, other: Scalar) -> Scalar {
        other * self
    }
}

impl Div<Float> for Scalar {
    type Output = Scalar;
    fn div(self, other: Float) -> Scalar {
        Scalar(self.0 / other, self.1)
    }
}

impl Div<Scalar> for Float {
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

impl Div<Unit> for Scalar {
    type Output = Scalar;
    fn div(self, rhs: Unit) -> Self::Output {
        Scalar(self.0, self.1 / rhs)
    }
}

impl PartialOrd for Scalar {
    #[track_caller]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.1 != other.1 {
            panic!(
                "Cannot compare scalars with different units: {} and {}",
                self.1, other.1
            );
        }
        self.0.partial_cmp(&other.0)
    }
}

impl PartialEq<Float> for Scalar {
    fn eq(&self, other: &Float) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Scalar> for Float {
    fn eq(&self, other: &Scalar) -> bool {
        *self == other.0
    }
}

impl PartialOrd<Float> for Scalar {
    fn partial_cmp(&self, other: &Float) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<Scalar> for Float {
    fn partial_cmp(&self, other: &Scalar) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&other.0)
    }
}

impl Deref for Scalar {
    type Target = Float;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Scalar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
