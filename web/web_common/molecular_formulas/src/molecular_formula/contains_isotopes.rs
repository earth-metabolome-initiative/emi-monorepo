//! Submodule providing the `contains_isotope` method for the
//! `MolecularFormula` struct

impl super::MolecularFormula {
    /// Checks if the molecular formula is isotopically defined.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use molecular_formulas::MolecularFormula;
    ///
    /// let water = MolecularFormula::try_from("H2O")?;
    /// assert!(!water.contains_isotope());
    ///
    /// let formula = MolecularFormula::try_from("³H2O")?;
    /// assert!(formula.contains_isotope());
    ///
    /// let formula = MolecularFormula::try_from("D2O")?;
    /// assert!(formula.contains_isotope());
    ///
    /// let formula = MolecularFormula::try_from("T2O")?;
    /// assert!(formula.contains_isotope());
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn contains_isotope(&self) -> bool {
        match self {
            Self::Element(_) | Self::Residual | Self::Greek(_) => false,
            Self::Isotope(_) => true,
            Self::Ion(ion) => ion.entry.contains_isotope(),
            Self::Mixture(formulas) | Self::Sequence(formulas) => {
                formulas.iter().any(Self::contains_isotope)
            }
            Self::Count(formula, _) | Self::RepeatingUnit(formula) | Self::Complex(formula) => {
                formula.contains_isotope()
            }
            Self::Radical(formula, _) => formula.contains_isotope(),
        }
    }
}
