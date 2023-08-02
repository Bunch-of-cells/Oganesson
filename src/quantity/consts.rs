#![allow(non_upper_case_globals)]
#![allow(clippy::excessive_precision)]

pub mod units {
    use crate::unit::Unit;
    use crate::Scalar;
    use std::f64::consts::PI;

    /// Kilogram
    pub const kg: Unit = Unit::M;

    /// Meter
    pub const m: Unit = Unit::L;

    /// Second
    pub const s: Unit = Unit::T;

    /// Ampere
    pub const A: Unit = Unit::I;

    /// Kelvin
    pub const K: Unit = Unit::Θ;

    /// Candela
    pub const cd: Unit = Unit::J;

    /// Mole
    pub const mol: Unit = Unit::N;

    /// Unitless
    pub const Null: Unit = Unit::NONE;

    /// Hertz
    pub const Hz: Unit = Null.div(s);

    /// Radian
    pub const rad: Unit = Null;

    /// Steradian
    pub const sr: Unit = Null;

    /// Newton
    pub const N: Unit = kg.mul(m).div(s.pow(2));

    /// Pascal
    pub const Pa: Unit = N.div(m.pow(2));

    /// Joule
    pub const J: Unit = N.mul(m);

    /// Watt
    pub const W: Unit = J.div(s);

    /// Coulomb
    pub const C: Unit = A.mul(s);

    /// Volt
    pub const V: Unit = W.div(A);

    /// Farad
    pub const F: Unit = C.div(V);

    /// Ohm
    pub const Ω: Unit = V.div(A);

    /// Ohm
    pub const Ohm: Unit = Ω;

    /// Siemens
    pub const S: Unit = Ω.div(A);

    /// Weber
    pub const Wb: Unit = V.div(Ω);

    /// Tesla
    pub const T: Unit = Wb.div(m.pow(2));

    /// Henry
    pub const H: Unit = Wb.div(A);

    /// Lumen
    pub const lm: Unit = cd.mul(sr);

    /// Lux
    pub const lx: Unit = lm.div(m.pow(2));

    /// Becquerel
    pub const Bq: Unit = Null.div(s);

    /// Gray
    pub const Gy: Unit = J.div(kg);

    /// Sievert
    pub const Sv: Unit = J.div(kg);

    /// Katal
    pub const kat: Unit = mol.div(s);

    /// Volt-Ampere Reactive
    pub const VA: Unit = V.mul(A);

    /// Minute
    pub const min: Scalar = Scalar(60.0, s);

    /// Hour
    pub const hr: Scalar = Scalar(3600.0, s);

    /// Day
    pub const d: Scalar = Scalar(86400.0, s);

    /// Astronomical Unit
    pub const au: Scalar = Scalar(149597870700.0, m);

    /// Degree
    pub const deg: Scalar = Scalar(PI / 180.0, rad);

    /// Arcminute
    pub const arcmin: Scalar = Scalar(PI / 1080.0, rad);

    /// Arcsecond
    pub const arcsec: Scalar = Scalar(PI / 6480.0, rad);

    /// Hectare
    pub const ha: Scalar = Scalar(10000.0, m.pow(2));

    /// Litre
    pub const L: Scalar = Scalar(0.001, m.pow(3));

    /// Tonne
    pub const t: Scalar = Scalar(1000.0, kg);

    /// Dalton
    pub const Da: Scalar = Scalar(1.66053906660e-27, kg);

    /// Galileo
    pub const Gal: Scalar = Scalar(0.01, m.div(s.pow(2)));

    /// Unified Atomic Mass Unit
    pub const u: Scalar = Da;

    /// Parsec
    pub const pc: Scalar = Scalar(3.0856775814913673e16, m);

    /// Bar
    pub const bar: Scalar = Scalar(100000.0, Pa);

    /// Standard Atmosphere
    pub const atm: Scalar = Scalar(101325.0, Pa);

    /// ångström
    pub const Å: Scalar = Scalar(1e-10, m);

    // Imperial Units -------------------------------------------------------------------

    /// twip
    pub const twip: Scalar = Scalar(0.0000176389, m);

    /// thou
    pub const th: Scalar = Scalar(0.0000254, m);

    /// barleycorn
    pub const barleycorn: Scalar = Scalar(0.0084667, m);

    /// inch
    pub const inch: Scalar = Scalar(0.0254, m);

    /// hand
    pub const hh: Scalar = Scalar(0.1016, m);

    /// foot
    pub const ft: Scalar = Scalar(0.3048, m);

    /// yard
    pub const yd: Scalar = Scalar(0.9144, m);

    /// chain
    pub const ch: Scalar = Scalar(20.1168, m);

    /// furlong
    pub const fur: Scalar = Scalar(201.168, m);

    /// mile
    pub const mi: Scalar = Scalar(1609.344, m);

    /// league
    pub const lea: Scalar = Scalar(4828.032, m);

    /// fanthom
    pub const ftm: Scalar = Scalar(1.852, m);

    /// cable
    pub const calbe: Scalar = Scalar(185.2, m);

