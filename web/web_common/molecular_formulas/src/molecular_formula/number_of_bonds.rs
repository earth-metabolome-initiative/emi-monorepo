//! Submodule implementing the unmatched electrons calculation for molecular
//! formulas.

use elements_rs::BondsNumber;

use crate::errors::Error;

impl crate::MolecularFormula {
    /// Returns the minimum and maximum number of bonds which the formula can
    /// form.
    ///
    /// # Errors
    ///
    /// * If the formula contains a non-element, such as a `Residual`.
    /// * If the formula contains a `Mixture`, the unmatched electrons of the
    ///   mixture are not defined and as such an error is returned.
    pub fn number_of_bonds(&self) -> Result<(i16, i16), Error> {
        Ok(match self {
            Self::Element(element) => {
                let (min, max) = element.number_of_bonds();
                (min.into(), max.into())
            }
            Self::Isotope(isotope) => {
                let (min, max) = isotope.number_of_bonds();
                (min.into(), max.into())
            }
            Self::Ion(ion) => {
                let (min, max) = ion.entry.number_of_bonds()?;
                (min - ion.charge, max - ion.charge)
            }
            Self::Count(formula, count) => {
                let (min, max) = formula.number_of_bonds()?;
                if count % 2 == 0 {
                    (0, max * i16::try_from(*count).map_err(|_| Error::InvalidNumber)?)
                } else {
                    (min, max * i16::try_from(*count).map_err(|_| Error::InvalidNumber)?)
                }
            }
            Self::Greek(_) => {
                unreachable!("Greek formulas do not have a defined number of bonds")
            }
            Self::Complex(formula) | Self::RepeatingUnit(formula) | Self::Radical(formula, _) => {
                formula.number_of_bonds()?
            }
            Self::Sequence(formulas) => {
                let mut total_minimum_number_of_bonds = 0;
                let mut total_maximum_number_of_bonds = 0;

                for formula in formulas {
                    if matches!(formula, Self::Greek(_)) {
                        continue;
                    }

                    let (min, max) = formula.number_of_bonds()?;
                    total_minimum_number_of_bonds += min;
                    total_maximum_number_of_bonds += max;
                }
                (total_minimum_number_of_bonds, total_maximum_number_of_bonds)
            }
            Self::Mixture(_) => {
                return Err(crate::errors::Error::InvalidOperationForMixture);
            }
            Self::Residual => {
                return Err(crate::errors::Error::InvalidOperationForResidual);
            }
        })
    }
}
