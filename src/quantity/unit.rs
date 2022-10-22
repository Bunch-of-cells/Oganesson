use std::{
    error::Error,
    fmt::{Debug, Display, Write},
    ops::{Div, Mul},
};

use crate::{Float, Scalar};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum SIPrefix {
    /// yotta
    Y = 24,
    /// zetta
    Z = 21,
    /// exa
    E = 18,
    /// peta
    P = 15,
    /// tera
    T = 12,
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
}

impl Mul<Float> for SIPrefix {
    type Output = Scalar;
    fn mul(self, rhs: Float) -> Self::Output {
        Scalar(rhs * Float::powi(10.0, self as _), Unit::NONE)
    }
}

impl Mul<SIPrefix> for Float {
    type Output = Scalar;
    fn mul(self, rhs: SIPrefix) -> Self::Output {
        rhs * self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unit {
    pub time: i32,
    pub length: i32,
    pub mass: i32,
    pub electric_current: i32,
    pub thermodynamic_temperature: i32,
    pub amount_of_substance: i32,
    pub luminous_intensity: i32,
}

impl Unit {
    pub const NONE: Unit = Unit {
        length: 0,
        mass: 0,
        time: 0,
        thermodynamic_temperature: 0,
        electric_current: 0,
        amount_of_substance: 0,
        luminous_intensity: 0,
    };

    /// time
    pub const T: Unit = Unit {
        length: 0,
        mass: 0,
        time: 1,
        thermodynamic_temperature: 0,
        electric_current: 0,
        amount_of_substance: 0,
        luminous_intensity: 0,
    };

    /// length
    pub const L: Unit = Unit {
        length: 1,
        mass: 0,
        time: 0,
        thermodynamic_temperature: 0,
        electric_current: 0,
        amount_of_substance: 0,
        luminous_intensity: 0,
    };

    /// mass
    pub const M: Unit = Unit {
        length: 0,
        mass: 1,
        time: 0,
        thermodynamic_temperature: 0,
        electric_current: 0,
        amount_of_substance: 0,
        luminous_intensity: 0,
    };

    /// electric current
    pub const I: Unit = Unit {
        length: 0,
        mass: 0,
        time: 0,
        thermodynamic_temperature: 0,
        electric_current: 1,
        amount_of_substance: 0,
        luminous_intensity: 0,
    };

    /// absolute temperature
    pub const Θ: Unit = Unit {
        length: 0,
        mass: 0,
        time: 0,
        thermodynamic_temperature: 1,
        electric_current: 0,
        amount_of_substance: 0,
        luminous_intensity: 0,
    };

    /// amount of substance
    pub const N: Unit = Unit {
        length: 0,
        mass: 0,
        time: 0,
        thermodynamic_temperature: 0,
        electric_current: 0,
        amount_of_substance: 1,
        luminous_intensity: 0,
    };

    /// luminous intensity
    pub const J: Unit = Unit {
        length: 0,
        mass: 0,
        time: 0,
        thermodynamic_temperature: 0,
        electric_current: 0,
        amount_of_substance: 0,
        luminous_intensity: 1,
    };

    pub const fn mul(self, rhs: Self) -> Unit {
        Unit {
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

    pub const fn div(self, rhs: Self) -> Unit {
        Unit {
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

    pub const fn pow(self, exp: i32) -> Unit {
        Unit {
            length: self.length * exp,
            mass: self.mass * exp,
            time: self.time * exp,
            thermodynamic_temperature: self.thermodynamic_temperature * exp,
            electric_current: self.electric_current * exp,
            amount_of_substance: self.amount_of_substance * exp,
            luminous_intensity: self.luminous_intensity * exp,
        }
    }

    pub fn try_radical(self, exp: i32) -> Option<Unit> {
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
        .any(|&unit| unit % exp != 0)
        {
            return None;
        }

        Some(Unit {
            length: self.length / exp,
            mass: self.mass / exp,
            time: self.time / exp,
            thermodynamic_temperature: self.thermodynamic_temperature / exp,
            electric_current: self.electric_current / exp,
            amount_of_substance: self.amount_of_substance / exp,
            luminous_intensity: self.luminous_intensity / exp,
        })
    }

    pub const fn recip(self) -> Unit {
        Unit {
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

        let units = [
            ("T", self.time),
            ("L", self.length),
            ("M", self.mass),
            ("I", self.electric_current),
            ("Θ", self.thermodynamic_temperature),
            ("N", self.amount_of_substance),
            ("J", self.luminous_intensity),
        ];

        units
            .into_iter()
            .filter(|&(_, exp)| exp != 0)
            .try_for_each(|(unit, exp)| {
                let unit = match exp {
                    1 => unit.to_string(),
                    _ => format!("{}^{}", unit, exp),
                };
                write!(out, "{} ", unit)
            })
            .unwrap();

        out
    }
}

impl Mul for Unit {
    type Output = Unit;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul(rhs)
    }
}

impl Div for Unit {
    type Output = Unit;

    fn div(self, rhs: Self) -> Self::Output {
        self.div(rhs)
    }
}

impl Default for Unit {
    fn default() -> Self {
        Unit::NONE
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self == &Self::NONE {
            return Ok(());
        }

        let mut out = String::new();

        let mut units = [
            ("s", self.time),
            ("m", self.length),
            ("kg", self.mass),
            ("A", self.electric_current),
            ("K", self.thermodynamic_temperature),
            ("mol", self.amount_of_substance),
            ("cd", self.luminous_intensity),
        ];

        units.sort_by_key(|&(_, exp)| -exp);

        let mut denominator = false;

        units
            .into_iter()
            .filter(|&(_, exp)| exp != 0)
            .try_for_each(|(unit, exp)| {
                let unit = match exp {
                    1 => unit.to_string(),
                    _ if exp < 0 => {
                        if !denominator {
                            denominator = true;
                            write!(out, "/ ")?;
                        }
                        match exp {
                            -1 => unit.to_string(),
                            _ => format!("{}^{}", unit, -exp),
                        }
                    }
                    _ => format!("{}^{}", unit, exp),
                };
                write!(out, "{} ", unit)
            })?;

        write!(f, "{}", out.trim_end())
    }
}

pub struct UnitError(pub String);

impl UnitError {
    pub fn new(message: &str) -> UnitError {
        UnitError(message.to_string())
    }

    pub fn expected_unit_of(expected: Unit, found: Unit, var: &str) -> UnitError {
        UnitError(format!(
            "Expected unit {} for {}, found {}",
            expected, var, found
        ))
    }
}

impl Display for UnitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for UnitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for UnitError {}

impl Mul<Float> for Unit {
    type Output = Scalar;
    fn mul(self, rhs: Float) -> Self::Output {
        Scalar(rhs, self)
    }
}

impl Div<Float> for Unit {
    type Output = Scalar;
    fn div(self, rhs: Float) -> Self::Output {
        Scalar(rhs, self.recip())
    }
}

impl Mul<Unit> for Float {
    type Output = Scalar;
    fn mul(self, rhs: Unit) -> Self::Output {
        Scalar(self, rhs)
    }
}

impl Div<Unit> for Float {
    type Output = Scalar;
    fn div(self, rhs: Unit) -> Self::Output {
        Scalar(self, rhs.recip())
    }
}
