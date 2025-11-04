//! Submodule providing the `is_diatomic` method for the `MolecularFormula`
//! struct, which returns whether the formula is composed solely of two
//! elements. This also returns true in case of homonuclear diatomics, such as
//! `H2`, `O2`, etc.

use elements_rs::{ElementVariant, ValenceElectrons};
mod homodiatomic;

impl crate::MolecularFormula {
    /// Returns the total number of non-unique elements in the formula.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let formula = MolecularFormula::from_str("H2O").unwrap();
    ///
    /// assert_eq!(formula.number_of_elements().unwrap(), 3);
    ///
    /// let formula = MolecularFormula::from_str("H2").unwrap();
    /// assert_eq!(formula.number_of_elements().unwrap(), 2);
    ///
    /// let formula = MolecularFormula::from_str("H2O2").unwrap();
    /// assert_eq!(formula.number_of_elements().unwrap(), 4);
    /// ```
    ///
    /// # Errors
    ///
    /// * If the formula contains a non-element, such as a `Residual`.
    pub fn number_of_elements(&self) -> Result<usize, crate::errors::Error> {
        Ok(match self {
            Self::Element(_) | Self::Isotope(_) => 1,
            Self::Ion(ion) => ion.entry.number_of_elements()?,
            Self::Count(formula, count) => formula.number_of_elements()? * (*count as usize),
            Self::Complex(formula) | Self::RepeatingUnit(formula) | Self::Radical(formula, _) => {
                formula.number_of_elements()?
            }
            Self::Mixture(formulas) | Self::Sequence(formulas) => {
                let mut total_number_of_elements = 0;
                for formula in formulas {
                    total_number_of_elements += formula.number_of_elements()?;
                }
                total_number_of_elements
            }
            Self::Greek(_) => 0,
            Self::Residual => {
                return Err(crate::errors::Error::InvalidOperationForResidual);
            }
        })
    }

    /// Returns whether the formula is diatomic, such as `H2`, `O2`, `N2`,
    /// `H2+`, `O2-`, etc. A mixture is considered diatomic if all of the
    /// sub-formulas are diatomic.
    ///
    /// # Errors
    ///
    /// * If the formula contains a non-element, such as a `Residual`.
    pub fn is_diatomic(&self) -> Result<bool, crate::errors::Error> {
        Ok(self.number_of_elements()? == 2)
    }

    fn inner_diatomic(
        &self,
        left: &mut Option<elements_rs::Element>,
        right: &mut Option<elements_rs::Element>,
    ) {
        match self {
            Self::Isotope(isotope) => {
                if left.is_none() {
                    *left = Some(isotope.element());
                } else if right.is_none() {
                    *right = Some(isotope.element());
                } else {
                    unreachable!()
                }
            }
            Self::Element(element) => {
                if left.is_none() {
                    *left = Some(*element);
                } else if right.is_none() {
                    *right = Some(*element);
                } else {
                    unreachable!()
                }
            }
            Self::Count(formula, count) => {
                if *count == 1 {
                    formula.inner_diatomic(left, right);
                } else if *count == 2 {
                    formula.inner_diatomic(left, &mut None);
                    *right = *left;
                } else {
                    unreachable!()
                }
            }
            Self::Greek(_) => {}
            Self::Ion(ion) => ion.entry.inner_diatomic(left, right),
            Self::Complex(formula) | Self::RepeatingUnit(formula) | Self::Radical(formula, _) => {
                formula.inner_diatomic(left, right);
            }
            Self::Sequence(formulas) => {
                for formula in formulas {
                    if left.is_none() || right.is_none() {
                        formula.inner_diatomic(left, right);
                    } else {
                        unreachable!()
                    }
                }
            }
            Self::Residual | Self::Mixture(_) => {
                unreachable!()
            }
        }
    }

