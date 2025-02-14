//! Submodule providing a trait defining a taxonomy.

use serde::Serialize;

use super::TaxonEntry;
use crate::errors::TaxonomyError;
use crate::traits::taxon::Taxon;

/// Trait defining a taxonomy.
pub trait Taxonomy {
    /// Type of the taxon entry.
    type TaxonEntry: super::TaxonEntry;
    /// Type of the identifier for the taxonomy.
    type Version: super::TaxonVersion;
    /// Type of the taxon.
    type Taxon<'a>: super::Taxon<'a, Taxonomy = Self>
    where
        Self: 'a;

    /// Returns the name of the taxonomy.
    fn name(&self) -> &str;

    /// Returns the version of the taxonomy.
    fn version(&self) -> Self::Version;

    /// Returns the taxon entry for the given identifier.
    ///
    /// # Arguments
    ///
    /// * `id`: Identifier of the taxon to retrieve.
    ///
    /// # Errors
    ///
    /// * [`TaxonNotFoundError`] if the taxon identifier is not found.
    fn taxon_by_id(
        &self,
        id: &<Self::TaxonEntry as super::TaxonEntry>::Id,
    ) -> Result<Self::Taxon<'_>, TaxonomyError<<Self::TaxonEntry as super::TaxonEntry>::Id>>;

    /// Iterates the taxon entries.
    fn taxons(&self) -> impl Iterator<Item = Self::Taxon<'_>> + '_;

    /// Returns the root of the taxonomy.
    fn root(&self) -> Self::Taxon<'_>;
}