    /// nautical mile
    pub const nmi: Scalar = Scalar(1852.0, m);

    /// link
    pub const link: Scalar = Scalar(0.201168, m);

    /// rod
    pub const rod: Scalar = Scalar(5.0292, m);

    /// perch
    pub const perch: Scalar = Scalar(25.29285264, m.pow(2));

    /// rood
    pub const rood: Scalar = Scalar(1011.7141056, m.pow(2));

    /// acre
    pub const acre: Scalar = Scalar(4046.8564224, m.pow(2));

    /// square mile
    pub const sq_mi: Scalar = Scalar(2589988.110336, m.pow(2));

    /// fluid ounce
    pub const fl_oz: Scalar = Scalar(28.4130625e-6, m.pow(3));

    /// gill
    pub const gi: Scalar = Scalar(142.0653125e-6, m.pow(3));

    /// pint
    pub const pt: Scalar = Scalar(568.26125e-6, m.pow(3));

    /// quart
    pub const qt: Scalar = Scalar(1136.5225e-6, m.pow(3));

    /// gallon
    pub const gal: Scalar = Scalar(4546.09e-6, m.pow(3));

    /// grain
    pub const gr: Scalar = Scalar(64.79891e-6, kg);

    /// drachm
    pub const dr: Scalar = Scalar(1.7718451953125e-3, kg);

    /// ounce
    pub const oz: Scalar = Scalar(28.349523125e-3, kg);

    /// pound
    pub const lb: Scalar = Scalar(0.45359237, kg);

    /// stone
    pub const st: Scalar = Scalar(6.35029318, kg);

    /// qaurter
    pub const qtr: Scalar = Scalar(12.70058636, kg);

    /// hundredweight
    pub const cwt: Scalar = Scalar(50.80234544, kg);

    /// ton
    pub const ton: Scalar = Scalar(1016.0469088, kg);

    /// slug
    pub const slug: Scalar = Scalar(14.59390294, kg);

    // ----------------------------------------------------------------------------------

    pub const of_velocity: Unit = m.div(s);
    pub const of_acceleration: Unit = m.div(s.pow(2));
    pub const of_jerk: Unit = m.div(s.pow(3));
    pub const of_snap: Unit = m.div(s.pow(4));
    pub const of_angular_velocity: Unit = rad.div(s);
    pub const of_angular_acceleration: Unit = rad.div(s.pow(2));
    pub const of_frequency_drift: Unit = Hz.div(s);
    pub const of_volumetric_flow: Unit = m.pow(3).div(s);

    pub const of_area: Unit = m.pow(2);
    pub const of_volume: Unit = m.pow(3);
    pub const of_momentum: Unit = N.mul(s);
    pub const of_angular_momentum: Unit = N.mul(m).mul(s);
    pub const of_torque: Unit = N.mul(m);
    pub const of_yank: Unit = N.div(s);
    pub const reciprocal_meter: Unit = m.pow(-1);
    pub const of_area_density: Unit = kg.div(m.pow(2));
    pub const of_density: Unit = kg.div(m.pow(3));
    pub const of_specific_volume: Unit = m.pow(3).div(kg);
    pub const of_action: Unit = J.mul(s);
    pub const of_specific_energy: Unit = J.div(kg);
    pub const of_energy_density: Unit = J.div(m.pow(3));
    pub const of_surface_tension: Unit = N.div(m);
    pub const of_irradiance: Unit = W.div(m.pow(2));
    pub const of_kinematic_viscosity: Unit = m.pow(2).div(s);
    pub const of_dynamic_viscosity: Unit = Pa.mul(s);
    pub const of_linear_mass_density: Unit = kg.div(m);
    pub const of_mass_flow_rate: Unit = kg.div(s);
    pub const of_radiance: Unit = W.div(sr).div(m.pow(2));
    pub const of_spectral_radiance: Unit = W.div(sr).div(m.pow(3));
    pub const of_spectral_power: Unit = W.div(m);
    pub const of_absorbed_dose_rate: Unit = Gy.div(s);
    pub const of_fuel_efficiency: Unit = m.div(m.pow(3));
    pub const of_spectral_irradiance: Unit = W.div(m.pow(3));
    pub const of_energy_flux_density: Unit = J.div(m.pow(2)).div(s);
    pub const of_moment_of_inertia: Unit = kg.mul(m.pow(2));
    pub const of_specific_angular_momentum: Unit = N.mul(m).mul(s).div(kg);
    pub const of_radiant_intensity: Unit = W.div(sr);
    pub const of_spectral_intensity: Unit = W.div(sr).div(m);

    pub const of_molarity: Unit = mol.div(m.pow(3));
    pub const molar_volume: Unit = m.pow(3).div(mol);
    pub const of_molar_heat_capacity: Unit = J.div(mol).div(K);
    pub const of_molar_energy: Unit = J.div(mol);
    pub const of_molar_conductivity: Unit = S.div(m.pow(2)).div(mol);
    pub const of_molality: Unit = mol.div(kg);
    pub const of_molar_mass: Unit = kg.div(mol);
    pub const of_catalystic_efficiency: Unit = m.pow(3).div(mol).div(s);
    pub const reciprocal_mole: Unit = mol.pow(-1);

