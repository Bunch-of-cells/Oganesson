use std::{
    ops::{Add, Mul, Neg},
    rc::Rc,
};

use crate::{
    unit::{Unit, UnitError},
    units, Float, Scalar, Vector,
};

const STEP: Float = 0.1;

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
