//! Submodule implementing the `countable` method for the `MolecularFormula`
//! struct which returns whether the specific sub-formula component can be
//! counted.

use super::MolecularFormula;

impl MolecularFormula {
    #[must_use]
    /// Returns `true` if the formula can be counted, i.e., it is not a `Count`
    /// or `Greek` type.
    pub fn is_countable(&self) -> bool {
        match self {
            Self::Sequence(_)
            | Self::Radical(_, _)
            | Self::Mixture(_)
            | Self::Isotope(_)
            | Self::Element(_)
            | Self::Ion(_)
            | Self::Residual
            | Self::Complex(_)
            | Self::RepeatingUnit(_) => true,
            Self::Count(_, _) | Self::Greek(_) => false,
        }
    }
}
