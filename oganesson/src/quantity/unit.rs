use std::{
    error::Error,
    fmt::{Debug, Display, Write},
    ops::{Div, Mul},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unit {
    pub length: i32,
    pub mass: i32,
    pub time: i32,
    pub temperature: i32,
    pub current: i32,
    pub amount_of_substance: i32,
    pub luminous_intensity: i32,
}

impl Unit {
    pub const NONE: Unit = Unit {
        length: 0,
        mass: 0,
        time: 0,
        temperature: 0,
        current: 0,
        amount_of_substance: 0,
        luminous_intensity: 0,
    };

    /// time
    pub const T: Unit = Unit {
        length: 0,
        mass: 0,
        time: 1,
        temperature: 0,
        current: 0,
        amount_of_substance: 0,
        luminous_intensity: 0,
    };

    /// length
    pub const L: Unit = Unit {
        length: 1,
        mass: 0,
        time: 0,
        temperature: 0,
        current: 0,
        amount_of_substance: 0,
        luminous_intensity: 0,
    };

    /// mass
    pub const M: Unit = Unit {
        length: 0,
        mass: 1,
        time: 0,
        temperature: 0,
        current: 0,
        amount_of_substance: 0,
        luminous_intensity: 0,
    };

    /// electric current
    pub const I: Unit = Unit {
        length: 0,
        mass: 0,
        time: 0,
        temperature: 0,
        current: 1,
        amount_of_substance: 0,
        luminous_intensity: 0,
    };

    /// absolute temperature
    pub const Θ: Unit = Unit {
        length: 0,
        mass: 0,
        time: 0,
        temperature: 1,
        current: 0,
        amount_of_substance: 0,
        luminous_intensity: 0,
    };

    /// amount of substance
    pub const N: Unit = Unit {
        length: 0,
        mass: 0,
        time: 0,
        temperature: 0,
        current: 0,
        amount_of_substance: 1,
        luminous_intensity: 0,
    };

    /// luminous intensity
    pub const J: Unit = Unit {
        length: 0,
        mass: 0,
        time: 0,
        temperature: 0,
        current: 0,
        amount_of_substance: 0,
        luminous_intensity: 1,
    };

    pub const fn mul(self, rhs: Self) -> Unit {
        Unit {
            length: self.length + rhs.length,
            mass: self.mass + rhs.mass,
            time: self.time + rhs.time,
            temperature: self.temperature + rhs.temperature,
            current: self.current + rhs.current,
            amount_of_substance: self.amount_of_substance + rhs.amount_of_substance,
            luminous_intensity: self.luminous_intensity + rhs.luminous_intensity,
        }
    }

    pub const fn div(self, rhs: Self) -> Unit {
        Unit {
            length: self.length - rhs.length,
            mass: self.mass - rhs.mass,
            time: self.time - rhs.time,
            temperature: self.temperature - rhs.temperature,
            current: self.current - rhs.current,
            amount_of_substance: self.amount_of_substance - rhs.amount_of_substance,
            luminous_intensity: self.luminous_intensity - rhs.luminous_intensity,
        }
    }

    pub const fn pow(self, exp: i32) -> Unit {
        Unit {
            length: self.length * exp,
            mass: self.mass * exp,
            time: self.time * exp,
            temperature: self.temperature * exp,
            current: self.current * exp,
            amount_of_substance: self.amount_of_substance * exp,
            luminous_intensity: self.luminous_intensity * exp,
        }
    }

    pub fn try_radical(self, exp: i32) -> Option<Unit> {
        if [
            self.length,
            self.mass,
            self.time,
            self.temperature,
            self.current,
            self.amount_of_substance,
            self.luminous_intensity,
        ].iter().any(|unit| (*unit as f32).powf(1.0 / exp as f32) % 1.0 != 0.0) {
            return None
        }

        Some(Unit {
            length: self.length / exp,
            mass: self.mass / exp,
            time: self.time / exp,
            temperature: self.temperature / exp,
            current: self.current / exp,
            amount_of_substance: self.amount_of_substance / exp,
            luminous_intensity: self.luminous_intensity / exp,
        })
    }

    pub fn dimentional_formula(&self) -> String {
        let mut out = String::new();

        let units = [
            ("T", self.time),
            ("L", self.length),
            ("M", self.mass),
            ("I", self.current),
            ("Θ", self.temperature),
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
            ("A", self.current),
            ("K", self.temperature),
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

    pub fn expected_unit(expected: Unit, found: Unit) -> UnitError {
        UnitError(format!("Expected unit {}, found {}", expected, found))
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
