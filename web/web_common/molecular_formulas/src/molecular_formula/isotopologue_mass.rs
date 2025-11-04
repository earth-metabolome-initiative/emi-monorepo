//! Submodule implementing the `isotopologue_mass_with_charge` and
//! `isotopologue_mass_without_charge` methods for the `MolecularFormula` struct

use elements_rs::RelativeAtomicMass;

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
            Self::Element(element) => Ok(element.relative_atomic_mass()),
            Self::Isotope(isotope) => Ok(isotope.relative_atomic_mass()),
            Self::Ion(ion) => {
                ion.entry.isotopologue_mass_with_charge().map(|isotopologue_mass_with_charge| {
                    isotopologue_mass_with_charge
                        - f64::from(ion.charge) * crate::ion::ELECTRON_MASS
                })
            }
            Self::Count(formula, count) => {
                formula.isotopologue_mass_with_charge().map(|isotopologue_mass_with_charge| {
                    isotopologue_mass_with_charge * f64::from(*count)
                })
            }
            Self::Mixture(formulas) | Self::Sequence(formulas) => {
                formulas.iter().map(Self::isotopologue_mass_with_charge).sum()
            }
            Self::RepeatingUnit(formula) | Self::Complex(formula) | Self::Radical(formula, _) => {
                formula.isotopologue_mass_with_charge()
            }
            Self::Greek(_) => Ok(0.0),
            Self::Residual => Err(crate::errors::Error::InvalidOperationForResidual),
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
            Self::Element(element) => Ok(element.relative_atomic_mass()),
            Self::Isotope(isotope) => Ok(isotope.relative_atomic_mass()),
            Self::Ion(ion) => ion.entry.isotopologue_mass_without_charge(),
            Self::Count(formula, count) => {
                formula.isotopologue_mass_without_charge().map(|isotopologue_mass_without_charge| {
                    isotopologue_mass_without_charge * f64::from(*count)
                })
            }
            Self::Mixture(formulas) | Self::Sequence(formulas) => {
                formulas.iter().map(Self::isotopologue_mass_without_charge).sum()
            }
            Self::RepeatingUnit(formula) | Self::Complex(formula) | Self::Radical(formula, _) => {
                formula.isotopologue_mass_without_charge()
            }
            Self::Greek(_) => Ok(0.0),
            Self::Residual => Err(crate::errors::Error::InvalidOperationForResidual),
        }
    }
}
