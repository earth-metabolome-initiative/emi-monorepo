//! Submodule implementing the `isotopologue_mass_with_charge` and
//! `isotopologue_mass_without_charge` methods for the `MolecularFormula` struct

use elements::RelativeAtomicMass;

use super::MolecularFormula;

impl MolecularFormula {
    /// Returns the isotopologue mass of the molecular formula, including the
    /// charge.
    ///
    /// # Errors
    ///
    /// * If the `MolecularFormula` contains Residual.
    pub fn isotopologue_mass_with_charge(&self) -> Result<f64, crate::errors::Error> {
        match self {
            MolecularFormula::Element(element) => Ok(element.relative_atomic_mass()),
            MolecularFormula::Isotope(isotope) => Ok(isotope.relative_atomic_mass()),
            MolecularFormula::Ion(ion) => {
                ion.entry.isotopologue_mass_with_charge().map(|isotopologue_mass_with_charge| {
                    isotopologue_mass_with_charge
                        - f64::from(ion.charge) * crate::ion::ELECTRON_MASS
                })
            }
            MolecularFormula::Count(formula, count) => {
                formula.isotopologue_mass_with_charge().map(|isotopologue_mass_with_charge| {
                    isotopologue_mass_with_charge * f64::from(*count)
                })
            }
            MolecularFormula::Mixture(formulas) | MolecularFormula::Sequence(formulas) => {
                formulas.iter().map(MolecularFormula::isotopologue_mass_with_charge).sum()
            }
            MolecularFormula::RepeatingUnit(formula) | MolecularFormula::Complex(formula) => {
                formula.isotopologue_mass_with_charge()
            }
            MolecularFormula::Greek(_) => Ok(0.0),
            MolecularFormula::Residual => Err(crate::errors::Error::InvalidOperationForResidual),
        }
    }

    /// Returns the isotopologue mass of the molecular formula, excluding the
    /// charge.
    ///
    /// # Errors
    ///
    /// * If the `MolecularFormula` contains Residual.
    pub fn isotopologue_mass_without_charge(&self) -> Result<f64, crate::errors::Error> {
        match self {
            MolecularFormula::Element(element) => Ok(element.relative_atomic_mass()),
            MolecularFormula::Isotope(isotope) => Ok(isotope.relative_atomic_mass()),
            MolecularFormula::Ion(ion) => ion.entry.isotopologue_mass_without_charge(),
            MolecularFormula::Count(formula, count) => {
                formula.isotopologue_mass_without_charge().map(|isotopologue_mass_without_charge| {
                    isotopologue_mass_without_charge * f64::from(*count)
                })
            }
            MolecularFormula::Mixture(formulas) | MolecularFormula::Sequence(formulas) => {
                formulas.iter().map(MolecularFormula::isotopologue_mass_without_charge).sum()
            }
            MolecularFormula::RepeatingUnit(formula) | MolecularFormula::Complex(formula) => {
                formula.isotopologue_mass_without_charge()
            }
            MolecularFormula::Greek(_) => Ok(0.0),
            MolecularFormula::Residual => Err(crate::errors::Error::InvalidOperationForResidual),
        }
    }
}
