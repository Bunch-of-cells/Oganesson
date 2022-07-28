use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub, SubAssign},
};

use crate::{
    unit::{Unit, UnitError},
    units::Null,
    Scalar,
};

#[derive(Clone, Copy, PartialEq)]
pub struct Vector<const T: usize>(pub [f64; T], pub Unit);
impl<const T: usize> Vector<T> {
    pub fn magnitude(&self) -> f64 {
        self.0.iter().fold(0.0, |acc, &x| acc + x.powi(2)).sqrt()
    }

    pub fn normalized(&self) -> Vector<T> {
        let magnitude = self.magnitude();
        *self / magnitude
    }

    pub fn zero() -> Vector<T> {
        Vector([0.0; T], Null)
    }

    pub fn dot(&self, other: &Vector<T>) -> f64 {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(0.0, |acc, (&x, &y)| acc + x + y)
    }

    pub fn checked_add(self, other: Vector<T>) -> Option<Vector<T>> {
        if self.1 != other.1 {
            None
        } else {
            Some(self + other)
        }
    }

    pub fn checked_sub(self, other: Vector<T>) -> Option<Vector<T>> {
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

impl Vector<3> {
    pub fn cross(&self, other: &Vector<3>) -> Vector<3> {
        Vector(
            [
                self.0[1] * other.0[2] - self.0[2] * other.0[1],
                self.0[2] * other.0[0] - self.0[0] * other.0[2],
                self.0[0] * other.0[1] - self.0[1] * other.0[0],
            ],
            self.1 * other.1,
        )
    }
}

impl<const T: usize> Default for Vector<T> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<const T: usize> Debug for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.0
                .iter()
                .fold(String::new(), |acc, &x| acc + &format!("{:.2} ", x))
                .trim(),
            self.1,
        )
    }
}

impl<const T: usize> From<[f64; T]> for Vector<T> {
    fn from(a: [f64; T]) -> Self {
        Vector(a, Null)
    }
}

impl<const T: usize> Add for Vector<T> {
    type Output = Vector<T>;
    fn add(self, other: Vector<T>) -> Vector<T> {
        if self.1 != other.1 {
            panic!(
                "Cannot add vectors with different units: {} and {}",
                self.1, other.1
            );
        }
        let mut result = [0.0; T];
        self.0
            .iter()
            .zip(other.0.iter())
            .map(|(&x, &y)| x + y)
            .zip(result.iter_mut())
            .for_each(|(new, curr)| *curr = new);
        Vector(result, self.1)
    }
}

impl<const T: usize> AddAssign for Vector<T> {
    fn add_assign(&mut self, other: Vector<T>) {
        *self = *self + other;
    }
}

impl<const T: usize> Sub for Vector<T> {
    type Output = Vector<T>;
    fn sub(self, other: Vector<T>) -> Vector<T> {
        if self.1 != other.1 {
            panic!(
                "Cannot add vectors with different units: {} and {}",
                self.1, other.1
            );
        }
        let mut result = [0.0; T];
        self.0
            .iter()
            .zip(other.0.iter())
            .map(|(&x, &y)| x - y)
            .zip(result.iter_mut())
            .for_each(|(new, curr)| *curr = new);
        Vector(result, self.1)
    }
}

impl<const T: usize> SubAssign for Vector<T> {
    fn sub_assign(&mut self, other: Vector<T>) {
        *self = *self - other;
    }
}

impl<const T: usize> Mul<f64> for Vector<T> {
    type Output = Vector<T>;
    fn mul(self, other: f64) -> Vector<T> {
        let mut result = [0.0; T];
        self.0
            .iter()
            .map(|&x| x * other)
            .zip(result.iter_mut())
            .for_each(|(new, curr)| *curr = new);
        Vector(result, self.1)
    }
}

impl<const T: usize> Mul<Vector<T>> for f64 {
    type Output = Vector<T>;
    fn mul(self, other: Vector<T>) -> Vector<T> {
        other * self
    }
}

impl<const T: usize> Div<f64> for Vector<T> {
    type Output = Vector<T>;
    fn div(self, other: f64) -> Vector<T> {
        let mut result = [0.0; T];
        self.0
            .iter()
            .map(|&x| x / other)
            .zip(result.iter_mut())
            .for_each(|(new, curr)| *curr = new);
        Vector(result, self.1)
    }
}

impl<const T: usize> Mul<Scalar> for Vector<T> {
    type Output = Vector<T>;
    fn mul(self, other: Scalar) -> Vector<T> {
        let mut result = [0.0; T];
        self.0
            .iter()
            .map(|&x| x * other)
            .zip(result.iter_mut())
            .for_each(|(new, curr)| *curr = new.value());
        Vector(result, self.1 * other.1)
    }
}

impl<const T: usize> Mul<Vector<T>> for Scalar {
    type Output = Vector<T>;
    fn mul(self, other: Vector<T>) -> Vector<T> {
        other * self
    }
}

impl<const T: usize> Div<Scalar> for Vector<T> {
    type Output = Vector<T>;
    fn div(self, other: Scalar) -> Vector<T> {
        let mut result = [0.0; T];
        self.0
            .iter()
            .map(|&x| x / other)
            .zip(result.iter_mut())
            .for_each(|(new, curr)| *curr = new.value());
        Vector(result, self.1 / other.1)
    }
}

impl<const T: usize> Neg for Vector<T> {
    type Output = Vector<T>;
    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl<const T: usize> Index<usize> for Vector<T> {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const T: usize> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const T: usize> Mul<Unit> for Vector<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: Unit) -> Self::Output {
        Vector(self.0, self.1 * rhs)
    }
}
