use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub, SubAssign},
};

use ggez::mint::{Point2, Point3};

use crate::{
    unit::{Unit, UnitError},
    units::Null,
    Float, Scalar,
};

#[derive(Clone, Copy, PartialEq)]
pub struct Vector<const N: usize>(pub [Float; N], pub Unit);

impl<const N: usize> Vector<N> {
    pub fn magnitude(&self) -> Scalar {
        Scalar(
            self.0.iter().fold(0.0, |acc, &x| acc + x.powi(2)).sqrt(),
            self.1,
        )
    }

    pub fn normalized(&self) -> Vector<N> {
        let magnitude = self.magnitude();
        *self / magnitude
    }

    pub const fn zero() -> Vector<N> {
        Vector([0.0; N], Null)
    }

    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&x| x.abs() <= Float::EPSILON)
    }

    pub fn dot(&self, other: &Vector<N>) -> Scalar {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(Scalar(0.0, self.1 * other.1), |acc, (&x1, &x2)| acc + x1 * x2)
    }

    pub fn checked_add(self, other: Vector<N>) -> Option<Vector<N>> {
        if self.1 != other.1 {
            None
        } else {
            let mut result = [0.0; N];
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(&x, &y)| x + y)
                .zip(result.iter_mut())
                .for_each(|(new, curr)| *curr = new);
            Some(Vector(result, self.1))
        }
    }

    pub fn checked_sub(self, other: Vector<N>) -> Option<Vector<N>> {
        if self.1 != other.1 {
            None
        } else {
            let mut result = [0.0; N];
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(&x, &y)| x - y)
                .zip(result.iter_mut())
                .for_each(|(new, curr)| *curr = new);

            Some(Vector(result, self.1))
        }
    }

    pub fn get_uniterror(&self, unit: Unit, var: &str) -> Result<(), UnitError> {
        if self.1 != unit {
            Err(UnitError::expected_unit_of(unit, self.1, var))
        } else {
            Ok(())
        }
    }

    pub const fn unit(&self) -> Unit {
        self.1
    }

    pub fn squared(self) -> Scalar {
        self.dot(&self)
    }

    pub const fn as_slice(&self) -> &[Float] {
        &self.0
    }

    #[track_caller]
    pub const fn unit_vector(direction: usize) -> Vector<N> {
        if direction > N {
            panic!("Vector::unit_vector: direction out of bounds");
        }
        let mut a = [0.0; N];
        a[direction] = 1.0;
        Vector(a, Null)
    }

    pub fn truncated<const M: usize>(&self) -> Vector<M> {
        assert!(M < N, "Vector::truncate: Cannot truncate a {}-dimentional vector into a {}-dimentional vector", N, M);
        Vector(unsafe { std::mem::transmute_copy(&self.0) }, self.1)
    }

    pub fn project(self, on: &Vector<N>) -> Self {
        self.dot(on) / on.magnitude() * on.normalized()
    }

    pub fn angle_to(&self, other: &Vector<N>) -> Float {
        (self.dot(other) / (self.magnitude() * other.magnitude())).acos()
    }
}

impl Vector<2> {
    #[track_caller]
    /// (r, φ)
    pub fn polar_coords(&self) -> (Scalar, Float) {
        let [x, y] = self.0;
        let r = self.magnitude();
        let φ = if r.abs() <= Float::EPSILON {
            todo!()
        } else if y.is_sign_negative() {
            -(x / r).acos()
        } else {
            (x / r).acos()
        };
        (r, φ)
    }

