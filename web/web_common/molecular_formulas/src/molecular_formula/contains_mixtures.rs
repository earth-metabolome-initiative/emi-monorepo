//! Submodule providing the `contains_mixture` method for the
//! `MolecularFormula` struct

impl super::MolecularFormula {
    /// Checks if the molecular formula contains a mixture.
    pub fn contains_mixture(&self) -> bool {
        match self {
            Self::Element(_) | Self::Residual | Self::Isotope(_) | Self::Greek(_) => false,
            Self::Ion(ion) => ion.entry.contains_mixture(),
            Self::Mixture(_) => true,
            Self::Sequence(formulas) => formulas.iter().any(Self::contains_mixture),
            Self::Count(formula, _) | Self::RepeatingUnit(formula) | Self::Complex(formula) => {
                formula.contains_mixture()
            }
            Self::Radical(formula, _) => {
                debug_assert!(!formula.contains_mixture(), "Radical should not contain mixtures");
                false
            }
        }
    }
}
