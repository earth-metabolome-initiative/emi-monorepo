//! Submodule implementing the TaxonEntry for a generic taxonomy.

use crate::traits::{Rank, TaxonEntry, TaxonIdentifier};

/// A generic implementation of a taxon entry.
pub struct GenericTaxonEntry<Id: TaxonIdentifier, R: Rank> {
    /// Identifier of the taxon.
    pub id: Id,
    /// Name of the taxon.
    pub name: String,
    /// Rank of the taxon.
    pub rank: R,
    /// Identifier of the parent taxon.
    pub parent_id: Option<Id>,
}

impl<Id: TaxonIdentifier, R: Rank> TaxonEntry for GenericTaxonEntry<Id, R> {
    type Id = Id;
    type Rank = R;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn rank(&self) -> &Self::Rank {
        &self.rank
    }

    fn parent_id(&self) -> Option<&Self::Id> {
        self.parent_id.as_ref()
    }
}