    /// Returns the tuple of elements in the formula if it is diatomic.
    ///
    /// # Returns
    ///
    /// * An option containing a tuple of two elements if the formula is
    ///   diatomic, otherwise `None`.
    ///
    /// # Errors
    ///
    /// * If the formula contains a non-element, such as a `Residual`.
    /// * If the formula contains a `Mixture`, whether it is diatomic or not, it
    ///   is not well defined and as such an error is returned.
    pub fn diatomic(
        &self,
    ) -> Result<Option<(elements_rs::Element, elements_rs::Element)>, crate::errors::Error> {
        if !self.is_diatomic()? {
            return Ok(None);
        }
        Ok(match self {
            Self::Isotope(_) | Self::Element(_) | Self::Residual => {
                unreachable!()
            }
            Self::Mixture(_) => return Err(crate::errors::Error::InvalidOperationForMixture),
            _ => {
                let mut left = None;
                let mut right = None;
                self.inner_diatomic(&mut left, &mut right);
                Some((left.unwrap(), right.unwrap()))
            }
        })
    }

    fn inner_diatomic_valence_electrons(&self, left: &mut Option<i16>, right: &mut Option<i16>) {
        match self {
            Self::Isotope(isotope) => {
                if left.is_none() {
                    *left = Some(i16::from(isotope.element().valence_electrons()));
                } else if right.is_none() {
                    *right = Some(i16::from(isotope.element().valence_electrons()));
                } else {
                    unreachable!()
                }
            }
            Self::Element(element) => {
                if left.is_none() {
                    *left = Some(i16::from(element.valence_electrons()));
                } else if right.is_none() {
                    *right = Some(i16::from(element.valence_electrons()));
                } else {
                    unreachable!()
                }
            }
            Self::Count(formula, count) => {
                if *count == 1 {
                    formula.inner_diatomic_valence_electrons(left, right);
                } else if *count == 2 {
                    formula.inner_diatomic_valence_electrons(left, &mut None);
                    *right = *left;
                } else {
                    unreachable!()
                }
            }
            Self::Ion(ion) => {
                ion.entry.inner_diatomic_valence_electrons(left, right);
                if let Some(left) = left {
                    *left -= ion.charge;
                } else if let Some(right) = right {
                    *right -= ion.charge;
                } else {
                    unreachable!()
                }
            }
            Self::Complex(formula) | Self::RepeatingUnit(formula) | Self::Radical(formula, _) => {
                formula.inner_diatomic_valence_electrons(left, right);
            }
            Self::Sequence(formulas) => {
                for formula in formulas {
                    if left.is_none() || right.is_none() {
                        formula.inner_diatomic_valence_electrons(left, right);
                    } else {
                        unreachable!()
                    }
                }
            }
            Self::Greek(_) => {
                // Greek letters do not have valence electrons.
            }
            Self::Residual | Self::Mixture(_) => {
                unreachable!()
            }
        }
    }

    /// Returns the tuple of diatomic valence electrons in the formula if it is
    /// diatomic.
    ///
    /// # Returns
    ///
    /// * An option containing a tuple of two valence electrons if the formula
    ///   is diatomic, otherwise `None`.
    ///
    /// # Errors
    ///
    /// * If the formula contains a non-element, such as a `Residual`.
    /// * If the formula contains a `Mixture`, whether it is diatomic or not, it
    ///   is not well defined and as such an error is returned.
    pub fn diatomic_valence_electrons(&self) -> Result<Option<(i16, i16)>, crate::errors::Error> {
        if !self.is_diatomic()? {
            return Ok(None);
        }
        Ok(match self {
            Self::Isotope(_) | Self::Element(_) | Self::Residual => {
                unreachable!()
            }
            Self::Mixture(_) => return Err(crate::errors::Error::InvalidOperationForMixture),
            _ => {
                let mut left = None;
                let mut right = None;
                self.inner_diatomic_valence_electrons(&mut left, &mut right);
                Some((left.unwrap(), right.unwrap()))
            }
        })
    }
}
