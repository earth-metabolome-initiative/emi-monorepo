//! Implementation of the taxonomy builder for the NCBI taxonomy.

use std::{io::BufReader, str::FromStr};

use csv::ReaderBuilder;
use downloader::Downloader;
use serde::Deserialize;

use super::{
    taxon_entry::NCBITaxonEntry, taxon_entry_builder::NCBITaxonEntryBuilder,
    taxonomy::NCBITaxonomy, version::NCBIVersion,
};
use crate::{
    TaxonEntryBuilder, impls::ncbi::rank::NCBIRank, traits::TaxonomyBuilder,
    utils::separator_fixed_reader::SeparatorFixedReader,
};

#[derive(Default)]
/// Implementation of the taxonomy trait for the NCBI.
pub struct NCBITaxonomyBuilder {
    /// Version of the NCBI taxonomy to build.
    version: Option<NCBIVersion>,
    /// Set the directory where the taxonomy is stored.
    directory: Option<std::path::PathBuf>,
    /// Root of the taxonomy.
    root_position: Option<u32>,
    /// Taxon entries.
    taxon_entries: Vec<NCBITaxonEntry>,
    /// Hashmap from taxon name to position in taxon entries.
    name_to_position: std::collections::HashMap<String, u32>,
    /// Hashmap from taxon identifier to position in taxon entries.
    id_to_position: std::collections::HashMap<u32, u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TaxonomyNameClass {
    Teleomorph,
    Anamorph,
    Synonym,
    Authority,
    ScientificName,
    BlastName,
    GenBankCommonName,
    GenBankSynonym,
    GenBankAcronym,
    GenBankAnamorph,
    InPart,
    EquivalentName,
    Includes,
    CommonName,
    Acronym,
}

impl FromStr for TaxonomyNameClass {
    type Err = crate::errors::TaxonEntryBuilderError<super::taxon_entry::NCBITaxonEntry>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "teleomorph" => Ok(TaxonomyNameClass::Teleomorph),
            "anamorph" => Ok(TaxonomyNameClass::Anamorph),
            "synonym" => Ok(TaxonomyNameClass::Synonym),
            "authority" => Ok(TaxonomyNameClass::Authority),
            "scientific name" => Ok(TaxonomyNameClass::ScientificName),
            "blast name" => Ok(TaxonomyNameClass::BlastName),
            "genbank common name" => Ok(TaxonomyNameClass::GenBankCommonName),
            "genbank synonym" => Ok(TaxonomyNameClass::GenBankSynonym),
            "genbank acronym" => Ok(TaxonomyNameClass::GenBankAcronym),
            "genbank anamorph" => Ok(TaxonomyNameClass::GenBankAnamorph),
            "in-part" => Ok(TaxonomyNameClass::InPart),
            "equivalent name" => Ok(TaxonomyNameClass::EquivalentName),
            "includes" => Ok(TaxonomyNameClass::Includes),
            "common name" => Ok(TaxonomyNameClass::CommonName),
            "acronym" => Ok(TaxonomyNameClass::Acronym),
            _ => {
                Err(crate::errors::TaxonEntryBuilderError::UnknownTaxonomicalNameClass(
                    s.to_owned(),
                ))
            }
        }
    }
}

impl<'de> Deserialize<'de> for TaxonomyNameClass {
    fn deserialize<D>(deserializer: D) -> Result<TaxonomyNameClass, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        TaxonomyNameClass::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
/// Row in the NCBI taxonomy.
struct NamesRow {
    /// Unique identifier of the taxon.
    tax_id: u32,
    /// Name of the taxon
    name_txt: String,
    /// Unique identifier of the name.
    unique_name: Option<String>,
    /// Name class.
    name_class: TaxonomyNameClass,
}

/// Returns an integer as boolean
fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: u8 = u8::deserialize(deserializer)?;
    Ok(s == 1)
}

/// Returns an integer as boolean if present.
fn maybe_deserialize_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<u8> = Option::deserialize(deserializer)?;
    Ok(s.map(|s| s == 1))
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
/// Row in the NCBI taxonomy.
struct NodesRow {
    /// Unique identifier of the taxon.
    tax_id: u32,
    /// Unique identifier of the parent.
    parent_tax_id: u32,
    /// Rank of the taxon.
    rank: NCBIRank,
    /// Locus-name prefix; not unique.
    embl_code: Option<String>,
    /// Division identifier.
    division_id: u32,
    /// 1 if node inherits division from parent.
    #[serde(deserialize_with = "deserialize_bool")]
    inherited_div_flag: bool,
    /// Genetic code identifier.
    genetic_code_id: u32,
    /// 1 if node inherits genetic code from parent.
    #[serde(deserialize_with = "deserialize_bool")]
    inherited_gc_flag: bool,
    /// Mitochondrial genetic code identifier.
    mitochondrial_genetic_code_id: u32,
    /// 1 if node inherits mitochondrial gencode from parent.
    #[serde(deserialize_with = "deserialize_bool")]
    inherited_mgc_flag: bool,
    /// 1 if name is suppressed in GenBank entry lineage.
    #[serde(deserialize_with = "deserialize_bool")]
    genbank_hidden_flag: bool,
    /// 1 if this subtree has no sequence data yet.
    #[serde(deserialize_with = "deserialize_bool")]
    hidden_subtree_root_flag: bool,
    /// Free-text comments and citations.
    comments: String,
    /// Plastid genetic code identifier.
    plastid_genetic_code_id: Option<u32>,
    /// 1 if node inherits plastid gencode from parent.
    #[serde(deserialize_with = "maybe_deserialize_bool")]
    inherited_pgc_flag: Option<bool>,
    /// 1 if species in the node's lineage has formal name.
    #[serde(deserialize_with = "deserialize_bool")]
    specified_species: bool,
    /// Hydrogenosome genetic code identifier.
    hydrogenosome_genetic_code_id: Option<u32>,
    /// 1 if node inherits hydrogenosome gencode from parent.
    #[serde(deserialize_with = "deserialize_bool")]
    inherited_hgc_flag: bool,
}

