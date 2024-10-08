use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub, SubAssign},
};

use macroquad::prelude::{Vec2, Vec3};

use crate::{
    dimension::{Dimension, DimensionError},
    Float, Scalar,
};

#[derive(Clone, Copy, PartialEq)]
pub struct Vector<const N: usize>(pub [Float; N], pub Dimension);

impl<const N: usize> Vector<N> {
    pub fn magnitude(&self) -> Scalar {
        self.0.iter().fold(0.0, |acc, &x| acc + x.powi(2)).sqrt() * self.1
    }

    /// Returns a normalized dimensionless vector
    pub fn normalized(&self) -> Vector<N> {
        let magnitude = self.magnitude();
        *self / magnitude
    }

    pub const fn zero() -> Vector<N> {
        Vector([0.0; N], Dimension::NONE)
    }

    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&x| x.abs() <= Float::EPSILON)
    }

    pub fn dot(&self, other: Vector<N>) -> Scalar {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(0.0 * self.1 * other.1, |acc, (&x1, &x2)| acc + x1 * x2)
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

    pub fn dimension_err(
        &self,
        dim: impl Into<Dimension>,
        var: &str,
    ) -> Result<(), DimensionError> {
        let dim = dim.into();
        if self.1 != dim {
            Err(DimensionError::expected_dimension_of(dim, self.1, var))
        } else {
            Ok(())
        }
    }

    pub const fn dim(&self) -> Dimension {
        self.1
    }

    pub fn squared(self) -> Scalar {
        self.dot(self)
    }

    pub const fn as_slice(&self) -> &[Float] {
        &self.0
    }

    pub fn triple_product(self, b: Vector<N>, c: Vector<N>) -> Vector<N> {
        self.dot(c) * b - self.dot(b) * c
    }

    #[track_caller]
    pub const fn basis(direction: usize) -> Vector<N> {
        if direction > N {
            panic!("Vector::basis: direction out of bounds");
        }
        let mut a = [0.0; N];
        a[direction] = 1.0;
        Vector(a, Dimension::NONE)
    }

    pub fn resize<const M: usize>(&self) -> Vector<M> {
        if M < N {
            let mut new = [0.0; M];
            for (i, &x) in self.0.iter().enumerate() {
                new[i] = x;
            }
            Vector(new, self.1)
        } else {
            Vector(unsafe { std::mem::transmute_copy(&self.0) }, self.1)
        }
    }

    pub fn project(self, on: Vector<N>) -> Self {
        self.dot(on) / on.magnitude() * on.normalized()
    }

    pub fn angle_to(&self, other: Vector<N>) -> Float {
        (self.dot(other) / (self.magnitude() * other.magnitude())).acos()
    }

    #[track_caller]
    pub fn basis_const<const M: usize>() -> Vector<N> {
        assert!(M < N);
        let mut v = [0.0; N];
        v[M] = 1.0;
        v.into()
    }
}

impl Vector<2> {
    #[allow(non_upper_case_globals)]
    pub const i: Vector<2> = Vector([1.0, 0.0], Dimension::NONE);
    #[allow(non_upper_case_globals)]
    pub const j: Vector<2> = Vector([0.0, 1.0], Dimension::NONE);
    pub const ZERO: Vector<2> = Vector([0.0, 0.0], Dimension::NONE);

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

    pub fn from_polar_coords(r: Scalar, θ: Float) -> Self {
        [r.value() * θ.cos(), r.value() * θ.sin()] * r.dim()
    }

    pub fn perpendicular(&self, clockwise: bool) -> Self {
        if clockwise {
            Vector([-self.0[1], self.0[0]], self.1)
        } else {
            Vector([self.0[1], -self.0[0]], self.1)
        }
    }

    pub fn rotate(&self, θ: Scalar) -> Self {
        Vector(
            [
                self[0] * θ.cos() - self[1] * θ.sin(),
                self[1] * θ.cos() + self[0] * θ.sin(),
            ],
            self.1,
        )
    }
}

impl Vector<3> {
    #[allow(non_upper_case_globals)]
    pub const i: Vector<3> = Vector([1.0, 0.0, 0.0], Dimension::NONE);
    #[allow(non_upper_case_globals)]
    pub const j: Vector<3> = Vector([0.0, 1.0, 0.0], Dimension::NONE);
    #[allow(non_upper_case_globals)]
    pub const k: Vector<3> = Vector([0.0, 0.0, 1.0], Dimension::NONE);
    pub const ZERO: Vector<3> = Vector([0.0, 0.0, 0.0], Dimension::NONE);

    pub fn cross(&self, other: Vector<3>) -> Vector<3> {
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
        [
            r.value() * θ.sin() * φ.cos(),
            r.value() * θ.sin() * φ.sin(),
            r.value() * θ.cos(),
        ] * r.dim()
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
        Self::from_spherical_coords(r * ρ.dim(), θ, φ)
    }

    pub fn scalar_triple_product(self, b: Vector<3>, c: Vector<3>) -> Scalar {
        self.dot(b.cross(c))
    }
}

impl<const N: usize> Default for Vector<N> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<const N: usize> Debug for Vector<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.0.iter();
        if let Some(dim) = iter.next() {
            write!(f, "({}", dim)?;
        }
        for dim in iter {
            write!(f, ", {}", dim)?;
        }
        write!(f, ") {}", self.1,)
    }
}

