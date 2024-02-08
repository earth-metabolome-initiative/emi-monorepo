//! Enrichers are a way to add additional information to the data that is being processed by the pipeline.
//! This can be used to add metadata, or to add additional data that is not present in the original data source.
#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod molecule_classifiers;
/// Prelude module
pub mod prelude {
    pub use crate::molecule_classifiers::*;
}
