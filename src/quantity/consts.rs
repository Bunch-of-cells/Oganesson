#![allow(non_upper_case_globals)]
#![allow(clippy::excessive_precision)]

pub mod units {
    use crate::dimension::Dimension;
    use crate::quantity::PI;
    use crate::Scalar;

    /// Kilogram
    pub const kg: Scalar = Scalar(1.0, Dimension::M);

    /// Meter
    pub const m: Scalar = Scalar(1.0, Dimension::L);

    /// Second
    pub const s: Scalar = Scalar(1.0, Dimension::N);

    /// Ampere
    pub const A: Scalar = Scalar(1.0, Dimension::I);

    /// Kelvin
    pub const K: Scalar = Scalar(1.0, Dimension::Θ);

    /// Candela
    pub const cd: Scalar = Scalar(1.0, Dimension::J);

    /// Mole
    pub const mol: Scalar = Scalar(1.0, Dimension::N);

    /// Hertz
    pub const Hz: Scalar = Scalar(1.0, s.dim().inv());

    /// Radian
    pub const rad: Scalar = Scalar(1.0, Dimension::NONE);

    /// Steradian
    pub const sr: Scalar = Scalar(1.0, Dimension::NONE);

    /// Newton
    pub const N: Scalar = Scalar(1.0, kg.dim().mul(m.dim()).div(s.dim().pow(2)));

    /// Pascal
    pub const Pa: Scalar = Scalar(1.0, N.dim().div(m.dim().pow(2)));

    /// Joule
    pub const J: Scalar = Scalar(1.0, N.dim().mul(m.dim()));

    /// Watt
    pub const W: Scalar = Scalar(1.0, J.dim().div(s.dim()));

    /// Coulomb
    pub const C: Scalar = Scalar(1.0, A.dim().mul(s.dim()));

    /// Volt
    pub const V: Scalar = Scalar(1.0, W.dim().div(A.dim()));

    /// Farad
    pub const F: Scalar = Scalar(1.0, C.dim().div(V.dim()));

    /// Ohm
    pub const Ω: Scalar = Scalar(1.0, V.dim().div(A.dim()));

    /// Ohm
    pub const Ohm: Scalar = Ω;

    /// Siemens
    pub const S: Scalar = Scalar(1.0, Ω.dim().div(A.dim()));

    /// Weber
    pub const Wb: Scalar = Scalar(1.0, V.dim().div(Ω.dim()));

    /// Tesla
    pub const T: Scalar = Scalar(1.0, Wb.dim().div(m.dim().pow(2)));

    /// Henry
    pub const H: Scalar = Scalar(1.0, Wb.dim().div(A.dim()));

    /// Lumen
    pub const lm: Scalar = Scalar(1.0, cd.dim().mul(sr.dim()));

    /// Lux
    pub const lx: Scalar = Scalar(1.0, lm.dim().div(m.dim().pow(2)));

    /// Becquerel
    pub const Bq: Scalar = Hz;

    /// Gray
    pub const Gy: Scalar = Scalar(1.0, J.dim().div(kg.dim()));

    /// Sievert
    pub const Sv: Scalar = Gy;

    /// Katal
    pub const kat: Scalar = Scalar(1.0, mol.dim().div(s.dim()));

    /// Volt-Ampere Reactive
    pub const VA: Scalar = Scalar(1.0, V.dim().mul(A.dim()));

    /// Minute
    pub const min: Scalar = Scalar(60.0, s.dim());

    /// Hour
    pub const hr: Scalar = Scalar(3600.0, s.dim());

    /// Day
    pub const d: Scalar = Scalar(86400.0, s.dim());

    /// Astronomical Unit
    pub const au: Scalar = Scalar(149597870700.0, m.dim());

    /// Degree
    pub const deg: Scalar = Scalar(PI / 180.0, rad.dim());

    /// Arcminute
    pub const arcmin: Scalar = Scalar(PI / 1080.0, rad.dim());

    /// Arcsecond
    pub const arcsec: Scalar = Scalar(PI / 6480.0, rad.dim());

