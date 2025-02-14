//! Submodule defining the Rank trait for taxonomic ranks.

use std::{
    fmt::{Debug, Display},
    hash::Hash,
    str::FromStr,
};

use serde::Serialize;
use strum::IntoEnumIterator;

#[derive(Debug, Serialize)]
struct RankRow {
    name: String,
    description: String,
}

/// Trait defining a taxonomic rank.
pub trait Rank:
    Display + Debug + Hash + Copy + Clone + Eq + PartialEq + FromStr + Serialize + IntoEnumIterator
{
    /// Writes out the ranks as a CSV.
    fn to_csv(path: &str) -> Result<(), std::io::Error> {
        let mut writer = csv::Writer::from_path(path)?;
        for rank in Self::iter() {
            writer.serialize(RankRow {
                name: rank.to_string(),
                description: rank.description().to_string(),
            })?;
        }
        Ok(())
    }

    /// Returns the description of the rank.
    fn description(&self) -> &str;
}
