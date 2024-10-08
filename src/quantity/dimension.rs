use std::{
    error::Error,
    fmt::{Debug, Display, Write},
    ops::{Div, Mul},
};

use crate::{Float, Scalar, Vector};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum SIPrefix {
    /// quetta
    Q = 30,
    /// ronna
    R = 27,
    /// yotta
    Y = 24,
    /// zetta
    Z = 21,
    /// exa
    E = 18,
    /// peta
    P = 15,
    /// tera
    N = 12,
    /// giga
    G = 9,
    /// mega
    M = 6,
    /// kilo
    k = 3,
    /// hecto
    h = 2,
    /// deca
    da = 1,
    /// deci
    d = -1,
    /// centi
    c = -2,
    /// milli
    m = -3,
    /// micro,
    μ = -6,
    /// nano
    n = -9,
    /// pico
    p = -12,
    /// femto
    f = -15,
    /// atto
    a = -18,
    /// zepto
    z = -21,
    /// yocto
    y = -24,
    /// ronto
    r = -27,
    /// quecto
    q = -30,
}

impl Mul<Float> for SIPrefix {
    type Output = Scalar;
    fn mul(self, rhs: Float) -> Self::Output {
        Scalar(rhs * Float::powi(10.0, self as _), Dimension::NONE)
    }
}

impl Mul<SIPrefix> for Float {
    type Output = Scalar;
    fn mul(self, rhs: SIPrefix) -> Self::Output {
        rhs * self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dimension {
    pub time: i32,
    pub length: i32,
    pub mass: i32,
    pub electric_current: i32,
    pub thermodynamic_temperature: i32,
    pub amount_of_substance: i32,
    pub luminous_intensity: i32,
}

impl Dimension {
    pub const NONE: Dimension = Dimension {
        length: 0,
        mass: 0,
        time: 0,
        thermodynamic_temperature: 0,
        electric_current: 0,
        amount_of_substance: 0,
        luminous_intensity: 0,
    };

    /// time
    pub const T: Dimension = Dimension {
        length: 0,
        mass: 0,
        time: 1,
        thermodynamic_temperature: 0,
        electric_current: 0,
        amount_of_substance: 0,
        luminous_intensity: 0,
    };

    /// length
    pub const L: Dimension = Dimension {
        length: 1,
        mass: 0,
        time: 0,
        thermodynamic_temperature: 0,
        electric_current: 0,
        amount_of_substance: 0,
        luminous_intensity: 0,
    };

    /// mass
    pub const M: Dimension = Dimension {
        length: 0,
        mass: 1,
        time: 0,
        thermodynamic_temperature: 0,
        electric_current: 0,
        amount_of_substance: 0,
        luminous_intensity: 0,
    };

    /// electric current
    pub const I: Dimension = Dimension {
        length: 0,
        mass: 0,
        time: 0,
        thermodynamic_temperature: 0,
        electric_current: 1,
        amount_of_substance: 0,
        luminous_intensity: 0,
    };

    /// absolute temperature
    pub const Θ: Dimension = Dimension {
        length: 0,
        mass: 0,
        time: 0,
        thermodynamic_temperature: 1,
        electric_current: 0,
        amount_of_substance: 0,
        luminous_intensity: 0,
    };

    /// amount of substance
    pub const N: Dimension = Dimension {
        length: 0,
        mass: 0,
        time: 0,
        thermodynamic_temperature: 0,
        electric_current: 0,
        amount_of_substance: 1,
        luminous_intensity: 0,
    };

    /// luminous intensity
    pub const J: Dimension = Dimension {
        length: 0,
        mass: 0,
        time: 0,
        thermodynamic_temperature: 0,
        electric_current: 0,
        amount_of_substance: 0,
        luminous_intensity: 1,
    };

    pub const fn mul(self, rhs: Self) -> Dimension {
        Dimension {
            length: self.length + rhs.length,
            mass: self.mass + rhs.mass,
            time: self.time + rhs.time,
            thermodynamic_temperature: self.thermodynamic_temperature
                + rhs.thermodynamic_temperature,
            electric_current: self.electric_current + rhs.electric_current,
            amount_of_substance: self.amount_of_substance + rhs.amount_of_substance,
            luminous_intensity: self.luminous_intensity + rhs.luminous_intensity,
        }
    }

    pub const fn div(self, rhs: Self) -> Dimension {
        Dimension {
            length: self.length - rhs.length,
            mass: self.mass - rhs.mass,
            time: self.time - rhs.time,
            thermodynamic_temperature: self.thermodynamic_temperature
                - rhs.thermodynamic_temperature,
            electric_current: self.electric_current - rhs.electric_current,
            amount_of_substance: self.amount_of_substance - rhs.amount_of_substance,
            luminous_intensity: self.luminous_intensity - rhs.luminous_intensity,
        }
    }

    pub const fn pow(self, exp: i32) -> Dimension {
        Dimension {
            length: self.length * exp,
            mass: self.mass * exp,
            time: self.time * exp,
            thermodynamic_temperature: self.thermodynamic_temperature * exp,
            electric_current: self.electric_current * exp,
            amount_of_substance: self.amount_of_substance * exp,
            luminous_intensity: self.luminous_intensity * exp,
        }
    }

    #[inline(always)]
    pub fn radical(self, exp: i32) -> Dimension {
        if [
            self.length,
            self.mass,
            self.time,
            self.thermodynamic_temperature,
            self.electric_current,
            self.amount_of_substance,
            self.luminous_intensity,
        ]
        .iter()
        .any(|&dim| dim % exp != 0)
        {
            panic!("Can't");
        }

        Dimension {
            length: self.length / exp,
            mass: self.mass / exp,
            time: self.time / exp,
            thermodynamic_temperature: self.thermodynamic_temperature / exp,
            electric_current: self.electric_current / exp,
            amount_of_substance: self.amount_of_substance / exp,
            luminous_intensity: self.luminous_intensity / exp,
        }
    }

    pub const fn inv(self) -> Dimension {
        Dimension {
            length: -self.length,
            mass: -self.mass,
            time: -self.time,
            thermodynamic_temperature: -self.thermodynamic_temperature,
            electric_current: -self.electric_current,
            amount_of_substance: -self.amount_of_substance,
            luminous_intensity: -self.luminous_intensity,
        }
    }

    pub fn dimentional_formula(&self) -> String {
        let mut out = String::new();

        let dimensions = [
            ("T", self.time),
            ("L", self.length),
            ("M", self.mass),
            ("I", self.electric_current),
            ("Θ", self.thermodynamic_temperature),
            ("N", self.amount_of_substance),
            ("J", self.luminous_intensity),
        ];

        dimensions
            .into_iter()
            .filter(|&(_, exp)| exp != 0)
            .try_for_each(|(dim, exp)| {
                let dim = match exp {
                    1 => dim.to_string(),
                    _ => format!("{}^{}", dim, exp),
                };
                write!(out, "{} ", dim)
            })
            .unwrap();

        out
    }
}

impl Mul for Dimension {
    type Output = Dimension;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul(rhs)
    }
}

impl Div for Dimension {
    type Output = Dimension;