    /// Hectare
    pub const ha: Scalar = Scalar(10000.0, m.dim().pow(2));

    /// Litre
    pub const L: Scalar = Scalar(0.001, m.dim().pow(3));

    /// Tonne
    pub const t: Scalar = Scalar(1000.0, kg.dim());

    /// Dalton
    pub const Da: Scalar = Scalar(1.66053906660e-27, kg.dim());

    /// Galileo
    pub const Gal: Scalar = Scalar(0.01, m.dim().div(s.dim().pow(2)));

    /// Unified Atomic Mass Unit
    pub const u: Scalar = Da;

    /// Parsec
    pub const pc: Scalar = Scalar(3.0856775814913673e16, m.dim());

    /// Bar
    pub const bar: Scalar = Scalar(100000.0, Pa.dim());

    /// Standard Atmosphere
    pub const atm: Scalar = Scalar(101325.0, Pa.dim());

    /// ångström
    pub const Å: Scalar = Scalar(1e-10, m.dim());

    // Imperial Units -------------------------------------------------------------------

    /// twip
    pub const twip: Scalar = Scalar(0.0000176389, m.dim());

    /// thou
    pub const th: Scalar = Scalar(0.0000254, m.dim());

    /// barleycorn
    pub const barleycorn: Scalar = Scalar(0.0084667, m.dim());

    /// inch
    pub const inch: Scalar = Scalar(0.0254, m.dim());

    /// hand
    pub const hh: Scalar = Scalar(0.1016, m.dim());

    /// foot
    pub const ft: Scalar = Scalar(0.3048, m.dim());

    /// yard
    pub const yd: Scalar = Scalar(0.9144, m.dim());

    /// chain
    pub const ch: Scalar = Scalar(20.1168, m.dim());

    /// furlong
    pub const fur: Scalar = Scalar(201.168, m.dim());

    /// mile
    pub const mi: Scalar = Scalar(1609.344, m.dim());

    /// league
    pub const lea: Scalar = Scalar(4828.032, m.dim());

    /// fanthom
    pub const ftm: Scalar = Scalar(1.852, m.dim());

    /// cable
    pub const calbe: Scalar = Scalar(185.2, m.dim());

    /// nautical mile
    pub const nmi: Scalar = Scalar(1852.0, m.dim());

    /// link
    pub const link: Scalar = Scalar(0.201168, m.dim());

    /// rod
    pub const rod: Scalar = Scalar(5.0292, m.dim());

    /// perch
    pub const perch: Scalar = Scalar(25.29285264, m.dim().pow(2));

    /// rood
    pub const rood: Scalar = Scalar(1011.7141056, m.dim().pow(2));

    /// acre
    pub const acre: Scalar = Scalar(4046.8564224, m.dim().pow(2));

    /// square mile
    pub const sq_mi: Scalar = Scalar(2589988.110336, m.dim().pow(2));

    /// fluid ounce
    pub const fl_oz: Scalar = Scalar(28.4130625e-6, m.dim().pow(3));

    /// gill
    pub const gi: Scalar = Scalar(142.0653125e-6, m.dim().pow(3));

    /// pint
    pub const pt: Scalar = Scalar(568.26125e-6, m.dim().pow(3));

    /// quart
    pub const qt: Scalar = Scalar(1136.5225e-6, m.dim().pow(3));

    /// gallon
    pub const gal: Scalar = Scalar(4546.09e-6, m.dim().pow(3));

    /// grain
    pub const gr: Scalar = Scalar(64.79891e-6, kg.dim());

    /// drachm
    pub const dr: Scalar = Scalar(1.7718451953125e-3, kg.dim());

    /// ounce
    pub const oz: Scalar = Scalar(28.349523125e-3, kg.dim());

    /// pound
    pub const lb: Scalar = Scalar(0.45359237, kg.dim());

    /// stone
    pub const st: Scalar = Scalar(6.35029318, kg.dim());

