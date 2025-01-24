//! Submodule defining a trait for an identifier of a taxon.
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::str::FromStr;

use serde::Serialize;

/// Trait defining an identifier of a taxon.
pub trait TaxonIdentifier:
    Display + Debug + ToString + FromStr + PartialEq + Eq + Clone + Copy + Hash + Serialize
{
}

impl TaxonIdentifier for u32 {}
