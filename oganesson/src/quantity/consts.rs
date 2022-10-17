#![allow(non_upper_case_globals)]

pub mod units {
    use crate::unit::Unit;

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

    /// None
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

    /// Degree Celsius
    pub const Celsius: Unit = K;

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

#[allow(clippy::excessive_precision)]
pub mod constants {
    use super::units::*;
    use crate::Scalar;

    /// Newtonian constant of gravitation
    pub const G: Scalar = Scalar(6.6743e-11, m.pow(3).div(kg).div(s.pow(2)));

    /// speed of light in vacuum
    pub const c: Scalar = Scalar(299792458.0, m.div(s));

    /// speed of light in vacuum squared
    pub const c2: Scalar = Scalar(89875517873681764.0, m.div(s));

    /// Planck constant
    pub const h: Scalar = Scalar(6.62607015e-34, J.div(Hz));

    /// reduced Planck constant
    pub const h_bar: Scalar = Scalar(1.054571817e-34, J.mul(s));

    /// Vacuum magnetic permeability
    pub const μ_0: Scalar = Scalar(1.25663706212e-6, N.div(A.pow(2)));

    /// Vacuum magnetic permeability
    pub const MagneticPermiability: Scalar = μ_0;

    /// Characteristic impedance of vacuum
    pub const Z_0: Scalar = Scalar(376.730313668, Ω);

    /// Vacuum electric permittivity
    pub const ε_0: Scalar = Scalar(8.8541878128e-12, C.div(V).div(m));

    /// Vacuum electric permittivity
    pub const ElectricPermittivity: Scalar = ε_0;

    /// Coulomb constant
    pub const k_e: Scalar = Scalar(8.9875517923e9, N.mul(m.div(C).pow(2)));

    /// Boltzmann constant
    pub const k_B: Scalar = Scalar(1.380649e-23, J.div(K));

    /// Stefan–Boltzmann constant
    pub const σ: Scalar = Scalar(5.670374419e-8, W.div(m.pow(2)).div(K.pow(4)));

    /// Stefan–Boltzmann constant
    pub const StefanBoltzmannConst: Scalar = σ;

    /// First radiation constant
    pub const c_1: Scalar = Scalar(33.741771852e-16, W.div(m.pow(2)));

    /// First radiation constant for spectral radiance
    pub const c_1L: Scalar = Scalar(1.1910429723971884e-16, W.mul(m.pow(2).div(sr)));

    /// Second radiation constant
    pub const c_2: Scalar = Scalar(1.438776877e-2, m.mul(K));

    /// Wien wavelength displacement law constant
    pub const b: Scalar = Scalar(2.897771955e-3, m.mul(K));

    /// Wien frequency displacement law constant
    pub const b_freq: Scalar = Scalar(5.878925757e10, Hz.div(K));

    /// Wien entropy displacement law constant
    pub const b_entropy: Scalar = Scalar(3.002916077e-3, m.mul(K));

    /// Elementary charge
    pub const e: Scalar = Scalar(1.602176634e-19, C);

    /// Conductance quantum
    pub const G_0: Scalar = Scalar(7.748091729e-5, S);

    /// Inverse conductance quantum
    pub const G_0_inv: Scalar = Scalar(12906.40372, Ω);

    /// Von Klitzing constant
    pub const R_K: Scalar = Scalar(25812.80745, Ω);

    /// Josephson constant
    pub const K_J: Scalar = Scalar(483597.8484e9, Hz.div(V));

    /// Fine-structure constant
    pub const α: Scalar = Scalar(7.2973525693e-3, Null);

    /// Fine-structure constant
    pub const FineStructureConst: Scalar = α;

    /// Inverse fine-structure constant
    pub const α_inv: Scalar = Scalar(137.035999084, Null);

    /// Inverse fine-structure constant
    pub const FineStructureConstInv: Scalar = α_inv;

    /// Electron mass
    pub const m_e: Scalar = Scalar(9.1093837015e-31, kg);

    /// Proton mass
    pub const m_p: Scalar = Scalar(1.67262192369e-27, kg);

    /// Neutron mass
    pub const m_n: Scalar = Scalar(1.67492749804e-27, kg);

    /// Muon mass
    pub const m_μ: Scalar = Scalar(1.883531627e-28, kg);

    /// Muon mass
    pub const MuonMass: Scalar = m_μ;

    /// Tau mass
    pub const m_τ: Scalar = Scalar(3.16754e-27, kg);

