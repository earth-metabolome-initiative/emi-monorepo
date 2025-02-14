//! Submodule defining a trait for building a taxonomy.
use crate::traits::taxon_version::TaxonVersion;

/// Trait defining a taxonomy builder.
pub trait TaxonomyBuilder: Sized + Default {
    /// The type of TaxonEntry to build.
    type TaxonEntry: super::TaxonEntry;
    /// Type of the taxonomy to build.
    type Taxonomy: super::Taxonomy<TaxonEntry = Self::TaxonEntry>;
    /// Type of the taxon builder to use.
    type TaxonEntryBuilder: super::TaxonEntryBuilder<TaxonEntry = Self::TaxonEntry>;

    /// Creates a new taxonomy builder with the latest version.
    fn latest() -> Self {
        Self::default().version(<Self::Taxonomy as super::Taxonomy>::Version::latest())
    }

    /// Sets the version of the taxonomy to build.
    fn version(self, version: <Self::Taxonomy as super::Taxonomy>::Version) -> Self;

    /// Sets the directory where the taxonomy is stored.
    fn directory(self, directory: std::path::PathBuf) -> Self;

    /// Returns whether a provided Id is already in use.
    fn is_id_in_use(&self, id: &<Self::TaxonEntry as super::TaxonEntry>::Id) -> bool;

    /// Returns whether a provided name is already in use.
    fn is_name_in_use(&self, name: &str) -> bool;

    /// Returns the [`TaxonEntry`] associated to the provided identifier, if any.
    fn get_taxon_entry(
        &self,
        id: &<Self::TaxonEntry as super::TaxonEntry>::Id,
    ) -> Option<&Self::TaxonEntry>;

    /// Builds a taxonomy from the given CSV file.
    fn build(
        self,
    ) -> impl std::future::Future<
        Output = Result<Self::Taxonomy, crate::errors::TaxonomyBuilderError<Self::TaxonEntry>>,
    > + Send;
}
