//! Submodule providing the `contains_residual` method for the
//! `MolecularFormula` struct

impl super::MolecularFormula {
    /// Checks if the molecular formula contains a residual.
    pub fn contains_residual(&self) -> bool {
        match self {
            Self::Element(_) | Self::Isotope(_) | Self::Greek(_) => false,
            Self::Ion(ion) => ion.entry.contains_residual(),
            Self::Mixture(formulas) | Self::Sequence(formulas) => {
                formulas.iter().any(Self::contains_residual)
            }
            Self::Count(formula, _) | Self::RepeatingUnit(formula) | Self::Complex(formula) => {
                formula.contains_residual()
            }
            Self::Residual => true,
            Self::Radical(formula, _) => formula.contains_residual(),
        }
    }
}
