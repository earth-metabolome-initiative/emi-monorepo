//! Submodule defining the errors used across the taxonomy-fetcher crate.

use crate::traits::{TaxonEntry, TaxonIdentifier};
use csv::Error as CsvError;
use downloader::DownloaderError;
use std::io::Error as IoError;

#[derive(Debug)]
/// Enum defining the errors that can occur when fetching a taxonomy.
pub enum TaxonomyError<TaxonId: TaxonIdentifier> {
    /// When the taxonomy identifier is not found.
    TaxonNotFound(TaxonId),
    /// Whether errors have occurred while reading a CSV/TSV file.
    CsvError(CsvError),
}

#[derive(Debug)]
/// Enum defining the errors that can occur when building a taxonomy.
pub enum TaxonomyBuilderError<TE: TaxonEntry> {
    /// Whether no root was found.
    NoRoot,
    /// Whether a taxonomy is disconnected.
    MultipleRoots,
    /// Whether the version was not specified.
    MissingVersion,
    /// Whether errors have occurred during the download.
    DownloaderError(DownloaderError),
    /// Whether errors have occurred while reading a CSV/TSV file.
    CsvError(CsvError),
    /// Whether reading a document failed.
    IoError(IoError),
    /// Whether errors have occurred while building a taxon.
    TaxonEntryBuilderError(TaxonEntryBuilderError<TE>),
}

impl<TE: TaxonEntry> TaxonomyBuilderError<TE> {
    /// Returns whether the error is a missing version error.
    pub fn is_multiple_roots(&self) -> bool {
        matches!(self, Self::MultipleRoots)
    }
}

#[derive(Debug)]
/// Enum defining the errors that can occur when building a taxon.
pub enum TaxonEntryBuilderError<TE: TaxonEntry> {
    /// When a parent identifier is not found.
    ParentNotFound(TE),
    /// When a taxon identifier is not unique.
    DuplicateIdentifierError(TE::Id),
    /// When a taxon name is not unique.
    DuplicateNameError(String),
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
    /// When a provided string cannot be converted to a taxonomical status.
    UnknownTaxonomicalStatus(String),
    /// When a provided string cannot be converted to a taxonomical name class.
    UnknownTaxonomicalNameClass(String),
    /// When a build is attempted while the taxon is missing the rank.
    MissingRank,
    /// When a build is attempted while the taxon is missing the name.
    MissingName,
    /// When a build is attempted while the taxon is missing the identifier.
    MissingId,
}

impl<TE: TaxonEntry> std::fmt::Display for TaxonEntryBuilderError<TE> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ParentNotFound(taxon) => write!(f, "Parent taxon not found: {}", taxon),
            Self::DuplicateIdentifierError(id) => write!(f, "Duplicate taxon identifier: {}", id),
            Self::DuplicateNameError(name) => write!(f, "Duplicate taxon name: {}", name),
            Self::InconsistentRankError { parent, child } => {
                write!(f, "Inconsistent rank: parent {} child {}", parent, child)
            }
            Self::CircularReferenceError(taxon) => {
                write!(f, "Circular reference: {}", taxon)
            }
            Self::UnknownRank(rank) => write!(f, "Unknown rank: '{}'", rank),
            Self::UnknownTaxonomicalStatus(status) => {
                write!(f, "Unknown taxonomical status: '{}'", status)
            }
            Self::UnknownTaxonomicalNameClass(name_class) => {
                write!(f, "Unknown taxonomical name class: '{}'", name_class)
            }
            Self::MissingRank => write!(f, "Missing rank"),
            Self::MissingName => write!(f, "Missing name"),
            Self::MissingId => write!(f, "Missing identifier"),
        }
    }
}

impl<TE: TaxonEntry> From<DownloaderError> for TaxonomyBuilderError<TE> {
    fn from(error: DownloaderError) -> Self {
        Self::DownloaderError(error)
    }
}

impl<TE: TaxonEntry> From<CsvError> for TaxonomyBuilderError<TE> {
    fn from(error: CsvError) -> Self {
        Self::CsvError(error)
    }
}

impl<TE: TaxonEntry> From<IoError> for TaxonomyBuilderError<TE> {
    fn from(error: IoError) -> Self {
        Self::IoError(error)
    }
}

impl<TaxonId: TaxonIdentifier> From<CsvError> for TaxonomyError<TaxonId> {
    fn from(error: CsvError) -> Self {
        Self::CsvError(error)
    }
}

impl<TE: TaxonEntry> From<TaxonEntryBuilderError<TE>> for TaxonomyBuilderError<TE> {
    fn from(error: TaxonEntryBuilderError<TE>) -> Self {
        Self::TaxonEntryBuilderError(error)
    }
}
