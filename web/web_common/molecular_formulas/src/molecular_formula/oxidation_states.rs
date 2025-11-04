//! Submodule providing a validation for oxidation states of the
//! `MolecularFormula` struct.

use elements_rs::ElementVariant;
use multi_ranged::{MultiRange, MultiRanged, Step};

impl super::MolecularFormula {
    #[inline]
    /// Returns the viable oxidation states of the molecular formula.
    ///
    /// # Errors
    ///
    /// * If the `MolecularFormula` contains Residual, an error is returned as
    ///   the oxidation states cannot be determined.
    pub fn oxidation_states(&self) -> Result<MultiRange<i16>, crate::errors::Error> {
        Ok(match self {
            Self::Element(element) => element.oxidation_states(),
            Self::Isotope(isotope) => isotope.element().oxidation_states(),
            Self::Complex(formula) | Self::RepeatingUnit(formula) | Self::Radical(formula, _) => {
                formula.oxidation_states()?
            }
            Self::Greek(_) => MultiRange::default(),
            Self::Count(formula, count) => {
                formula
                    .oxidation_states()?
                    .checked_mul(
                        i16::try_from(*count).map_err(|_| crate::errors::Error::InvalidNumber)?,
                    )
                    .ok_or(crate::errors::Error::InvalidNumber)?
            }
            Self::Ion(ion) => MultiRange::from(ion.charge),
            Self::Mixture(formulas) | Self::Sequence(formulas) => {
                let mut oxidation_states = formulas.first().unwrap().oxidation_states()?;
                for formula in &formulas[1..] {
                    let formula_oxidation_states = formula.oxidation_states()?;

                    if formula_oxidation_states.is_dense() && oxidation_states.is_dense() {
                        oxidation_states = MultiRange::try_from((
                            formula_oxidation_states.absolute_start().unwrap()
                                + oxidation_states.absolute_start().unwrap(),
                            formula_oxidation_states.absolute_end().unwrap().prev()
                                + oxidation_states.absolute_end().unwrap().prev()
                                + 1,
                        ))
                        .unwrap();
                        continue;
                    }

                    for molecular_oxidation_state in oxidation_states.clone() {
                        for oxidation_state in formula_oxidation_states.clone() {
                            let _ = oxidation_states
                                .insert(molecular_oxidation_state + oxidation_state);
                        }
                    }
                }
                oxidation_states
            }
            Self::Residual => {
                return Err(crate::errors::Error::InvalidOperationForResidual);
            }
        })
    }

    #[inline]
    /// Returns whether the provided oxidation state is valid for the molecular
    /// formula.
    ///
    /// # Errors
    ///
    /// * If the `MolecularFormula` contains Residual, an error is returned as
    ///   the oxidation states cannot be determined.
    pub fn is_valid_oxidation_state(
        &self,
        oxidation_state: i16,
    ) -> Result<bool, crate::errors::Error> {
        match self {
            Self::Element(element) => Ok(element.oxidation_states().contains(oxidation_state)),
            Self::Isotope(isotope) => {
                Ok(isotope.element().oxidation_states().contains(oxidation_state))
            }
            Self::Complex(formula) | Self::RepeatingUnit(formula) | Self::Radical(formula, _) => {
                formula.is_valid_oxidation_state(oxidation_state)
            }
            Self::Count(formula, _) => {
                Ok(if formula.is_valid_oxidation_state(oxidation_state)? {
                    true
                } else {
                    self.oxidation_states()?.contains(oxidation_state)
                })
            }
            Self::Greek(_) => Ok(true),
            Self::Ion(ion) => Ok(ion.charge == oxidation_state),
            Self::Mixture(formulas) | Self::Sequence(formulas) => {
                Ok(
                    if formulas.iter().any(|formula| {
                        formula.is_valid_oxidation_state(oxidation_state).unwrap_or(false)
                    }) {
                        true
                    } else {
                        self.oxidation_states()?.contains(oxidation_state)
                    },
                )
            }
            Self::Residual => Err(crate::errors::Error::InvalidOperationForResidual),
        }
    }
}
