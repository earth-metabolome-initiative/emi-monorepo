//! Submodule defining the trait for building taxon entries.

/// Trait defining a taxon builder.
pub trait TaxonEntryBuilder: Sized {
    /// Type of the taxon to build.
    type TaxonEntry: super::TaxonEntry;

    /// Sets the identifier of the taxon.
    ///
    /// # Arguments
    ///
    /// * `id` - Identifier of the taxon.
    ///
    /// # Errors
    ///
    /// * [`TaxonEntryBuilderError`] when the identifier is not unique.
    fn set_id(
        self,
        id: <Self::TaxonEntry as super::TaxonEntry>::Id,
    ) -> Result<Self, crate::errors::TaxonEntryBuilderError<Self::TaxonEntry>>;

    /// Sets the name of the taxon.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the taxon.
    ///
    /// # Errors
    ///
    /// * [`TaxonEntryBuilderError`] when the name is not unique.
    fn set_name<S: ToString>(
        self,
        name: S,
    ) -> Result<Self, crate::errors::TaxonEntryBuilderError<Self::TaxonEntry>>;

    /// Sets the rank of the taxon.
    ///
    /// # Arguments
    ///
    /// * `rank` - Rank of the taxon.
    ///
    /// # Errors
    ///
    /// * If the provided rank is invalid relatively to the other set
    ///   parameters.
    fn set_rank(
        self,
        rank: <Self::TaxonEntry as super::TaxonEntry>::Rank,
    ) -> Result<Self, crate::errors::TaxonEntryBuilderError<Self::TaxonEntry>>;

    /// Sets the parent identifier of the taxon.
    ///
    /// # Arguments
    ///
    /// * `parent_id` - Identifier of the parent taxon.
    ///
    /// # Errors
    ///
    /// * [`TaxonEntryBuilderError`] when the parent identifier is not unique.
    fn set_parent_id(
        self,
        parent_id: Option<<Self::TaxonEntry as super::TaxonEntry>::Id>,
    ) -> Result<Self, crate::errors::TaxonEntryBuilderError<Self::TaxonEntry>>;

    /// Builds the taxon entry.
    ///
    /// # Errors
    ///
    /// * [`TaxonEntryBuilderError`] when the taxon entry cannot be built.
    fn build<TB>(
        self,
        taxon_builder: &TB,
    ) -> Result<Self::TaxonEntry, crate::errors::TaxonEntryBuilderError<Self::TaxonEntry>>
    where
        TB: super::TaxonomyBuilder<TaxonEntry = Self::TaxonEntry, TaxonEntryBuilder = Self>;
}
