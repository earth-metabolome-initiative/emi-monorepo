//! Submodule providing the `contains_elements` method for the
//! `MolecularFormula` struct

impl super::MolecularFormula {
    /// Checks if the molecular formula contains elements.
    pub fn contains_elements(&self) -> bool {
        match self {
            Self::Element(_) => true,
            Self::Isotope(_) | Self::Residual | Self::Greek(_) => false,
            Self::Ion(ion) => ion.entry.contains_elements(),
            Self::Mixture(formulas) | Self::Sequence(formulas) => {
                formulas.iter().any(Self::contains_elements)
            }
            Self::Count(formula, _) | Self::RepeatingUnit(formula) | Self::Complex(formula) => {
                formula.contains_elements()
            }
            Self::Radical(formula, _) => formula.contains_elements(),
        }
    }
}