impl<const N: usize> From<[Float; N]> for Vector<N> {
    fn from(a: [Float; N]) -> Self {
        Vector(a, Dimension::NONE)
    }
}

impl<const N: usize> From<[Scalar; N]> for Vector<N> {
    fn from(a: [Scalar; N]) -> Self {
        Vector(a.map(|s| s.0), a.first().map(|s| s.1).unwrap_or_default())
    }
}

impl<const N: usize> Add for Vector<N> {
    type Output = Vector<N>;
    #[track_caller]
    fn add(self, other: Vector<N>) -> Vector<N> {
        match self.checked_add(other) {
            Some(v) => v,
            None => panic!(
                "Cannot add vectors with different dimensions: {} and {}",
                self.1, other.1
            ),
        }
    }
}

impl<const N: usize> AddAssign for Vector<N> {
    #[track_caller]
    fn add_assign(&mut self, other: Vector<N>) {
        *self = *self + other;
    }
}

impl<const N: usize> Sub for Vector<N> {
    type Output = Vector<N>;
    #[track_caller]
    fn sub(self, other: Vector<N>) -> Vector<N> {
        match self.checked_sub(other) {
            Some(v) => v,
            None => panic!(
                "Cannot subtract vectors with different dimensions: {} and {}",
                self.1, other.1
            ),
        }
    }
}

impl<const N: usize> SubAssign for Vector<N> {
    #[track_caller]
    fn sub_assign(&mut self, other: Vector<N>) {
        *self = *self - other;
    }
}

impl<const N: usize> Mul<Float> for Vector<N> {
    type Output = Vector<N>;
    fn mul(self, other: Float) -> Vector<N> {
        let mut result = [0.0; N];
        self.0
            .iter()
            .map(|&x| x * other)
            .zip(result.iter_mut())
            .for_each(|(new, curr)| *curr = new);
        Vector(result, self.1)
    }
}

impl<const N: usize> Mul<Vector<N>> for Float {
    type Output = Vector<N>;
    fn mul(self, other: Vector<N>) -> Vector<N> {
        other * self
    }
}

impl<const N: usize> Div<Float> for Vector<N> {
    type Output = Vector<N>;
    fn div(self, other: Float) -> Vector<N> {
        let mut result = [0.0; N];
        self.0
            .iter()
            .map(|&x| x / other)
            .zip(result.iter_mut())
            .for_each(|(new, curr)| *curr = new);
        Vector(result, self.1)
    }
}

impl<const N: usize> Mul<Scalar> for Vector<N> {
    type Output = Vector<N>;
    fn mul(self, other: Scalar) -> Vector<N> {
        let mut result = [0.0; N];
        self.0
            .iter()
            .map(|&x| x * other)
            .zip(result.iter_mut())
            .for_each(|(new, curr)| *curr = new.value());
        Vector(result, self.1 * other.1)
    }
}

impl<const N: usize> Mul<Scalar> for [Float; N] {
    type Output = Vector<N>;
    fn mul(self, other: Scalar) -> Vector<N> {
        let mut result = [0.0; N];
        self.iter()
            .map(|&x| x * other)
            .zip(result.iter_mut())
            .for_each(|(new, curr)| *curr = new.value());
        Vector(result, other.1)
    }
}

impl<const N: usize> Mul<Vector<N>> for Scalar {
    type Output = Vector<N>;
    fn mul(self, other: Vector<N>) -> Vector<N> {
        other * self
    }
}

impl<const N: usize> Div<Scalar> for Vector<N> {
    type Output = Vector<N>;
    fn div(self, other: Scalar) -> Vector<N> {
        let mut result = [0.0; N];
        self.0
            .iter()
            .map(|&x| x / other)
            .zip(result.iter_mut())
            .for_each(|(new, curr)| *curr = new.value());
        Vector(result, self.1 / other.1)
    }
}

impl<const N: usize> Div<Scalar> for [Float; N] {
    type Output = Vector<N>;
    fn div(self, other: Scalar) -> Vector<N> {
        let mut result = [0.0; N];
        self.iter()
            .map(|&x| x / other)
            .zip(result.iter_mut())
            .for_each(|(new, curr)| *curr = new.value());
        Vector(result, other.1.inv())
    }
}

impl<const N: usize> Neg for Vector<N> {
    type Output = Vector<N>;
    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl<const N: usize> Index<usize> for Vector<N> {
    type Output = Float;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> IndexMut<usize> for Vector<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const N: usize> Mul<Dimension> for Vector<N> {
    type Output = Vector<N>;
    fn mul(self, rhs: Dimension) -> Self::Output {
        Vector(self.0, self.1 * rhs)
    }
}

impl<const N: usize> Div<Dimension> for Vector<N> {
    type Output = Vector<N>;
    fn div(self, rhs: Dimension) -> Self::Output {
        Vector(self.0, self.1 / rhs)
    }
}

impl From<Vector<2>> for Vec2 {
    fn from(v: Vector<2>) -> Vec2 {
        Vec2 {
            x: v.0[0] as _,
            y: v.0[1] as _,
        }
    }
}

impl From<Vector<3>> for Vec3 {
    fn from(v: Vector<3>) -> Vec3 {
        Vec3 {
            x: v.0[0] as _,
            y: v.0[1] as _,
            z: v.0[2] as _,
        }
    }
}

impl<const N: usize> From<Vector<N>> for Dimension {
    fn from(val: Vector<N>) -> Dimension {
        val.1
    }
}
