//! Submodule implementing the `is_homodiatomic` method for the
//! `MolecularFormula` struct, which returns whether the formula is homodiatomic
//! (e.g. `H2`, `O2`, `N2`, etc.).

impl crate::MolecularFormula {
    /// Returns whether the formula is homodiatomic, such as `H2`, `O2`,
    /// `N2`, `H2+`, `O2-`, etc. A mixture is considered homodiatomic if all
    /// of the sub-formulas are diatomic.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let formula = MolecularFormula::from_str("H2").unwrap();
    /// assert!(formula.is_homodiatomic().unwrap());
    ///
    /// let formula = MolecularFormula::from_str("H2O").unwrap();
    ///
    /// assert!(!formula.is_homodiatomic().unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// * If the formula contains a non-element, such as a `Residual`.
    pub fn is_homodiatomic(&self) -> Result<bool, crate::errors::Error> {
        Ok(self.is_homonuclear()? && self.number_of_elements()? == 2)
    }
}
