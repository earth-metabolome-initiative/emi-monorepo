//! Submodule providing a validation for oxidation states of the
//! `MolecularFormula` struct.

use std::collections::HashSet;

use elements::ElementVariant;

impl super::MolecularFormula {
    /// Returns the viable oxidation states of the molecular formula.
    ///
    /// # Errors
    ///
    /// * If the `MolecularFormula` contains Residual, an error is returned as
    ///   the oxidation states cannot be determined.
    pub fn oxidation_states(&self) -> Result<Vec<i16>, crate::errors::Error> {
        let mut oxidation_states: HashSet<i16> = HashSet::new();
        oxidation_states.insert(0);

        match self {
            Self::Element(element) => {
                oxidation_states.extend(element.oxidation_states().iter().copied());
            }
            Self::Isotope(isotope) => {
                oxidation_states.extend(isotope.element().oxidation_states().iter().copied());
            }
            Self::Complex(formula) | Self::RepeatingUnit(formula) => {
                let formula_oxidation_states = formula.oxidation_states()?;
                for molecular_oxidation_state in oxidation_states.clone() {
                    for oxidation_state in &formula_oxidation_states {
                        oxidation_states.insert(molecular_oxidation_state + oxidation_state);
                    }
                }
            }
            Self::Count(formula, count) => {
                let formula_oxidation_states = formula.oxidation_states()?;
                for _ in 0..*count {
                    for molecular_oxidation_state in oxidation_states.clone() {
                        for oxidation_state in &formula_oxidation_states {
                            oxidation_states.insert(molecular_oxidation_state + oxidation_state);
                        }
                    }
                }
            }
            Self::Ion(ion) => {
                oxidation_states = HashSet::from_iter(vec![ion.charge]);
            }
            Self::Mixture(formulas) | Self::Sequence(formulas) => {
                for formula in formulas {
                    let formula_oxidation_states = formula.oxidation_states()?;
                    for molecular_oxidation_state in oxidation_states.clone() {
                        for oxidation_state in &formula_oxidation_states {
                            oxidation_states.insert(molecular_oxidation_state + oxidation_state);
                        }
                    }
                }
            }
            Self::Residual => {
                return Err(crate::errors::Error::InvalidOperationForResidual);
            }
        }

        let mut oxidation_states: Vec<i16> = oxidation_states.into_iter().collect();
        oxidation_states.sort_unstable();
        Ok(oxidation_states)
    }

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
            Self::Element(element) => Ok(element.oxidation_states().contains(&oxidation_state)),
            Self::Isotope(isotope) => {
                Ok(isotope.element().oxidation_states().contains(&oxidation_state))
            }
            Self::Complex(formula) | Self::RepeatingUnit(formula) => {
                formula.is_valid_oxidation_state(oxidation_state)
            }
            Self::Count(formula, _) => {
                Ok(if !formula.is_valid_oxidation_state(oxidation_state)? {
                    self.oxidation_states()?.contains(&oxidation_state)
                } else {
                    true
                })
            }
            Self::Ion(ion) => Ok(ion.charge == oxidation_state),
            Self::Mixture(formulas) | Self::Sequence(formulas) => {
                Ok(
                    if !formulas.iter().any(|formula| {
                        formula.is_valid_oxidation_state(oxidation_state).unwrap_or(false)
                    }) {
                        self.oxidation_states()?.contains(&oxidation_state)
                    } else {
                        true
                    },
                )
            }
            Self::Residual => Err(crate::errors::Error::InvalidOperationForResidual),
        }
    }
}
