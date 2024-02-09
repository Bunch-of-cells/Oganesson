use std::{
    ops::{Add, Mul, Neg},
    rc::Rc,
};

use crate::{
    dimension::{Dimension, DimensionError},
    units, Scalar, Vector, STEP,
};

#[derive(Clone)]
pub struct ScalarField<'a, const N: usize> {
    field: Rc<dyn Fn(Vector<N>) -> Scalar + 'a>,
    dim: Dimension,
}

impl<const N: usize> ScalarField<'_, N> {
    fn derivative(&self, x: Vector<N>, n: Vector<N>) -> Scalar {
        (self.at(x + STEP * n).unwrap() - self.at(x - STEP * n).unwrap()) / (2.0 * STEP) / n.dim()
    }

    fn derivative2(&self, x: Vector<N>, n: Vector<N>) -> Scalar {
        (self.at(x + STEP * n).unwrap() - 2.0 * self.at(x).unwrap()
            + self.at(x - STEP * n).unwrap())
            / STEP.powi(2)
            / n.dim().pow(2)
    }

    pub fn dim(&self) -> Dimension {
        self.dim
    }

    pub fn at(&self, x: Vector<N>) -> Result<Scalar, DimensionError> {
        x.dimension_err(units::m.dim(), "x")?;
        let at = (self.field)(x);
        assert_eq!(at.1, self.dim);
        Ok(at)
    }

    pub fn gradient(&self) -> VectorField<N> {
        (
            |x| {
                (0..N).fold(Vector::zero() * self.dim / units::m, |acc, i| {
                    acc + self.derivative(x, Vector::basis(i)) * Vector::basis(i)
                })
            },
            self.dim / units::m,
        )
            .into()
    }

    pub fn laplacian(&self) -> ScalarField<N> {
        (
            move |x| {
                (0..N).fold(Scalar::ZERO * self.dim / units::m.powi(2), |acc, i| {
                    acc + self.derivative2(x, Vector::basis(i))
                })
            },
            self.dim / units::m.powi(2),
        )
            .into()
    }
}

impl<'a, const N: usize, F, D: Into<Dimension>> From<(F, D)> for ScalarField<'a, N>
where
    F: Fn(Vector<N>) -> Scalar + 'a,
{
    fn from(field: (F, D)) -> Self {
        ScalarField {
            field: Rc::new(field.0),
            dim: field.1.into(),
        }
    }
}

impl<'a, const N: usize> Add for ScalarField<'a, N> {
    type Output = ScalarField<'a, N>;
    #[track_caller]
    fn add(mut self, rhs: Self) -> Self::Output {
        if self.dim != rhs.dim {
            panic!(
                "Cannot add scalar fields of dimensions {} and {}",
                self.dim, rhs.dim
            )
        }
        self.field = Rc::new(move |x| (self.field)(x) + (rhs.field)(x));
        self
    }
}

impl<'a, const N: usize> Mul<Scalar> for ScalarField<'a, N> {
    type Output = ScalarField<'a, N>;
    fn mul(mut self, rhs: Scalar) -> Self::Output {
        self.field = Rc::new(move |x| (self.field)(x) * rhs);
        self.dim = self.dim * rhs.1;
        self
    }
}

impl<'a, const N: usize> Neg for ScalarField<'a, N> {
    type Output = ScalarField<'a, N>;
    fn neg(mut self) -> Self::Output {
        self.field = Rc::new(move |x| -(self.field)(x));
        self
    }
}

#[derive(Clone)]
pub struct VectorField<'a, const N: usize> {
    field: Rc<dyn Fn(Vector<N>) -> Vector<N> + 'a>,
    dim: Dimension,
}

impl<const N: usize> VectorField<'_, N> {
    fn derivative(&self, x: Vector<N>, n: Vector<N>) -> Scalar {
        (self.at(x + STEP * n).unwrap() - self.at(x - STEP * n).unwrap()).dot(n)
            / (2.0 * STEP)
            / n.dim()
    }

    // fn derivative2(&self, x: Vector<N>, n: Vector<N>) -> Scalar {
    //     (self.at(x + STEP * n).unwrap() - 2.0 * self.at(x).unwrap()
    //         + self.at(x - STEP  * n).unwrap())
    //     .dot(n)
    //         / STEP.powi(2)
    //         / n.dim().pow(2)
    // }

    pub fn dim(&self) -> Dimension {
        self.dim
    }

    #[track_caller]
    pub fn impose(&mut self, s: Scalar, new: Self) -> Result<(), DimensionError> {
        if self.dim != new.dim {
            panic!(
                "Cannot impose a vector field of dimensions {} on a vector field of dimension {}",
                new.dim, self.dim
            )
        }
        let old = self.field.clone();
        self.field = Rc::new(move |x: Vector<N>| {
            if x.squared() < s.squared() {
                (new.field)(x)
            } else {
                old(x)
            }
        });
        Ok(())
    }

    pub fn at(&self, x: Vector<N>) -> Result<Vector<N>, DimensionError> {
        x.dimension_err(units::m, "x")?;
        let at = (self.field)(x);
        assert_eq!(at.1, self.dim);
        Ok(at)
    }

    pub fn divergence(&self) -> ScalarField<N> {
        (
            move |x| {
                (0..N).fold(Scalar::ZERO * self.dim / units::m, |acc, i| {
                    acc + self.derivative(x, Vector::basis(i))
                })
            },
            self.dim / units::m,
        )
            .into()
    }
}

