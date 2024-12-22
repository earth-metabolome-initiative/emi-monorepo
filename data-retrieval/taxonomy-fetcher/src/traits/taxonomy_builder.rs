//! Submodule defining a trait for building a taxonomy.

/// Trait defining a taxonomy builder.
pub trait TaxonomyBuilder {
    /// Type of the taxonomy to build.
    type Taxonomy: super::Taxonomy;
    /// Type of the taxon builder to use.
    type TaxonEntryBuilder: super::TaxonEntryBuilder<TaxonEntry = <Self::Taxonomy as super::Taxonomy>::TaxonEntry>;

    /// Builds a taxonomy from the given CSV file.
    fn build(
        &self,
    ) -> Result<
        Self::Taxonomy,
        crate::errors::TaxonomyBuilderError<
            <<Self::Taxonomy as super::Taxonomy>::TaxonEntry as super::TaxonEntry>::Id,
        >,
    >;
}