    pub fn from_spherical_coords(r: Scalar, θ: Float) -> Self {
        Vector([r.value() * θ.cos(), r.value() * θ.sin()], r.unit())
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

    #[track_caller]
    /// (r, θ, φ)
    pub fn spherical_coords(&self) -> (Scalar, Float, Float) {
        let [x, y, z] = self.0;
        let r = self.magnitude();
        let θ = (z / r).acos();
        let r_xy = x.hypot(y);
        let φ = if r_xy.abs() <= Float::EPSILON {
            todo!()
        } else if y.is_sign_negative() {
            -(x / r_xy).acos()
        } else {
            (x / r_xy).acos()
        };
        (r, θ, φ)
    }

    pub fn from_spherical_coords(r: Scalar, θ: Float, φ: Float) -> Self {
        Vector(
            [
                r.value() * θ.sin() * φ.cos(),
                r.value() * θ.sin() * φ.sin(),
                r.value() * θ.cos(),
            ],
            r.unit(),
        )
    }

    #[track_caller]
    /// (ρ, φ, z)
    pub fn cylindrical_coords(&self) -> (Scalar, Float, Float) {
        let (r, θ, φ) = self.spherical_coords();
        (r * θ.sin(), φ, r.value() * θ.cos())
    }

    #[track_caller]
    pub fn from_cylindrical_coords(ρ: Scalar, φ: Float, z: Float) -> Self {
        let r = (ρ * ρ + z * z).sqrt();
        let θ = (z / r).atan();
        Self::from_spherical_coords(r * ρ.unit(), θ, φ)
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
                .fold(String::new(), |acc, &x| acc + &format!("{:.2?} ", x))
                .trim(),
            self.1,
        )
    }
}

impl<const T: usize> From<[Float; T]> for Vector<T> {
    fn from(a: [Float; T]) -> Self {
        Vector(a, Null)
    }
}

impl<const T: usize> Add for Vector<T> {
    type Output = Vector<T>;
    #[track_caller]
    fn add(self, other: Vector<T>) -> Vector<T> {
        match self.checked_add(other) {
            Some(v) => v,
            None => panic!(
                "Cannot add vectors with different units: {} and {}",
                self.1, other.1
            ),
        }
    }
}

impl<const T: usize> AddAssign for Vector<T> {
    #[track_caller]
    fn add_assign(&mut self, other: Vector<T>) {
        *self = *self + other;
    }
}

impl<const T: usize> Sub for Vector<T> {
    type Output = Vector<T>;
    #[track_caller]
    fn sub(self, other: Vector<T>) -> Vector<T> {
        match self.checked_sub(other) {
            Some(v) => v,
            None => panic!(
                "Cannot subtract vectors with different units: {} and {}",
                self.1, other.1
            ),
        }
    }
}

impl<const T: usize> SubAssign for Vector<T> {
    #[track_caller]
    fn sub_assign(&mut self, other: Vector<T>) {
        *self = *self - other;
    }
}

impl<const T: usize> Mul<Float> for Vector<T> {
    type Output = Vector<T>;
    fn mul(self, other: Float) -> Vector<T> {
        let mut result = [0.0; T];
        self.0
            .iter()
            .map(|&x| x * other)
            .zip(result.iter_mut())
            .for_each(|(new, curr)| *curr = new);
        Vector(result, self.1)
    }
}

impl<const T: usize> Mul<Vector<T>> for Float {
    type Output = Vector<T>;
    fn mul(self, other: Vector<T>) -> Vector<T> {
        other * self
    }
}

impl<const T: usize> Div<Float> for Vector<T> {
    type Output = Vector<T>;
    fn div(self, other: Float) -> Vector<T> {
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
    type Output = Float;
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

impl<const T: usize> Div<Unit> for Vector<T> {
    type Output = Vector<T>;
    fn div(self, rhs: Unit) -> Self::Output {
        Vector(self.0, self.1 / rhs)
    }
}

#[cfg(feature = "simulation")]
impl From<Vector<2>> for Point2<Float> {
    fn from(vector: Vector<2>) -> Self {
        vector.0.into()
    }
}

#[cfg(feature = "simulation")]
impl From<Vector<3>> for Point3<Float> {
    fn from(vector: Vector<3>) -> Self {
        vector.0.into()
    }
}