impl TaxonomyBuilder for NCBITaxonomyBuilder {
    type TaxonEntry = NCBITaxonEntry;
    type Taxonomy = NCBITaxonomy;
    type TaxonEntryBuilder = NCBITaxonEntryBuilder;

    fn version(self, version: <Self::Taxonomy as crate::traits::Taxonomy>::Version) -> Self {
        Self { version: Some(version), ..self }
    }

    fn directory(self, directory: std::path::PathBuf) -> Self {
        Self { directory: Some(directory), ..self }
    }

    fn is_id_in_use(&self, id: &u32) -> bool {
        self.id_to_position.contains_key(id)
    }

    fn is_name_in_use(&self, name: &str) -> bool {
        self.name_to_position.contains_key(name)
    }

    fn get_taxon_entry(
        &self,
        id: &<Self::TaxonEntry as crate::traits::TaxonEntry>::Id,
    ) -> Option<&Self::TaxonEntry> {
        self.id_to_position.get(id).map(|&pos| &self.taxon_entries[pos as usize])
    }

    async fn build(
        mut self,
    ) -> Result<Self::Taxonomy, crate::errors::TaxonomyBuilderError<Self::TaxonEntry>> {
        let version = self.version.ok_or(crate::errors::TaxonomyBuilderError::MissingVersion)?;
        let _reports = Downloader::default()
            .task(version.url())?
            .extract()
            .cache()
            .show_loading_bar()
            .execute()
            .await?;

        // We create a reader where we replace the odd delimiter
        // that is used in the NCBI taxonomy, i.e. '\t|\t',
        // by a comma, in stream mode.
        let names_reader =
            BufReader::new(std::fs::File::open(format!("{}/names.dmp", version.directory()))?);

        let names_reader = SeparatorFixedReader::new(names_reader, "\t", "\t|\t");
        let names_reader = SeparatorFixedReader::new(names_reader, "\n", "\t|\n");

        let names_reader = SeparatorFixedReader::new(names_reader, "\'", "\"");

        // We read the taxonomy file.
        let mut names_csv_reader =
            ReaderBuilder::new().delimiter(b'\t').has_headers(false).from_reader(names_reader);

        let mut names_iter = names_csv_reader.deserialize::<NamesRow>();

        // We create a reader where we replace the odd delimiter
        // that is used in the NCBI taxonomy, i.e. '\t|\t',
        // by a comma, in stream mode.
        let nodes_reader = SeparatorFixedReader::new(
            BufReader::new(std::fs::File::open(format!("{}/nodes.dmp", version.directory()))?),
            "\t",
            "\t|\t",
        );
        let nodes_reader = SeparatorFixedReader::new(nodes_reader, "\n", "\t|\n");

        // We read the taxonomy file.
        let mut nodes_csv_reader =
            ReaderBuilder::new().delimiter(b'\t').has_headers(false).from_reader(nodes_reader);

        // We iterate over the records.
        'external: for nodes_row in nodes_csv_reader.deserialize() {
            let record: NodesRow = nodes_row?;

            let name_row: NamesRow = loop {
                if let Some(new_name_row) = names_iter.next().transpose()? {
                    if new_name_row.name_class == TaxonomyNameClass::ScientificName {
                        break new_name_row;
                    }
                } else {
                    break 'external;
                }
            };

            assert_eq!(
                record.tax_id, name_row.tax_id,
                "Taxon identifiers do not match: {:?}, {:?}, version: {:?}",
                record, name_row, version
            );

            let taxon_entry = NCBITaxonEntryBuilder::default()
                .set_id(record.tax_id)?
                .set_name(name_row.name_txt)?
                .set_parent_id(if record.parent_tax_id == record.tax_id {
                    None
                } else {
                    Some(record.parent_tax_id)
                })?
                .set_rank(record.rank)?
                .build(&self)?;

            self.id_to_position.insert(taxon_entry.id, self.taxon_entries.len() as u32);
            self.name_to_position.insert(taxon_entry.name.clone(), self.taxon_entries.len() as u32);
            if record.parent_tax_id == record.tax_id {
                if self.root_position.is_some() {
                    return Err(crate::errors::TaxonomyBuilderError::MultipleRoots);
                }
                self.root_position = Some(self.taxon_entries.len() as u32);
            }
            self.taxon_entries.push(taxon_entry);
        }

        // We check that the root position has been set.
        if self.root_position.is_none() {
            return Err(crate::errors::TaxonomyBuilderError::NoRoot);
        }

        // We check for each taxon entry that has a parent that the parent exists.
        for taxon_entry in &self.taxon_entries {
            if let Some(parent_id) = taxon_entry.parent_id {
                if !self.id_to_position.contains_key(&parent_id) {
                    return Err(crate::errors::TaxonEntryBuilderError::ParentNotFound(
                        taxon_entry.clone(),
                    )
                    .into());
                }
            }

            // TODO! Check that parent has a compatible rank!
        }

        Ok(NCBITaxonomy {
            version,
            root_position: self.root_position.unwrap(),
            taxon_entries: self.taxon_entries,
        })
    }
}