    fn div(self, rhs: Self) -> Self::Output {
        self.div(rhs)
    }
}

impl Default for Dimension {
    fn default() -> Self {
        Dimension::NONE
    }
}

impl Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self == &Self::NONE {
            return Ok(());
        }

        let mut out = String::new();

        let mut dimensions = [
            ("T", self.time),
            ("L", self.length),
            ("M", self.mass),
            ("I", self.electric_current),
            ("Θ", self.thermodynamic_temperature),
            ("N", self.amount_of_substance),
            ("J", self.luminous_intensity),
        ];

        dimensions.sort_by_key(|&(_, exp)| -exp);

        let mut denominator = false;

        dimensions
            .into_iter()
            .filter(|&(_, exp)| exp != 0)
            .try_for_each(|(dim, exp)| {
                let dim = match exp {
                    1 => dim.to_string(),
                    _ if exp < 0 => {
                        if !denominator {
                            denominator = true;
                            write!(out, "/ ")?;
                        }
                        match exp {
                            -1 => dim.to_string(),
                            _ => format!("{}^{}", dim, -exp),
                        }
                    }
                    _ => format!("{}^{}", dim, exp),
                };
                write!(out, "{} ", dim)
            })?;

        write!(f, "{}", out.trim_end())
    }
}

pub struct DimensionError(pub String);

impl DimensionError {
    pub fn new(message: &str) -> DimensionError {
        DimensionError(message.to_string())
    }

    pub fn expected_dimension_of(
        expected: Dimension,
        found: Dimension,
        var: &str,
    ) -> DimensionError {
        DimensionError(format!(
            "Expected dimension {} for {}, found {}",
            expected, var, found
        ))
    }
}

impl Display for DimensionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for DimensionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for DimensionError {}

impl Mul<Float> for Dimension {
    type Output = Scalar;
    fn mul(self, rhs: Float) -> Self::Output {
        Scalar(rhs, self)
    }
}

impl Div<Float> for Dimension {
    type Output = Scalar;
    fn div(self, rhs: Float) -> Self::Output {
        Scalar(rhs, self.inv())
    }
}

impl Mul<Dimension> for Float {
    type Output = Scalar;
    fn mul(self, rhs: Dimension) -> Self::Output {
        Scalar(self, rhs)
    }
}

impl Div<Dimension> for Float {
    type Output = Scalar;
    fn div(self, rhs: Dimension) -> Self::Output {
        Scalar(self, rhs.inv())
    }
}

impl<const N: usize> Mul<Dimension> for [Float; N] {
    type Output = Vector<N>;
    fn mul(self, rhs: Dimension) -> Self::Output {
        Vector(self, rhs)
    }
}

impl<const N: usize> Div<Dimension> for [Float; N] {
    type Output = Vector<N>;
    fn div(self, rhs: Dimension) -> Self::Output {
        Vector(self, rhs.inv())
    }
}
