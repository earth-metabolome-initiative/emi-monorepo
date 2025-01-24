//! Submodule defining the versions of the NCBI taxonomy.

use crate::traits::TaxonVersion;
use chrono::NaiveDateTime;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
/// Enum representing the versions of the NCBI taxonomy.
pub enum NCBIVersion {
    /// The 2018_12_01 version of the NCBI taxonomy.
    V2018_12_01,
    /// The 2019_01_01 version of the NCBI taxonomy.
    V2019_01_01,
    /// The 2019_02_01 version of the NCBI taxonomy.
    V2019_02_01,
    /// The 2019_03_01 version of the NCBI taxonomy.
    V2019_03_01,
    /// The 2019_04_01 version of the NCBI taxonomy.
    V2019_04_01,
    /// The 2019_05_01 version of the NCBI taxonomy.
    V2019_05_01,
    /// The 2019_06_01 version of the NCBI taxonomy.
    V2019_06_01,
    /// The 2019_07_01 version of the NCBI taxonomy.
    V2019_07_01,
    /// The 2019_08_01 version of the NCBI taxonomy.
    V2019_08_01,
    /// The 2019_09_01 version of the NCBI taxonomy.
    V2019_09_01,
    /// The 2019_10_01 version of the NCBI taxonomy.
    V2019_10_01,
    /// The 2019_11_01 version of the NCBI taxonomy.
    V2019_11_01,
    /// The 2019_12_01 version of the NCBI taxonomy.
    V2019_12_01,
    /// The 2020_01_01 version of the NCBI taxonomy.
    V2020_01_01,
    /// The 2020_02_01 version of the NCBI taxonomy.
    V2020_02_01,
    /// The 2020_03_01 version of the NCBI taxonomy.
    V2020_03_01,
    /// The 2020_04_01 version of the NCBI taxonomy.
    V2020_04_01,
    /// The 2020_05_01 version of the NCBI taxonomy.
    V2020_05_01,
    /// The 2020_05_31 version of the NCBI taxonomy.
    V2020_05_31,
    /// The 2020_07_01 version of the NCBI taxonomy.
    V2020_07_01,
    /// The 2020_08_01 version of the NCBI taxonomy.
    V2020_08_01,
    /// The 2020_09_01 version of the NCBI taxonomy.
    V2020_09_01,
    /// The 2020_10_01 version of the NCBI taxonomy.
    V2020_10_01,
    /// The 2020_11_01 version of the NCBI taxonomy.
    V2020_11_01,
    /// The 2020_12_01 version of the NCBI taxonomy.
    V2020_12_01,
    /// The 2021_01_01 version of the NCBI taxonomy.
    V2021_01_01,
    /// The 2021_02_01 version of the NCBI taxonomy.
    V2021_02_01,
    /// The 2021_03_01 version of the NCBI taxonomy.
    V2021_03_01,
    /// The 2021_04_01 version of the NCBI taxonomy.
    V2021_04_01,
    /// The 2021_05_01 version of the NCBI taxonomy.
    V2021_05_01,
    /// The 2021_06_01 version of the NCBI taxonomy.
    V2021_06_01,
    /// The 2021_07_01 version of the NCBI taxonomy.
    V2021_07_01,
    /// The 2021_08_01 version of the NCBI taxonomy.
    V2021_08_01,
    /// The 2021_09_01 version of the NCBI taxonomy.
    V2021_09_01,
    /// The 2021_10_01 version of the NCBI taxonomy.
    V2021_10_01,
    /// The 2021_11_01 version of the NCBI taxonomy.
    V2021_11_01,
    /// The 2021_12_01 version of the NCBI taxonomy.
    V2021_12_01,
    /// The 2022_01_01 version of the NCBI taxonomy.
    V2022_01_01,
    /// The 2022_02_01 version of the NCBI taxonomy.
    V2022_02_01,
    /// The 2022_03_01 version of the NCBI taxonomy.
    V2022_03_01,
    /// The 2022_04_01 version of the NCBI taxonomy.
    V2022_04_01,
    /// The 2022_05_01 version of the NCBI taxonomy.
    V2022_05_01,
    /// The 2022_06_01 version of the NCBI taxonomy.
    V2022_06_01,
    /// The 2022_07_01 version of the NCBI taxonomy.
    V2022_07_01,
    /// The 2022_08_01 version of the NCBI taxonomy.
    V2022_08_01,
    /// The 2022_09_01 version of the NCBI taxonomy.
    V2022_09_01,
    /// The 2022_10_01 version of the NCBI taxonomy.
    V2022_10_01,
    /// The 2022_11_01 version of the NCBI taxonomy.
    V2022_11_01,
    /// The 2022_12_01 version of the NCBI taxonomy.
    V2022_12_01,
    /// The 2023_01_01 version of the NCBI taxonomy.
    V2023_01_01,
    /// The 2023_02_01 version of the NCBI taxonomy.
    V2023_02_01,
    /// The 2023_03_01 version of the NCBI taxonomy.
    V2023_03_01,
    /// The 2023_04_01 version of the NCBI taxonomy.
    V2023_04_01,
    /// The 2023_05_01 version of the NCBI taxonomy.
    V2023_05_01,
    /// The 2023_06_01 version of the NCBI taxonomy.
    V2023_06_01,
    /// The 2023_07_01 version of the NCBI taxonomy.
    V2023_07_01,
    /// The 2023_08_01 version of the NCBI taxonomy.
    V2023_08_01,
    /// The 2023_09_01 version of the NCBI taxonomy.
    V2023_09_01,
    /// The 2023_10_01 version of the NCBI taxonomy.
    V2023_10_01,
    /// The 2023_11_01 version of the NCBI taxonomy.
    V2023_11_01,
    /// The 2023_12_01 version of the NCBI taxonomy.
    V2023_12_01,
    /// The 2024_01_01 version of the NCBI taxonomy.
    V2024_01_01,
    /// The 2024_02_01 version of the NCBI taxonomy.
    V2024_02_01,
    /// The 2024_03_01 version of the NCBI taxonomy.
    V2024_03_01,
    /// The 2024_04_01 version of the NCBI taxonomy.
    V2024_04_01,
    /// The 2024_05_01 version of the NCBI taxonomy.
    V2024_05_01,
    /// The 2024_06_01 version of the NCBI taxonomy.
    V2024_06_01,
    /// The 2024_07_01 version of the NCBI taxonomy.
    V2024_07_01,
    /// The 2024_08_01 version of the NCBI taxonomy.
    V2024_08_01,
    /// The 2024_09_01 version of the NCBI taxonomy.
    V2024_09_01,
    /// The 2024_10_01 version of the NCBI taxonomy.
    V2024_10_01,
    /// The 2024_11_01 version of the NCBI taxonomy.
    V2024_11_01,
    /// The 2024_12_01 version of the NCBI taxonomy.
    V2024_12_01,
    /// The 2025_01_01 version of the NCBI taxonomy.
    V2025_01_01,
}

