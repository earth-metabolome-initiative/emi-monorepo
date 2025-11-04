//! Submodule implementing the `molar_mass` method for the `MolecularFormula`
//! struct

use elements_rs::ElementVariant;

use super::MolecularFormula;

impl MolecularFormula {
    /// Returns the molar mass of the molecular formula.
    ///
    /// # Errors
    ///
    /// * If the formula is a `Residual`, an error is returned.
    pub fn molar_mass(&self) -> Result<f64, crate::errors::Error> {
        match self {
            Self::Element(element) => Ok(element.standard_atomic_weight()),
            Self::Isotope(isotope) => Ok(isotope.element().standard_atomic_weight()),
            Self::Ion(ion) => {
                ion.entry.molar_mass().map(|molar_mass| {
                    molar_mass - f64::from(ion.charge) * crate::ion::ELECTRON_MASS
                })
            }
            Self::Count(formula, count) => {
                formula.molar_mass().map(|molar_mass| molar_mass * f64::from(*count))
            }
            Self::Mixture(formulas) | Self::Sequence(formulas) => {
                formulas.iter().map(Self::molar_mass).sum()
            }
            Self::RepeatingUnit(formula) | Self::Complex(formula) | Self::Radical(formula, _) => {
                formula.molar_mass()
            }
            Self::Greek(_) => Ok(0.0),
            Self::Residual => Err(crate::errors::Error::InvalidOperationForResidual),
        }
    }
}
