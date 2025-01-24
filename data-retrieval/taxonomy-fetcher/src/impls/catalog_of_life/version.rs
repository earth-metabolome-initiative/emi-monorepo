//! Submodule defining the versions of the Catalog of Life taxonomy.

use crate::traits::TaxonVersion;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
/// Enum representing the versions of the Catalog of Life taxonomy.
pub enum CatalogOfLifeVersion {
	/// The version 2013 of the Catalog of Life taxonomy.
	V2013,
	/// The version 2014 of the Catalog of Life taxonomy.
	V2014,
	/// The version 2015 of the Catalog of Life taxonomy.
	V2015,
	/// The version 2016 of the Catalog of Life taxonomy.
	V2016,
	/// The version 2017 of the Catalog of Life taxonomy.
	V2017,
	/// The version 2018 of the Catalog of Life taxonomy.
	V2018,
	/// The version 2019 of the Catalog of Life taxonomy.
	V2019,
	/// The version 2020 of the Catalog of Life taxonomy.
	V2020,
	/// The version 2021 of the Catalog of Life taxonomy.
	V2021,
	/// The version 2022 of the Catalog of Life taxonomy.
	V2022,
	/// The version 2023 of the Catalog of Life taxonomy.
	V2023,
	/// The version 2024 of the Catalog of Life taxonomy.
    V2024,
}

impl CatalogOfLifeVersion {
    /// Returns the release date of the version.
    pub fn year(&self) -> u16 {
        match self {
			CatalogOfLifeVersion::V2013 => 2013,
			CatalogOfLifeVersion::V2014 => 2014,
			CatalogOfLifeVersion::V2015 => 2015,
			CatalogOfLifeVersion::V2016 => 2016,
			CatalogOfLifeVersion::V2017 => 2017,
			CatalogOfLifeVersion::V2018 => 2018,
			CatalogOfLifeVersion::V2019 => 2019,
			CatalogOfLifeVersion::V2020 => 2020,
			CatalogOfLifeVersion::V2021 => 2021,
			CatalogOfLifeVersion::V2022 => 2022,
			CatalogOfLifeVersion::V2023 => 2023,
			CatalogOfLifeVersion::V2024 => 2024,
		}
    }

    /// Returns the URL associated with the version.
    pub fn url(&self) -> &str {
        match self {
			CatalogOfLifeVersion::V2013 => "https://download.checklistbank.org/col/annual/2013_dwca.zip",
			CatalogOfLifeVersion::V2014 => "https://download.checklistbank.org/col/annual/2014_dwca.zip",
			CatalogOfLifeVersion::V2015 => "https://download.checklistbank.org/col/annual/2015_dwca.zip",
			CatalogOfLifeVersion::V2016 => "https://download.checklistbank.org/col/annual/2016_dwca.zip",
			CatalogOfLifeVersion::V2017 => "https://download.checklistbank.org/col/annual/2017_dwca.zip",
			CatalogOfLifeVersion::V2018 => "https://download.checklistbank.org/col/annual/2018_dwca.zip",
			CatalogOfLifeVersion::V2019 => "https://download.checklistbank.org/col/annual/2019_dwca.zip",
			CatalogOfLifeVersion::V2020 => "https://download.checklistbank.org/col/annual/2020_dwca.zip",
			CatalogOfLifeVersion::V2021 => "https://download.checklistbank.org/col/annual/2021_dwca.zip",
			CatalogOfLifeVersion::V2022 => "https://download.checklistbank.org/col/annual/2022_dwca.zip",
			CatalogOfLifeVersion::V2023 => "https://download.checklistbank.org/col/annual/2023_dwca.zip",
            CatalogOfLifeVersion::V2024 => "https://download.checklistbank.org/col/annual/2024_dwca.zip",
        }
    }

    /// Returns the path to the taxonomy file in the archive.
    pub fn taxonomy_file(&self) -> &str {
        match self {
            CatalogOfLifeVersion::V2024 => "2024_dwca/Taxon.tsv",
			_ => panic!("Taxonomy file not available for version {:?}", self),
        }
    }
}

impl Ord for CatalogOfLifeVersion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.year().cmp(&other.year())
    }
}

impl PartialOrd for CatalogOfLifeVersion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TaxonVersion for CatalogOfLifeVersion {
    /// Returns the latest version of the Catalog of Life taxonomy.
    fn latest() -> CatalogOfLifeVersion {
        CatalogOfLifeVersion::iter().max().unwrap()
    }
}
