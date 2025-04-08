//! Submodule defining the versions of the Open Tree of Life taxonomy.

use chrono::NaiveDateTime;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::traits::TaxonVersion;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
/// Enum representing the versions of the Open Tree of Life taxonomy.
pub enum OpenTreeOfLifeVersion {
    /// The version 2.2 of the Open Tree of Life taxonomy.
    V2_2,
    /// The version 2.3 of the Open Tree of Life taxonomy.
    V2_3,
    /// The version 2.4 of the Open Tree of Life taxonomy.
    V2_4,
    /// The version 2.5 of the Open Tree of Life taxonomy.
    V2_5,
    /// The version 2.6 of the Open Tree of Life taxonomy.
    V2_6,
    /// The version 2.7 of the Open Tree of Life taxonomy.
    V2_7,
    /// The version 2.8 of the Open Tree of Life taxonomy.
    V2_8,
    /// The version 2.9 of the Open Tree of Life taxonomy.
    V2_9,
    /// The version 2.10 of the Open Tree of Life taxonomy.
    V2_10,
    /// The version 3.0 of the Open Tree of Life taxonomy.
    V3_0,
    /// The version 3.1 of the Open Tree of Life taxonomy.
    V3_1,
    /// The version 3.2 of the Open Tree of Life taxonomy.
    V3_2,
    /// The version 3.3 of the Open Tree of Life taxonomy.
    V3_3,
    /// The version 3.4 of the Open Tree of Life taxonomy.
    V3_4,
    /// The version 3.5 of the Open Tree of Life taxonomy.
    V3_5,
    /// The version 3.6 of the Open Tree of Life taxonomy.
    V3_6,
    /// The version 3.7 of the Open Tree of Life taxonomy.
    V3_7,
}

impl OpenTreeOfLifeVersion {
    /// Returns the release date of the version.
    pub fn release_date(&self) -> NaiveDateTime {
        match self {
            OpenTreeOfLifeVersion::V2_2 => {
                NaiveDateTime::parse_from_str("2013-09-16 04:10", "%Y-%m-%d %H:%M").unwrap()
            }
            OpenTreeOfLifeVersion::V2_3 => {
                NaiveDateTime::parse_from_str("2013-12-04 22:50", "%Y-%m-%d %H:%M").unwrap()
            }
            OpenTreeOfLifeVersion::V2_4 => {
                NaiveDateTime::parse_from_str("2014-03-17 16:33", "%Y-%m-%d %H:%M").unwrap()
            }
            OpenTreeOfLifeVersion::V2_5 => {
                NaiveDateTime::parse_from_str("2014-03-28 18:12", "%Y-%m-%d %H:%M").unwrap()
            }
            OpenTreeOfLifeVersion::V2_6 => {
                NaiveDateTime::parse_from_str("2014-04-11 23:00", "%Y-%m-%d %H:%M").unwrap()
            }
            OpenTreeOfLifeVersion::V2_7 => {
                NaiveDateTime::parse_from_str("2014-05-09 00:02", "%Y-%m-%d %H:%M").unwrap()
            }
            OpenTreeOfLifeVersion::V2_8 => {
                NaiveDateTime::parse_from_str("2014-06-12 03:04", "%Y-%m-%d %H:%M").unwrap()
            }
            OpenTreeOfLifeVersion::V2_9 => {
                NaiveDateTime::parse_from_str("2015-10-13 03:37", "%Y-%m-%d %H:%M").unwrap()
            }
            OpenTreeOfLifeVersion::V2_10 => {
                NaiveDateTime::parse_from_str("2016-10-02 00:45", "%Y-%m-%d %H:%M").unwrap()
            }
            OpenTreeOfLifeVersion::V3_0 => {
                NaiveDateTime::parse_from_str("2015-10-13 03:37", "%Y-%m-%d %H:%M").unwrap()
            }
            OpenTreeOfLifeVersion::V3_1 => {
                NaiveDateTime::parse_from_str("2019-09-16 20:09", "%Y-%m-%d %H:%M").unwrap()
            }
            OpenTreeOfLifeVersion::V3_2 => {
                NaiveDateTime::parse_from_str("2019-10-30 16:42", "%Y-%m-%d %H:%M").unwrap()
            }
            OpenTreeOfLifeVersion::V3_3 => {
                NaiveDateTime::parse_from_str("2021-06-02 00:26", "%Y-%m-%d %H:%M").unwrap()
            }
            OpenTreeOfLifeVersion::V3_4 => {
                NaiveDateTime::parse_from_str("2022-10-31 22:57", "%Y-%m-%d %H:%M").unwrap()
            }
            OpenTreeOfLifeVersion::V3_5 => {
                NaiveDateTime::parse_from_str("2023-05-23 00:02", "%Y-%m-%d %H:%M").unwrap()
            }
            OpenTreeOfLifeVersion::V3_6 => {
                NaiveDateTime::parse_from_str("2023-09-19 02:16", "%Y-%m-%d %H:%M").unwrap()
            }
            OpenTreeOfLifeVersion::V3_7 => {
                NaiveDateTime::parse_from_str("2024-04-23 18:01", "%Y-%m-%d %H:%M").unwrap()
            }
        }
    }

