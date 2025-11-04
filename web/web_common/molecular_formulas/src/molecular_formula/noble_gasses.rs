//! Submodule providing methods to work with molecular formulas containing noble
//! gasses.

use elements_rs::BondsNumber;

impl crate::MolecularFormula {
    /// Returns whether the formula solely contains noble gasses.
    ///
    /// # Errors
    ///
    /// * If the formula contains a non-element, such as a `Residual`.
    pub fn is_noble_gas_compound(&self) -> Result<bool, crate::errors::Error> {
        Ok(match self {
            Self::Residual => return Err(crate::errors::Error::InvalidOperationForResidual),
            Self::Element(element) => element.is_noble_gas(),
            Self::Isotope(isotope) => isotope.is_noble_gas(),
            Self::Ion(ion) => ion.entry.is_noble_gas_compound()?,
            Self::Count(formula, _) => formula.is_noble_gas_compound()?,
            Self::Complex(formula) | Self::RepeatingUnit(formula) | Self::Radical(formula, _) => {
                formula.is_noble_gas_compound()?
            }
            Self::Greek(_) => {
                unreachable!("Greek letters should not be checked for noble gas compounds")
            }
            Self::Sequence(formulas) | Self::Mixture(formulas) => {
                for formula in formulas {
                    if matches!(formula, Self::Greek(_)) {
                        continue;
                    }
                    if !formula.is_noble_gas_compound()? {
                        return Ok(false);
                    }
                }
                true
            }
        })
    }
}