impl NCBIVersion {
    /// Returns the release date of the version.
    pub fn release_date(&self) -> NaiveDateTime {
        match self {
            NCBIVersion::V2018_12_01 => {
                NaiveDateTime::parse_from_str("2018-11-30 23:34", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2019_01_01 => {
                NaiveDateTime::parse_from_str("2018-12-31 23:37", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2019_02_01 => {
                NaiveDateTime::parse_from_str("2019-01-31 23:37", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2019_03_01 => {
                NaiveDateTime::parse_from_str("2019-02-28 23:37", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2019_04_01 => {
                NaiveDateTime::parse_from_str("2019-03-31 23:34", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2019_05_01 => {
                NaiveDateTime::parse_from_str("2019-04-30 23:34", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2019_06_01 => {
                NaiveDateTime::parse_from_str("2019-05-31 23:34", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2019_07_01 => {
                NaiveDateTime::parse_from_str("2019-06-30 23:35", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2019_08_01 => {
                NaiveDateTime::parse_from_str("2019-07-31 23:37", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2019_09_01 => {
                NaiveDateTime::parse_from_str("2019-08-31 23:37", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2019_10_01 => {
                NaiveDateTime::parse_from_str("2019-09-30 23:36", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2019_11_01 => {
                NaiveDateTime::parse_from_str("2019-10-31 23:37", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2019_12_01 => {
                NaiveDateTime::parse_from_str("2019-11-30 23:37", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2020_01_01 => {
                NaiveDateTime::parse_from_str("2019-12-31 23:37", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2020_02_01 => {
                NaiveDateTime::parse_from_str("2020-01-31 23:38", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2020_03_01 => {
                NaiveDateTime::parse_from_str("2020-02-29 23:37", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2020_04_01 => {
                NaiveDateTime::parse_from_str("2020-03-31 23:38", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2020_05_01 => {
                NaiveDateTime::parse_from_str("2020-04-30 23:37", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2020_05_31 => {
                NaiveDateTime::parse_from_str("2020-05-31 23:37", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2020_07_01 => {
                NaiveDateTime::parse_from_str("2020-06-30 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2020_08_01 => {
                NaiveDateTime::parse_from_str("2020-07-31 23:41", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2020_09_01 => {
                NaiveDateTime::parse_from_str("2020-08-31 23:39", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2020_10_01 => {
                NaiveDateTime::parse_from_str("2020-09-30 23:39", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2020_11_01 => {
                NaiveDateTime::parse_from_str("2020-10-31 23:39", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2020_12_01 => {
                NaiveDateTime::parse_from_str("2020-11-30 23:39", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2021_01_01 => {
                NaiveDateTime::parse_from_str("2020-12-31 23:39", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2021_02_01 => {
                NaiveDateTime::parse_from_str("2021-01-31 23:39", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2021_03_01 => {
                NaiveDateTime::parse_from_str("2021-02-28 23:39", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2021_04_01 => {
                NaiveDateTime::parse_from_str("2021-03-31 23:39", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2021_05_01 => {
                NaiveDateTime::parse_from_str("2021-04-30 23:39", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2021_06_01 => {
                NaiveDateTime::parse_from_str("2021-05-31 23:39", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2021_07_01 => {
                NaiveDateTime::parse_from_str("2021-06-30 23:39", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2021_08_01 => {
                NaiveDateTime::parse_from_str("2021-07-31 23:39", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2021_09_01 => {
                NaiveDateTime::parse_from_str("2021-08-31 23:39", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2021_10_01 => {
                NaiveDateTime::parse_from_str("2021-09-30 23:39", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2021_11_01 => {
                NaiveDateTime::parse_from_str("2021-10-31 23:39", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2021_12_01 => {
                NaiveDateTime::parse_from_str("2021-11-30 23:39", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2022_01_01 => {
                NaiveDateTime::parse_from_str("2021-12-31 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2022_02_01 => {
                NaiveDateTime::parse_from_str("2022-01-31 23:39", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2022_03_01 => {
                NaiveDateTime::parse_from_str("2022-02-28 23:39", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2022_04_01 => {
                NaiveDateTime::parse_from_str("2022-03-31 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2022_05_01 => {
                NaiveDateTime::parse_from_str("2022-04-30 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2022_06_01 => {
                NaiveDateTime::parse_from_str("2022-05-31 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2022_07_01 => {
                NaiveDateTime::parse_from_str("2022-06-30 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2022_08_01 => {
                NaiveDateTime::parse_from_str("2022-07-31 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2022_09_01 => {
                NaiveDateTime::parse_from_str("2022-08-31 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2022_10_01 => {
                NaiveDateTime::parse_from_str("2022-09-30 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2022_11_01 => {
                NaiveDateTime::parse_from_str("2022-10-31 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2022_12_01 => {
                NaiveDateTime::parse_from_str("2022-11-30 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2023_01_01 => {
                NaiveDateTime::parse_from_str("2022-12-31 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2023_02_01 => {
                NaiveDateTime::parse_from_str("2023-01-31 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2023_03_01 => {
                NaiveDateTime::parse_from_str("2023-02-28 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2023_04_01 => {
                NaiveDateTime::parse_from_str("2023-03-31 23:39", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2023_05_01 => {
                NaiveDateTime::parse_from_str("2023-04-30 23:39", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2023_06_01 => {
                NaiveDateTime::parse_from_str("2023-05-31 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2023_07_01 => {
                NaiveDateTime::parse_from_str("2023-06-30 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2023_08_01 => {
                NaiveDateTime::parse_from_str("2023-07-31 23:39", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2023_09_01 => {
                NaiveDateTime::parse_from_str("2023-08-31 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2023_10_01 => {
                NaiveDateTime::parse_from_str("2023-09-30 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2023_11_01 => {
                NaiveDateTime::parse_from_str("2023-10-31 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2023_12_01 => {
                NaiveDateTime::parse_from_str("2023-11-30 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2024_01_01 => {
                NaiveDateTime::parse_from_str("2023-12-31 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2024_02_01 => {
                NaiveDateTime::parse_from_str("2024-01-31 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2024_03_01 => {
                NaiveDateTime::parse_from_str("2024-02-29 23:39", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2024_04_01 => {
                NaiveDateTime::parse_from_str("2024-03-31 23:39", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2024_05_01 => {
                NaiveDateTime::parse_from_str("2024-04-30 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2024_06_01 => {
                NaiveDateTime::parse_from_str("2024-05-31 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2024_07_01 => {
                NaiveDateTime::parse_from_str("2024-06-30 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2024_08_01 => {
                NaiveDateTime::parse_from_str("2024-07-31 23:41", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2024_09_01 => {
                NaiveDateTime::parse_from_str("2024-08-31 23:41", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2024_10_01 => {
                NaiveDateTime::parse_from_str("2024-09-30 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2024_11_01 => {
                NaiveDateTime::parse_from_str("2024-10-31 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2024_12_01 => {
                NaiveDateTime::parse_from_str("2024-11-30 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
            NCBIVersion::V2025_01_01 => {
                NaiveDateTime::parse_from_str("2024-12-31 23:40", "%Y-%m-%d %H:%M").unwrap()
            }
        }
    }

    /// Returns the URL associated with the version.
    pub fn url(&self) -> &str {
        match self {
            NCBIVersion::V2018_12_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2018-12-01.zip",
            NCBIVersion::V2019_01_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2019-01-01.zip",
            NCBIVersion::V2019_02_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2019-02-01.zip",
            NCBIVersion::V2019_03_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2019-03-01.zip",
            NCBIVersion::V2019_04_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2019-04-01.zip",
            NCBIVersion::V2019_05_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2019-05-01.zip",
            NCBIVersion::V2019_06_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2019-06-01.zip",
            NCBIVersion::V2019_07_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2019-07-01.zip",
            NCBIVersion::V2019_08_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2019-08-01.zip",
            NCBIVersion::V2019_09_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2019-09-01.zip",
            NCBIVersion::V2019_10_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2019-10-01.zip",
            NCBIVersion::V2019_11_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2019-11-01.zip",
            NCBIVersion::V2019_12_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2019-12-01.zip",
            NCBIVersion::V2020_01_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2020-01-01.zip",
            NCBIVersion::V2020_02_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2020-02-01.zip",
            NCBIVersion::V2020_03_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2020-03-01.zip",
            NCBIVersion::V2020_04_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2020-04-01.zip",
            NCBIVersion::V2020_05_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2020-05-01.zip",
            NCBIVersion::V2020_05_31 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2020-05-31.zip",
            NCBIVersion::V2020_07_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2020-07-01.zip",
            NCBIVersion::V2020_08_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2020-08-01.zip",
            NCBIVersion::V2020_09_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2020-09-01.zip",
            NCBIVersion::V2020_10_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2020-10-01.zip",
            NCBIVersion::V2020_11_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2020-11-01.zip",
            NCBIVersion::V2020_12_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2020-12-01.zip",
            NCBIVersion::V2021_01_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2021-01-01.zip",
            NCBIVersion::V2021_02_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2021-02-01.zip",
            NCBIVersion::V2021_03_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2021-03-01.zip",
            NCBIVersion::V2021_04_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2021-04-01.zip",
            NCBIVersion::V2021_05_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2021-05-01.zip",
            NCBIVersion::V2021_06_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2021-06-01.zip",
            NCBIVersion::V2021_07_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2021-07-01.zip",
            NCBIVersion::V2021_08_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2021-08-01.zip",
            NCBIVersion::V2021_09_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2021-09-01.zip",
            NCBIVersion::V2021_10_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2021-10-01.zip",
            NCBIVersion::V2021_11_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2021-11-01.zip",
            NCBIVersion::V2021_12_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2021-12-01.zip",
            NCBIVersion::V2022_01_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2022-01-01.zip",
            NCBIVersion::V2022_02_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2022-02-01.zip",
            NCBIVersion::V2022_03_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2022-03-01.zip",
            NCBIVersion::V2022_04_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2022-04-01.zip",
            NCBIVersion::V2022_05_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2022-05-01.zip",
            NCBIVersion::V2022_06_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2022-06-01.zip",
            NCBIVersion::V2022_07_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2022-07-01.zip",
            NCBIVersion::V2022_08_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2022-08-01.zip",
            NCBIVersion::V2022_09_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2022-09-01.zip",
            NCBIVersion::V2022_10_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2022-10-01.zip",
            NCBIVersion::V2022_11_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2022-11-01.zip",
            NCBIVersion::V2022_12_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2022-12-01.zip",
            NCBIVersion::V2023_01_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2023-01-01.zip",
            NCBIVersion::V2023_02_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2023-02-01.zip",
            NCBIVersion::V2023_03_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2023-03-01.zip",
            NCBIVersion::V2023_04_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2023-04-01.zip",
            NCBIVersion::V2023_05_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2023-05-01.zip",
            NCBIVersion::V2023_06_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2023-06-01.zip",
            NCBIVersion::V2023_07_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2023-07-01.zip",
            NCBIVersion::V2023_08_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2023-08-01.zip",
            NCBIVersion::V2023_09_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2023-09-01.zip",
            NCBIVersion::V2023_10_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2023-10-01.zip",
            NCBIVersion::V2023_11_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2023-11-01.zip",
            NCBIVersion::V2023_12_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2023-12-01.zip",
            NCBIVersion::V2024_01_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2024-01-01.zip",
            NCBIVersion::V2024_02_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2024-02-01.zip",
            NCBIVersion::V2024_03_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2024-03-01.zip",
            NCBIVersion::V2024_04_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2024-04-01.zip",
            NCBIVersion::V2024_05_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2024-05-01.zip",
            NCBIVersion::V2024_06_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2024-06-01.zip",
            NCBIVersion::V2024_07_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2024-07-01.zip",
            NCBIVersion::V2024_08_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2024-08-01.zip",
            NCBIVersion::V2024_09_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2024-09-01.zip",
            NCBIVersion::V2024_10_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2024-10-01.zip",
            NCBIVersion::V2024_11_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2024-11-01.zip",
            NCBIVersion::V2024_12_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2024-12-01.zip",
            NCBIVersion::V2025_01_01 => "https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/taxdump_archive/new_taxdump_2025-01-01.zip",
        }
    }
    /// Returns the name of the directory containing the version.
    pub fn directory(&self) -> String {
        self.url()
            .split('/')
            .last()
            .unwrap()
            .split('.')
            .next()
            .unwrap()
            .to_owned()
    }
}

impl Ord for NCBIVersion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.release_date().cmp(&other.release_date())
    }
}

impl PartialOrd for NCBIVersion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TaxonVersion for NCBIVersion {
    /// Returns the latest version of the NCBI taxonomy.
    fn latest() -> NCBIVersion {
        NCBIVersion::iter().max().unwrap()
    }
}
