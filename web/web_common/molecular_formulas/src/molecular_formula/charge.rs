use super::MolecularFormula;

impl MolecularFormula {
    /// Returns the overall charge of the molecular formula.
    /// The charge is calculated by summing the charges of all components.
    ///
    /// # Errors
    ///
    /// * If the `MolecularFormula` contains Residual.
    pub fn charge(&self) -> Result<i16, crate::errors::Error> {
        Ok(match self {
            Self::Ion(ion) => ion.charge,
            Self::Element(_) | Self::Isotope(_) | Self::Greek(_) => 0,
            Self::Count(formula, count) => {
                formula.charge()?
                    * i16::try_from(*count).map_err(|_| crate::errors::Error::InvalidNumber)?
            }
            Self::Sequence(formulas) | Self::Mixture(formulas) => {
                let mut charge = 0;
                for formula in formulas {
                    charge += formula.charge()?;
                }
                charge
            }
            Self::Radical(formula, _) | Self::RepeatingUnit(formula) | Self::Complex(formula) => {
                formula.charge()?
            }
            Self::Residual => return Err(crate::errors::Error::InvalidOperationForResidual),
        })
    }
}