    /// Returns the URL associated with the version.
    pub fn url(&self) -> &str {
        match self {
            OpenTreeOfLifeVersion::V2_2 => "https://files.opentreeoflife.org/ott/ott2.2/ott2.2.tgz",
            OpenTreeOfLifeVersion::V2_3 => "https://files.opentreeoflife.org/ott/ott2.3/ott2.3.tgz",
            OpenTreeOfLifeVersion::V2_4 => "https://files.opentreeoflife.org/ott/ott2.4/ott2.4.tgz",
            OpenTreeOfLifeVersion::V2_5 => "https://files.opentreeoflife.org/ott/ott2.5/ott2.5.tgz",
            OpenTreeOfLifeVersion::V2_6 => "https://files.opentreeoflife.org/ott/ott2.6/ott2.6.tgz",
            OpenTreeOfLifeVersion::V2_7 => "https://files.opentreeoflife.org/ott/ott2.7/ott2.7.tgz",
            OpenTreeOfLifeVersion::V2_8 => "https://files.opentreeoflife.org/ott/ott2.8/ott2.8.tgz",
            OpenTreeOfLifeVersion::V2_9 => "https://files.opentreeoflife.org/ott/ott2.9/ott2.9.tgz",
            OpenTreeOfLifeVersion::V2_10 => {
                "https://files.opentreeoflife.org/ott/ott2.10/ott2.10.tgz"
            }
            OpenTreeOfLifeVersion::V3_0 => "https://files.opentreeoflife.org/ott/ott3.0/ott3.0.tgz",
            OpenTreeOfLifeVersion::V3_1 => "https://files.opentreeoflife.org/ott/ott3.1/ott3.1.tgz",
            OpenTreeOfLifeVersion::V3_2 => "https://files.opentreeoflife.org/ott/ott3.2/ott3.2.tgz",
            OpenTreeOfLifeVersion::V3_3 => "https://files.opentreeoflife.org/ott/ott3.3/ott3.3.tgz",
            OpenTreeOfLifeVersion::V3_4 => "https://files.opentreeoflife.org/ott/ott3.4/ott3.4.tgz",
            OpenTreeOfLifeVersion::V3_5 => "https://files.opentreeoflife.org/ott/ott3.5/ott3.5.tgz",
            OpenTreeOfLifeVersion::V3_6 => "https://files.opentreeoflife.org/ott/ott3.6/ott3.6.tgz",
            OpenTreeOfLifeVersion::V3_7 => "https://files.opentreeoflife.org/ott/ott3.7/ott3.7.tgz",
        }
    }

