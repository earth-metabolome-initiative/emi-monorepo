//! Submodule implementing the `monoisotopic_mass` method for the
//! `MolecularFormula` struct

use elements::RelativeAtomicMass;

use super::MolecularFormula;

impl MolecularFormula {
    /// Returns the exact mass of the molecular formula.
    pub fn monoisotopic_mass(&self) -> Option<f64> {
        match self {
            MolecularFormula::Element(element) => Some(element.relative_atomic_mass()),
            MolecularFormula::Ion(ion) => {
                ion.entry.monoisotopic_mass().map(|monoisotopic_mass| {
                    monoisotopic_mass - f64::from(ion.charge) * crate::ion::ELECTRON_MASS
                })
            }
            MolecularFormula::Count(formula, count) => {
                formula
                    .monoisotopic_mass()
                    .map(|monoisotopic_mass| monoisotopic_mass * f64::from(*count))
            }
            MolecularFormula::Mixture(formulas) | MolecularFormula::Sequence(formulas) => {
                formulas.iter().map(MolecularFormula::monoisotopic_mass).sum()
            }
            MolecularFormula::Complex(formula) => formula.monoisotopic_mass(),
            MolecularFormula::RepeatingUnit(formula) => formula.monoisotopic_mass(),
            MolecularFormula::Residual => None,
        }
    }
}
