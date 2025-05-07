//! Submodule implementing the `MolarMass` trait for the `MolecularFormula`
//! struct

use elements::MolarMass;

use super::MolecularFormula;

impl MolarMass for MolecularFormula {
    fn molar_mass(&self) -> f64 {
        match self {
            MolecularFormula::Element(element) => element.molar_mass(),
            MolecularFormula::Ion(ion) => ion.molar_mass(),
            MolecularFormula::Solvation(solvation) => solvation.molar_mass(),
            MolecularFormula::Count(formula, count) => formula.molar_mass() * f64::from(*count),
            MolecularFormula::Sequence(formulas) => {
                formulas.iter().map(MolarMass::molar_mass).sum()
            }
            MolecularFormula::Complex(formula) => formula.molar_mass(),
            MolecularFormula::RepeatingUnit(formula) => formula.molar_mass(),
        }
    }
}
