//! Submodule implementing the `MolarMass` trait for the `MolecularFormula`
//! struct

use super::MolecularFormula;

impl MolecularFormula {
    /// Returns the molar mass of the molecular formula.
    pub fn molar_mass(&self) -> Option<f64> {
        match self {
            MolecularFormula::Element(element) => Some(element.standard_atomic_weight()),
            MolecularFormula::Ion(ion) => {
                ion.entry.molar_mass().map(|molar_mass| {
                    molar_mass - f64::from(ion.charge) * crate::ion::ELECTRON_MOLAR_MASS
                })
            }
            MolecularFormula::Solvation(solvation) => {
                match (solvation.solvate.molar_mass(), solvation.solvant.molar_mass()) {
                    (Some(solvate_molar_mass), Some(solvent_molar_mass)) => {
                        Some(solvate_molar_mass + solvent_molar_mass)
                    }
                    _ => None,
                }
            }
            MolecularFormula::Count(formula, count) => {
                formula.molar_mass().map(|molar_mass| molar_mass * f64::from(*count))
            }
            MolecularFormula::Sequence(formulas) => {
                formulas.iter().map(MolecularFormula::molar_mass).sum()
            }
            MolecularFormula::Complex(formula) => formula.molar_mass(),
            MolecularFormula::RepeatingUnit(formula) => formula.molar_mass(),
            MolecularFormula::Residual => None,
        }
    }
}