impl VectorField<'_, 3> {
    pub fn curl(&self) -> VectorField<3> {
        (
            move |x| {
                [
                    self.derivative(x, Vector::<3>::k) - self.derivative(x, Vector::<3>::j),
                    self.derivative(x, Vector::<3>::i) - self.derivative(x, Vector::<3>::k),
                    self.derivative(x, Vector::<3>::j) - self.derivative(x, Vector::<3>::i),
                ]
                .map(|s| s.value())
                    * self.dim
                    / units::m
            },
            self.dim / units::m,
        )
            .into()
    }

    // pub fn laplacian(&self) -> VectorField<3> {
    //     let c1 = self.curl();
    //     let c2 = c1.curl();
    //     let d1 = self.divergence();
    //     let g1 = d1.gradient();
    //     g1 + (-c2)
    // }
}

impl<'a, const N: usize, F, D: Into<Dimension>> From<(F, D)> for VectorField<'a, N>
where
    F: Fn(Vector<N>) -> Vector<N> + 'a,
{
    fn from(field: (F, D)) -> Self {
        VectorField {
            field: Rc::new(field.0),
            dim: field.1.into(),
        }
    }
}

impl<'a, const N: usize> Add for VectorField<'a, N> {
    type Output = VectorField<'a, N>;
    #[track_caller]
    fn add(mut self, rhs: Self) -> Self::Output {
        if self.dim != rhs.dim {
            panic!(
                "Cannot add vector fields of dimensions {} and {}",
                self.dim, rhs.dim
            )
        }
        self.field = Rc::new(move |x| (self.field)(x) + (rhs.field)(x));
        self
    }
}

impl<'a, const N: usize> Mul<Scalar> for VectorField<'a, N> {
    type Output = VectorField<'a, N>;
    fn mul(mut self, rhs: Scalar) -> Self::Output {
        self.field = Rc::new(move |x| (self.field)(x) * rhs);
        self.dim = self.dim * rhs.1;
        self
    }
}

impl<'a, const N: usize> Mul<Vector<N>> for ScalarField<'a, N> {
    type Output = VectorField<'a, N>;
    fn mul(self, rhs: Vector<N>) -> Self::Output {
        VectorField {
            field: Rc::new(move |x| (self.field)(x) * rhs),
            dim: self.dim * rhs.1,
        }
    }
}

impl<'a, const N: usize> Neg for VectorField<'a, N> {
    type Output = VectorField<'a, N>;
    fn neg(mut self) -> Self::Output {
        self.field = Rc::new(move |x| -(self.field)(x));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::*;

    #[test]
    fn test_grad() {
        let f = ScalarField::from((|x: Vector<3>| x[0] * m, m));
        let f = f.gradient();
        assert_eq!(f.at(Vector::zero() * m).unwrap().0[0], 1.0)
    }

    #[test]
    fn test_div() {
        let f = VectorField::from((|x: Vector<3>| x, m));
        let f = f.divergence();
        assert_eq!(f.at(Vector::zero() * m).unwrap(), 3.0)
    }

    #[test]
    fn test_curl() {
        let f = VectorField::from((|x: Vector<3>| x, m));
        let f = f.curl();
        assert_eq!(f.at(Vector::zero() * m).unwrap(), Vector::zero())
    }

    #[test]
    fn test_curl_of_grad() {
        let f = ScalarField::from((|x: Vector<3>| x.dot(5.0 * Vector::<3>::i * m), m * m));
        let grad = f.gradient();
        let curl = grad.curl();
        assert_eq!(curl.at(Vector::zero() * m).unwrap(), Vector::zero());
        assert_eq!(curl.at([2.0, 3.5, 7.8] * m).unwrap(), Vector::zero());
    }
}