    pub const of_luminous_energy: Unit = lm.mul(s);
    pub const of_luminous_exposure: Unit = lx.mul(s);
    pub const of_luminance: Unit = cd.div(m.pow(2));
    pub const of_luminous_efficacy: Unit = lm.div(W);

    pub const of_heat_capacitance: Unit = J.div(K);
    pub const of_specific_heat_capacitance: Unit = J.div(kg).div(K);
    pub const of_thermal_conductivity: Unit = W.div(m).div(K);
    pub const of_thermal_resistance: Unit = K.div(W);
    pub const reciprocal_kelvin: Unit = K.pow(-1);
    pub const of_temperature_gradient: Unit = K.div(m);

    pub const of_polarization_density: Unit = C.div(m.pow(2));
    pub const of_electric_charge_density: Unit = C.div(m.pow(3));
    pub const of_electric_current_density: Unit = A.div(m.pow(2));
    pub const of_electrical_conductivity: Unit = S.div(m);
    pub const of_permittivity: Unit = F.div(m);
    pub const of_magnetic_permeability: Unit = H.div(m);
    pub const of_electric_field_strength: Unit = V.div(m);
    pub const of_magnetization: Unit = A.div(m);
    pub const of_exposure: Unit = C.div(kg);
    pub const of_resistivity: Unit = Ω.mul(m);
    pub const of_linear_charge_density: Unit = C.div(m);
    pub const of_magnetic_dipole_moment: Unit = J.div(T);
    pub const of_electron_mobility: Unit = m.pow(2).div(V).div(s);
    pub const reciprocal_henry: Unit = H.pow(-1);
    pub const of_magnetic_vector_potential: Unit = Wb.div(m);
    pub const of_magnetic_moment: Unit = Wb.mul(m);
    pub const of_magnetic_rigidity: Unit = T.mul(m);
    pub const of_magnetomotive_force: Unit = A.mul(rad);
    pub const of_magnetic_susceptibility: Unit = m.div(H);
}

pub mod constants {
    use super::units::*;
    use crate::{unit::SIPrefix, Scalar};
    pub use std::f64::consts::PI;

    // SI-UNITS-----------------------------------------------------------------

    /// speed of light in vacuum
    // pub const c: Scalar = Scalar(299792458.0, m.div(s));
    pub const c: Scalar = Scalar(1000.0, m.div(s));

    /// Planck constant
    pub const h: Scalar = Scalar(6.62607015e-34, J.mul(s));

    /// Hyperfine transition frequency of 133Cs
    pub const ΔνCs: Scalar = Scalar(9192631770.0, Hz);

    /// Elementary charge
    pub const e: Scalar = Scalar(1.602176634e-19, C);

    /// Boltzmann constant
    pub const k_B: Scalar = Scalar(1.380649e-23, J.div(K));

    /// Avogadro constant
    pub const N_A: Scalar = Scalar(6.02214076e23, mol.pow(-1));

    /// Luminous efficacy of 540 THz monochromatic radiation
    pub const K_cd: Scalar = Scalar(683.0, lm.div(W));

    // --------------------------------------------------------------------------

    /// Newtonian constant of gravitation
    pub const G: Scalar = Scalar(6.6743e-11, m.pow(3).div(kg).div(s.pow(2)));

    /// Fine-structure constant
    pub const α: Scalar = Scalar(0.0072973525693, Null);

    /// Wien wavelength displacement law constant
    pub const b: Scalar = Scalar(2.897771955e-3, m.mul(K));

    /// Wien frequency displacement law constant
    pub const b_freq: Scalar = Scalar(5.878925757e10, Hz.div(K));

    /// Wien entropy displacement law constant
    pub const b_entropy: Scalar = Scalar(3.002916077e-3, m.mul(K));

    /// Electron mass
    pub const m_e: Scalar = Scalar(9.1093837015e-31, kg);

    /// Proton mass
    pub const m_p: Scalar = Scalar(1.67262192369e-27, kg);

    /// Neutron mass
    pub const m_n: Scalar = Scalar(1.67492749804e-27, kg);

    /// Muon mass
    pub const m_μ: Scalar = Scalar(1.883531627e-28, kg);

    /// Tau mass
    pub const m_τ: Scalar = Scalar(3.16754e-27, kg);

    /// Top quark mass
    pub const m_t: Scalar = Scalar(3.0784e-25, kg);

    /// W to Z mass ratio
    pub const m_W_ratio_m_Z: Scalar = Scalar(0.88153, Null);

    /// Proton g-factor
    pub const g_p: Scalar = Scalar(5.5856946893, Null);

    /// Electron g-factor
    pub const g_e: Scalar = Scalar(-2.00231930436256, Null);

    /// Muon g-factor
    pub const g_μ: Scalar = Scalar(-2.0023318418, Null);

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
        pub const θ_W: Scalar = m_W_ratio_m_Z.acos() * Null;

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
pub const g: Vector<3> = Vector([0.0, 9.80665, 0.0], m.div(s.pow(2)));
