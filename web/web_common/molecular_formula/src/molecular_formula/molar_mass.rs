//! Submodule implementing the `molar_mass` method for the `MolecularFormula`
//! struct

use elements::ElementVariant;

use super::MolecularFormula;

impl MolecularFormula {
    /// Returns the molar mass of the molecular formula.
    pub fn molar_mass(&self) -> Option<f64> {
        match self {
            MolecularFormula::Element(element) => Some(element.standard_atomic_weight()),
            MolecularFormula::Isotope(isotope) => Some(isotope.element().standard_atomic_weight()),
            MolecularFormula::Ion(ion) => {
                ion.entry.molar_mass().map(|molar_mass| {
                    molar_mass - f64::from(ion.charge) * crate::ion::ELECTRON_MASS
                })
            }
            MolecularFormula::Count(formula, count) => {
                formula.molar_mass().map(|molar_mass| molar_mass * f64::from(*count))
            }
            MolecularFormula::Mixture(formulas) | MolecularFormula::Sequence(formulas) => {
                formulas.iter().map(MolecularFormula::molar_mass).sum()
            }
            MolecularFormula::RepeatingUnit(formula) | MolecularFormula::Complex(formula) => {
                formula.molar_mass()
            }
            MolecularFormula::Residual => None,
        }
    }
}
