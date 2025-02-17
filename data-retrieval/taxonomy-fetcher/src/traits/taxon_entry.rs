//! Submodule providing a taxonomy entry trait.

use std::fmt::{Debug, Display};

/// Trait defining a taxon entry.
pub trait TaxonEntry: Display + Debug {
    /// Type of the identifier for the taxon.
    type Id: super::TaxonIdentifier;
    /// Type of the rank of the taxon.
    type Rank: super::Rank;

    /// Returns the identifier of the taxon.
    fn id(&self) -> &Self::Id;

    /// Returns the name of the taxon.
    fn name(&self) -> &str;

    /// Returns the rank of the taxon.
    fn rank(&self) -> &Self::Rank;

    /// Returns the identifier of the parent taxon.
    fn parent_id(&self) -> Option<&Self::Id>;
}
