//! Submodule providing a trait defining a taxonomy.

use serde::Serialize;

use super::TaxonEntry;
use crate::{errors::TaxonomyError, traits::taxon::Taxon};

#[derive(Debug, Serialize)]
struct CSVTaxonEntry<TE: TaxonEntry> {
    id: TE::Id,
    name: String,
    parent_id: Option<TE::Id>,
    #[serde(rename = "ranks.name")]
    rank: TE::Rank,
}

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

    /// Writes the taxonomy to a CSV file.
    /// 
    /// # Arguments
    /// 
    /// * `path`: The path where to write out the taxonomy.
    /// 
    /// # Errors
    /// 
    /// * If an `IOError` occurs while writing out the taxonomy.
    fn to_csv(
        &self,
        path: &str,
    ) -> Result<(), TaxonomyError<<Self::TaxonEntry as super::TaxonEntry>::Id>> {
        let mut writer = csv::Writer::from_path(path)?;

        for taxon in self.taxons() {
            let taxon_entry: CSVTaxonEntry<Self::TaxonEntry> = CSVTaxonEntry {
                id: *taxon.id(),
                name: taxon.name().to_string(),
                parent_id: taxon.parent_id().copied(),
                rank: *taxon.rank(),
            };
            writer.serialize(taxon_entry)?;
        }

        writer.flush()?;

        Ok(())
    }

    /// Returns the root of the taxonomy.
    fn root(&self) -> Self::Taxon<'_>;
}
