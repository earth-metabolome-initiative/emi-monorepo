//! Submodule providing the `contains_residual` method for the
//! `MolecularFormula` struct

impl super::MolecularFormula {
    /// Checks if the molecular formula contains a residual.
    pub fn contains_residual(&self) -> bool {
        match self {
            Self::Element(_) => false,
            Self::Ion(ion) => ion.entry.contains_residual(),
            Self::Solvation(solvation) => solvation.contains_residual(),
            Self::Count(formula, _) => formula.contains_residual(),
            Self::Sequence(formulas) => formulas.iter().any(Self::contains_residual),
            Self::Complex(formula) => formula.contains_residual(),
            Self::RepeatingUnit(formula) => formula.contains_residual(),
            Self::Residual => true,
        }
    }
}