    /// qaurter
    pub const qtr: Scalar = Scalar(12.70058636, kg.dim());

    /// hundredweight
    pub const cwt: Scalar = Scalar(50.80234544, kg.dim());

    /// ton
    pub const ton: Scalar = Scalar(1016.0469088, kg.dim());

    /// slug
    pub const slug: Scalar = Scalar(14.59390294, kg.dim());
}

pub mod constants {
    use super::units::*;
    pub use crate::quantity::{E, PI};
    use crate::{dimension::Dimension, dimension::SIPrefix, Scalar};

    // SI-UNITS-----------------------------------------------------------------

    /// speed of light in vacuum
    pub const c: Scalar = Scalar(299792458.0, m.dim().div(s.dim()));

    /// Planck constant
    pub const h: Scalar = Scalar(6.62607015e-34, J.dim().mul(s.dim()));

    /// Hyperfine transition frequency of 133Cs
    pub const ΔνCs: Scalar = Scalar(9192631770.0, Hz.dim());

    /// Elementary charge
    pub const e: Scalar = Scalar(1.602176634e-19, C.dim());

    /// Boltzmann constant
    pub const k_B: Scalar = Scalar(1.380649e-23, J.dim().div(K.dim()));

    /// Avogadro constant
    pub const N_A: Scalar = Scalar(6.02214076e23, mol.dim().inv());

    /// Luminous efficacy of 540 THz monochromatic radiation
    pub const K_cd: Scalar = Scalar(683.0, lm.dim().div(W.dim()));

    // --------------------------------------------------------------------------

    /// Newtonian constant of gravitation
    pub const G: Scalar = Scalar(6.6743e-11, m.dim().pow(3).div(kg.dim()).div(s.dim().pow(2)));

    /// Fine-structure constant
    pub const α: Scalar = Scalar(0.0072973525693, Dimension::NONE);

    /// Wien wavelength displacement law constant
    pub const b: Scalar = Scalar(2.897771955e-3, m.dim().mul(K.dim()));

    /// Wien frequency displacement law constant
    pub const b_freq: Scalar = Scalar(5.878925757e10, Hz.dim().div(K.dim()));

    /// Wien entropy displacement law constant
    pub const b_entropy: Scalar = Scalar(3.002916077e-3, m.dim().mul(K.dim()));

    /// Electron mass
    pub const m_e: Scalar = Scalar(9.1093837015e-31, kg.dim());

    /// Proton mass
    pub const m_p: Scalar = Scalar(1.67262192369e-27, kg.dim());

    /// Neutron mass
    pub const m_n: Scalar = Scalar(1.67492749804e-27, kg.dim());

    /// Muon mass
    pub const m_μ: Scalar = Scalar(1.883531627e-28, kg.dim());

    /// Tau mass
    pub const m_τ: Scalar = Scalar(3.16754e-27, kg.dim());

    /// Top quark mass
    pub const m_t: Scalar = Scalar(3.0784e-25, kg.dim());

    /// W to Z mass ratio
    pub const m_W_ratio_m_Z: Scalar = Scalar(0.88153, Dimension::NONE);

    /// Proton g-factor
    pub const g_p: Scalar = Scalar(5.5856946893, Dimension::NONE);

    /// Electron g-factor
    pub const g_e: Scalar = Scalar(-2.00231930436256, Dimension::NONE);

    /// Muon g-factor
    pub const g_μ: Scalar = Scalar(-2.0023318418, Dimension::NONE);

