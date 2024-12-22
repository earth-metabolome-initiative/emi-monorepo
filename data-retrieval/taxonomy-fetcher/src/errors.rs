//! Submodule defining the errors used across the taxonomy-fetcher crate.

use crate::traits::{TaxonEntry, TaxonIdentifier};

/// Enum defining the errors that can occur when fetching a taxonomy.
pub enum TaxonomyError<TaxonId: TaxonIdentifier> {
    /// When the taxonomy identifier is not found.
    TaxonNotFoundError(TaxonId),
}

/// Enum defining the errors that can occur when building a taxonomy.
pub enum TaxonomyBuilderError<TaxonId: TaxonIdentifier> {
    /// Whether a taxonomy is disconnected.
    MultipleRootsError(Vec<TaxonId>),
}

/// Enum defining the errors that can occur when building a taxon.
pub enum TaxonEntryBuilderError<TE: TaxonEntry> {
    /// When a parent identifier is not found.
    ParentNotFoundError(TE),
    /// When a taxon identifier is not unique.
    DuplicateIdentifierError(TE),
    /// When a taxon name is not unique.
    DuplicateNameError(TE),
    /// When the parent rank is not higher than the child rank.
    InconsistentRankError {
        /// Parent taxon.
        parent: TE,
        /// Child taxon.
        child: TE,
    },
    /// When a circular reference is detected.
    CircularReferenceError(TE),
    /// When a provided string cannot be converted to a rank.
    UnknownRank(String),
}
