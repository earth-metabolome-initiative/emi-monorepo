//! Submodule providing the `contains_isotope` method for the
//! `MolecularFormula` struct

impl super::MolecularFormula {
    /// Checks if the molecular formula is isotopically defined.
    pub fn contains_isotope(&self) -> bool {
        match self {
            Self::Element(_) | Self::Residual => false,
            Self::Isotope(_) => true,
            Self::Ion(ion) => ion.entry.contains_isotope(),
            Self::Mixture(formulas) | Self::Sequence(formulas) => {
                formulas.iter().any(Self::contains_isotope)
            }
            Self::Count(formula, _) | Self::RepeatingUnit(formula) | Self::Complex(formula) => {
                formula.contains_isotope()
            }
        }
    }
}