    /// Tau mass
    pub const TauMass: Scalar = m_τ;

    /// Top quark mass
    pub const m_t: Scalar = Scalar(3.0784e-25, kg);

    /// Proton to electron mass ratio
    pub const m_p_ratio_m_e: Scalar = Scalar(1836.15267343, Null);

    /// W to Z mass ratio
    pub const m_W_ratio_m_Z: Scalar = Scalar(0.88153, kg);

    /// Weak mixing angle
    pub const sin2_θ_W: Scalar = Scalar(0.22290, Null);

    /// Weak mixing angle
    pub const WeakMixingAngle: Scalar = sin2_θ_W;

    /// Electron g-factor
    pub const g_e: Scalar = Scalar(-2.00231930436256, Null);

    /// Muon g-factor
    pub const g_μ: Scalar = Scalar(-2.0023318418, Null);

    /// Muon g-factor
    pub const MuonGFactor: Scalar = g_μ;

    /// Proton g-factor
    pub const g_p: Scalar = Scalar(5.5856946893, Null);

    /// Quantum of circulation
    pub const quantum_of_circulation: Scalar = Scalar(3.6369475516e-4, m.pow(2).div(s));

    /// Bohr magneton
    pub const μ_B: Scalar = Scalar(9.2740100783e-24, J.div(T));

    /// Nuclear magneton
    pub const μ_N: Scalar = Scalar(5.0507837461e-27, J.div(T));

    /// Classical electron radius
    pub const r_e: Scalar = Scalar(2.8179403262e-15, m);

    /// Thomson cross section
    pub const σ_e: Scalar = Scalar(6.6524587321e-29, m.pow(2));

    /// Thomson cross section
    pub const ThomsonCrossSection: Scalar = σ_e;

    /// Bohr radius
    pub const a_0: Scalar = Scalar(5.29177210903e-11, m);

    /// Hartree energy
    pub const E_h: Scalar = Scalar(4.3597447222071e-18, J);

    /// Rydberg unit of energy
    pub const R_y: Scalar = Scalar(10973731.568160, m.pow(-1));

    /// Rydberg constant
    pub const R_H: Scalar = Scalar(10973731.568160, m.pow(-1));

    /// Fermi coupling constant (1.1663787e-5 GeV)
    pub const fermi_coupling_constant: Scalar = Scalar(2.200805022e-20, J);

    /// Avogadro constant
    pub const N_A: Scalar = Scalar(6.02214076e23, mol.pow(-1));

    /// Molar gas constant
    pub const R: Scalar = Scalar(8.31446261815324, J.div(mol).div(K));

    /// Faraday constant
    pub const F: Scalar = Scalar(96485.33212331002, C.div(mol));

    /// Molar Planck constant
    pub const N_A_h: Scalar = Scalar(3.9903127128934314e-10, J.mul(s).div(mol));

    /// Atomic mass of carbon-12
    pub const m_12C: Scalar = Scalar(1.99264687992e-26, kg);

    /// Molar mass of carbon-12
    pub const M_12C: Scalar = Scalar(11.9999999958e-3, kg.div(mol));

    /// Atomic mass constant
    pub const m_u: Scalar = Scalar(1.66053906660e-27, kg);

    /// Molar mass constant
    pub const M_u: Scalar = Scalar(0.99999999965e-3, kg);

    /// Molar volume of Silicon
    pub const V_m_Si: Scalar = Scalar(1.205883199e-5, m.pow(3).div(mol));

    /// Hyperfine transition frequency of 133Cs
    pub const ΔνCs: Scalar = Scalar(9192631770.0, Hz);

    /// Hyperfine transition frequency of 133Cs
    pub const TransitionFrequency133Cs: Scalar = ΔνCs;

    /// Cosmological sonstant
    pub const Λ: Scalar = Scalar(2.036e-35, s.pow(-2));

    /// Cosmological sonstant
    pub const CosmologicalConstant: Scalar = Λ;

    /// Einstein gravitational constant
    pub const κ: Scalar = Scalar(2.076579e-43, N.recip());

    /// Einstein gravitational constant
    pub const EinsteinGravitationalConstant: Scalar = κ;
}

use crate::Vector;
use units::{m, s};

/// standard gravitational acceleration for the surface of the Earth
pub const g: Vector<3> = Vector([0.0, 9.80665, 0.0], m.div(s.pow(2)));
