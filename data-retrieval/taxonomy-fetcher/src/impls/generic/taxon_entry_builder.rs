//! Submodule implementing the TaxonEntryBuilder trait for the Open Tree of Life taxonomy.

use crate::traits::{Rank, TaxonEntryBuilder, TaxonIdentifier, TaxonomyBuilder};

use super::taxon_entry::GenericTaxonEntry;

/// Implementation of the taxon entry builder for the Open Tree of Life taxonomy.
pub struct GenericTaxonEntryBuilder<Id: TaxonIdentifier, R: Rank> {
    /// Identifier of the taxon entry.
    id: Option<Id>,
    /// Name of the taxon entry.
    name: Option<String>,
    /// Rank of the taxon entry.
    rank: Option<R>,
    /// Parent of the taxon entry.
    parent_id: Option<Id>,
}

impl<Id: TaxonIdentifier, R: Rank> TaxonEntryBuilder for GenericTaxonEntryBuilder<Id, R> {
    type TaxonEntry = GenericTaxonEntry<Id, R>;

    fn set_id(
        mut self,
        id: <Self::TaxonEntry as crate::traits::TaxonEntry>::Id,
    ) -> Result<Self, crate::errors::TaxonEntryBuilderError<Self::TaxonEntry>> {
        self.id = Some(id);
        Ok(self)
    }

    fn set_name<S: ToString>(
        self,
        name: S,
    ) -> Result<Self, crate::errors::TaxonEntryBuilderError<Self::TaxonEntry>> {
        Ok(Self {
            name: Some(name.to_string()),
            ..self
        })
    }

    fn set_parent_id(
        mut self,
        parent_id: Option<<Self::TaxonEntry as crate::traits::TaxonEntry>::Id>,
    ) -> Result<Self, crate::errors::TaxonEntryBuilderError<Self::TaxonEntry>> {
        self.parent_id = parent_id;
        Ok(self)
    }

    fn set_rank(
        mut self,
        rank: <Self::TaxonEntry as crate::traits::TaxonEntry>::Rank,
    ) -> Result<Self, crate::errors::TaxonEntryBuilderError<Self::TaxonEntry>> {
        self.rank = Some(rank);
        Ok(self)
    }

    fn build<TB>(
        self,
        taxonomy_builder: &TB,
    ) -> Result<Self::TaxonEntry, crate::errors::TaxonEntryBuilderError<Self::TaxonEntry>>
    where
        TB: TaxonomyBuilder<TaxonEntry = Self::TaxonEntry, TaxonEntryBuilder = Self>,
    {
        let id = self
            .id
            .ok_or(crate::errors::TaxonEntryBuilderError::MissingId)?;
        let name = self
            .name
            .ok_or(crate::errors::TaxonEntryBuilderError::MissingName)?;
        let rank = self
            .rank
            .ok_or(crate::errors::TaxonEntryBuilderError::MissingRank)?;

        if taxonomy_builder.is_id_in_use(&id) {
            return Err(crate::errors::TaxonEntryBuilderError::DuplicateIdentifierError(id));
        }

        if taxonomy_builder.is_name_in_use(&name) {
            return Err(crate::errors::TaxonEntryBuilderError::DuplicateNameError(
                name,
            ));
        }

        Ok(GenericTaxonEntry {
            id,
            name,
            rank,
            parent_id: self.parent_id,
        })
    }
}
