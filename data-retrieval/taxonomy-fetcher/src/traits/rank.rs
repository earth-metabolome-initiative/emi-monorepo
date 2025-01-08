//! Submodule defining the Rank trait for taxonomic ranks.

use std::{fmt::{Debug, Display}, hash::Hash, str::FromStr};

/// Trait defining a taxonomic rank.
pub trait Rank: Display + Debug+ Hash + Copy + Clone + Eq + PartialEq + FromStr {}
