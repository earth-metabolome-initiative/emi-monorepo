//! Submodule providing a trait defining a Taxonomy Version.

/// Trait defining a Taxonomy Version.
pub trait TaxonVersion: Ord {
    /// Returns the latest version of the taxonomy.
    fn latest() -> Self;
}