    /// Returns the path to the taxonomy file in the archive.
    pub fn taxonomy_file(&self) -> &str {
        match self {
            OpenTreeOfLifeVersion::V2_2 => "ott2.2/ott2.2/taxonomy",
            OpenTreeOfLifeVersion::V2_3 => "ott2.3/ott/taxonomy.tsv",
            OpenTreeOfLifeVersion::V2_4 => "ott2.4/ott/taxonomy.tsv",
            OpenTreeOfLifeVersion::V2_5 => "ott2.5/ott/taxonomy.tsv",
            OpenTreeOfLifeVersion::V2_6 => "ott2.6/ott/taxonomy.tsv",
            OpenTreeOfLifeVersion::V2_7 => "ott2.7/ott/taxonomy.tsv",
            OpenTreeOfLifeVersion::V2_8 => "ott2.8/ott2.8/taxonomy.tsv",
            OpenTreeOfLifeVersion::V2_9 => "ott2.9/ott/taxonomy.tsv",
            OpenTreeOfLifeVersion::V2_10 => "ott2.10/ott/taxonomy.tsv",
            OpenTreeOfLifeVersion::V3_0 => "ott3.0/ott/taxonomy.tsv",
            OpenTreeOfLifeVersion::V3_1 => "ott3.1/ott3.1/taxonomy.tsv",
            OpenTreeOfLifeVersion::V3_2 => "ott3.2/ott3.2/taxonomy.tsv",
            OpenTreeOfLifeVersion::V3_3 => "ott3.3/ott3.3/taxonomy.tsv",
            OpenTreeOfLifeVersion::V3_4 => "ott3.4/ott3.4/taxonomy.tsv",
            OpenTreeOfLifeVersion::V3_5 => "ott3.5/ott3.5/taxonomy.tsv",
            OpenTreeOfLifeVersion::V3_6 => "ott3.6/ott3.6/taxonomy.tsv",
            OpenTreeOfLifeVersion::V3_7 => "ott3.7/ott3.7/taxonomy.tsv",
        }
    }

    /// Returns the path to the synonyms file in the archive.
    pub fn synonyms_file(&self) -> &str {
        match self {
            OpenTreeOfLifeVersion::V2_2 => "ott2.2/ott2.2/synonyms",
            OpenTreeOfLifeVersion::V2_3 => "ott2.3/ott/synonyms.tsv",
            OpenTreeOfLifeVersion::V2_4 => "ott2.4/ott/synonyms.tsv",
            OpenTreeOfLifeVersion::V2_5 => "ott2.5/ott/synonyms.tsv",
            OpenTreeOfLifeVersion::V2_6 => "ott2.6/ott/synonyms.tsv",
            OpenTreeOfLifeVersion::V2_7 => "ott2.7/ott/synonyms.tsv",
            OpenTreeOfLifeVersion::V2_8 => "ott2.8/ott2.8/synonyms.tsv",
            OpenTreeOfLifeVersion::V2_9 => "ott2.9/ott/synonyms.tsv",
            OpenTreeOfLifeVersion::V2_10 => "ott2.10/ott/synonyms.tsv",
            OpenTreeOfLifeVersion::V3_0 => "ott3.0/ott/synonyms.tsv",
            OpenTreeOfLifeVersion::V3_1 => "ott3.1/ott3.1/synonyms.tsv",
            OpenTreeOfLifeVersion::V3_2 => "ott3.2/ott3.2/synonyms.tsv",
            OpenTreeOfLifeVersion::V3_3 => "ott3.3/ott3.3/synonyms.tsv",
            OpenTreeOfLifeVersion::V3_4 => "ott3.4/ott3.4/synonyms.tsv",
            OpenTreeOfLifeVersion::V3_5 => "ott3.5/ott3.5/synonyms.tsv",
            OpenTreeOfLifeVersion::V3_6 => "ott3.6/ott3.6/synonyms.tsv",
            OpenTreeOfLifeVersion::V3_7 => "ott3.7/ott3.7/synonyms.tsv",
        }
    }
}

impl Ord for OpenTreeOfLifeVersion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.release_date().cmp(&other.release_date())
    }
}

impl PartialOrd for OpenTreeOfLifeVersion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TaxonVersion for OpenTreeOfLifeVersion {
    /// Returns the latest version of the Open Tree of Life taxonomy.
    fn latest() -> OpenTreeOfLifeVersion {
        OpenTreeOfLifeVersion::iter().max().unwrap()
    }
}
