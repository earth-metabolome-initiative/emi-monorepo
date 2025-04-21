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
    /// 
    /// # Arguments
    /// 
    /// * `path`: The path where to write out the ranks.
    /// 
    /// # Errors
    /// 
    /// * If an `IOError` occurs while writing out the taxonomy.
    fn to_csv(path: &str) -> Result<(), std::io::Error> {
        let mut writer = csv::Writer::from_path(path)?;
        for rank in Self::iter() {
            writer.serialize(RankRow {
                name: rank.to_string(),
                description: rank.description().to_string(),
            })?;
        }

        writer.flush()?;

        Ok(())
    }

    /// Returns the description of the rank.
    fn description(&self) -> &str;
}
