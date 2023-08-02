use std::{
    ops::{Add, Mul, Neg},
    rc::Rc,
};

use crate::{
    unit::{Unit, UnitError},
    units, Scalar, Vector, STEP,
};

#[derive(Clone)]
pub struct ScalarField<'a, const N: usize> {
    field: Rc<dyn Fn(Vector<N>) -> Scalar + 'a>,
    unit: Unit,
}

impl<const N: usize> ScalarField<'_, N> {
    fn derivative(&self, x: Vector<N>, i: usize) -> Vector<N> {
        (self.at(x + STEP * units::m * Vector::basis(i)).unwrap()
            - self.at(x - STEP * units::m * Vector::basis(i)).unwrap())
            / (2.0 * STEP)
            * Vector::basis(i)
            / units::m
    }

    pub fn unit(&self) -> Unit {
        self.unit
    }

    pub fn at(&self, x: Vector<N>) -> Result<Scalar, UnitError> {
        x.get_uniterror(units::m, "x")?;
        let at = (self.field)(x);
        assert_eq!(at.1, self.unit);
        Ok(at)
    }

    pub fn gradient(&self) -> VectorField<N> {
        (
            |x| {
                (0..N).fold(Vector::zero() * self.unit / units::m, |acc, i| {
                    acc + self.derivative(x, i)
                })
            },
            self.unit / units::m,
        )
            .into()
    }
}

impl<'a, const N: usize, F> From<(F, Unit)> for ScalarField<'a, N>
where
    F: Fn(Vector<N>) -> Scalar + 'a,
{
    fn from(field: (F, Unit)) -> Self {
        ScalarField {
            field: Rc::new(field.0),
            unit: field.1,
        }
    }
}

impl<'a, const N: usize> Add for ScalarField<'a, N> {
    type Output = ScalarField<'a, N>;
    #[track_caller]
    fn add(mut self, rhs: Self) -> Self::Output {
        if self.unit != rhs.unit {
            panic!(
                "Cannot add scalar fields of units {} and {}",
                self.unit, rhs.unit
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
        self.unit = self.unit * rhs.1;
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
    unit: Unit,
}

impl<const N: usize> VectorField<'_, N> {
    fn derivative(&self, x: Vector<N>, i: usize) -> Scalar {
        (self.at(x + STEP * units::m * Vector::basis(i)).unwrap().0[i]
            - self.at(x - STEP * units::m * Vector::basis(i)).unwrap().0[i])
            / (2.0 * STEP)
            * self.unit
            / units::m
    }

    pub fn unit(&self) -> Unit {
        self.unit
    }

    #[track_caller]
    pub fn impose(&mut self, s: Scalar, new: Self) -> Result<(), UnitError> {
        if self.unit != new.unit {
            panic!(
                "Cannot impose a vector field of units {} on a vector field of unit {}",
                new.unit, self.unit
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

    pub fn at(&self, x: Vector<N>) -> Result<Vector<N>, UnitError> {
        x.get_uniterror(units::m, "x")?;
        let at = (self.field)(x);
        assert_eq!(at.1, self.unit);
        Ok(at)
    }

    pub fn divergence(&self) -> ScalarField<N> {
        (
            move |x| {
                (0..N).fold(Scalar::zero() * self.unit / units::m, |acc, i| {
                    acc + self.derivative(x, i)
                })
            },
            self.unit / units::m,
        )
            .into()
    }
}

impl VectorField<'_, 3> {
    pub fn curl(&self) -> VectorField<3> {
        (
            move |x| {
                Vector(
                    [
                        self.derivative(x, 2) - self.derivative(x, 1),
                        self.derivative(x, 0) - self.derivative(x, 2),
                        self.derivative(x, 1) - self.derivative(x, 0),
                    ]
                    .map(|s| s.value()),
                    self.unit / units::m,
                )
            },
            self.unit / units::m,
        )
            .into()
    }
}

impl<'a, const N: usize, F> From<(F, Unit)> for VectorField<'a, N>
where
    F: Fn(Vector<N>) -> Vector<N> + 'a,
{
    fn from(field: (F, Unit)) -> Self {
        VectorField {
            field: Rc::new(field.0),
            unit: field.1,
        }
    }
}

impl<'a, const N: usize> Add for VectorField<'a, N> {
    type Output = VectorField<'a, N>;
    #[track_caller]
    fn add(mut self, rhs: Self) -> Self::Output {
        if self.unit != rhs.unit {
            panic!(
                "Cannot add vector fields of units {} and {}",
                self.unit, rhs.unit
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
        self.unit = self.unit * rhs.1;
        self
    }
}

impl<'a, const N: usize> Mul<Vector<N>> for ScalarField<'a, N> {
    type Output = VectorField<'a, N>;
    fn mul(self, rhs: Vector<N>) -> Self::Output {
        VectorField {
            field: Rc::new(move |x| (self.field)(x) * rhs),
            unit: self.unit * rhs.1,
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
}