    crate::c! {
        /// speed of light in vacuum squared
        pub const c2: Scalar = c.squared();

        /// Reduced Planck constant / Dirac Constant
        pub const ℏ: Scalar = h / (2.0 * PI);

        /// Vacuum magnetic permeability
        pub const μ_0: Scalar = 2.0 * α * h / (e.squared() * c);

        /// Characteristic impedance of vacuum
        pub const Z_0: Scalar = (ε_0() * c).recip();

        /// Vacuum electric permittivity
        pub const ε_0: Scalar = (μ_0() * c2()).recip();

        /// Coulomb constant
        pub const k_e: Scalar = (4.0 * PI * ε_0()).recip();

        /// Stefan–Boltzmann constant
        pub const σ: Scalar = 2.0 * PI.powi(5) * k_B.powi(4) / (15.0 * h.powi(3) * c2());

        /// First radiation constant
        pub const c_1: Scalar = 2.0 * PI * h * c2();

        /// First radiation constant for spectral radiance
        pub const c_1L: Scalar = 2.0 * h * c2() / sr;

        /// Second radiation constant
        pub const c_2: Scalar = h * c / k_B;

        /// Conductance quantum
        pub const G_0: Scalar = 2.0 * e.squared() / h;

        /// Inverse conductance quantum
        pub const G_0_inv: Scalar = G_0().recip();

        /// Von Klitzing constant
        pub const R_K: Scalar = h / e.squared();

        /// Josephson constant
        pub const K_J: Scalar = 2.0 * e / h;

        /// Magnetic Flux Quantum
        pub const Φ_0: Scalar = K_J().recip();

        /// Inverse fine-structure constant
        pub const α_inv: Scalar = α.recip();

        /// Proton to electron mass ratio
        pub const m_p_ratio_m_e: Scalar = m_p / m_e;

        /// Weak mixing angle
        pub const θ_W: Scalar = m_W_ratio_m_Z.acos() * Dimension::NONE;

        /// sin^2 Weak mixing angle
        pub const sin2_θ_W: Scalar = 1.0 - m_W_ratio_m_Z.squared();

        /// Quantum of circulation
        pub const QuantumOfCirculation: Scalar = h / (2.0 * m_e);

        /// Bohr magneton
        pub const μ_B: Scalar = e * h / (4.0 * PI * m_e);

        /// Nuclear magneton
        pub const μ_N: Scalar = e * h / (4.0 * PI * m_p);

        /// Classical electron radius
        pub const r_e: Scalar = e.squared() * k_e() / (m_e * c2());

        /// Thomson cross section
        pub const σ_e: Scalar = 8.0 * PI * r_e().squared() / 3.0;

        /// Bohr radius
        pub const a_0: Scalar = r_e() / α.squared();

        /// Hartree energy
        pub const E_h: Scalar = α.squared() * c2() * m_e;

        /// Rydberg unit of energy
        pub const R_y: Scalar = E_h() / 2.0;

        /// Rydberg constant
        pub const R_H: Scalar = α.squared() * m_e * c / (2.0 * h);

        /// Molar gas constant
        pub const R: Scalar = N_A * k_B;

        /// Faraday constant
        pub const F: Scalar = N_A * e;

        /// Atomic mass of carbon-12
        pub const m_12C: Scalar = 12.0 * SIPrefix::m * kg / N_A;

        /// Molar mass of carbon-12
        pub const M_12C: Scalar = N_A * m_12C();

        /// Atomic mass constant
        pub const m_u: Scalar = m_12C() / 12.0;

        /// Molar mass constant
        pub const M_u: Scalar = M_12C() / 12.0;

        /// Einstein gravitational constant
        pub const κ: Scalar = 8.0 * PI * G / c2();

        /// Electron Volt
        pub const eV: Scalar = e * V;

        /// Molar Planck constant
        pub const N_A_h: Scalar = N_A * h;

        /// Planck length
        pub const l_P: Scalar = (ℏ() * G / c.powi(3)).sqrt();

        /// Planck time
        pub const t_P: Scalar = (ℏ() * G / c.powi(5)).sqrt();

        /// Planck mass
        pub const m_P: Scalar = (ℏ() * c / G).sqrt();

        /// Planck temperature
        pub const T_P: Scalar = (ℏ() * c.powi(5) / G).sqrt() / k_B;
    }
}

use crate::Vector;
use units::{m, s};

/// standard gravitational acceleration for the surface of the Earth
pub const g: Vector<3> = Vector([0.0, 9.80665, 0.0], m.dim().div(s.dim().pow(2)));
