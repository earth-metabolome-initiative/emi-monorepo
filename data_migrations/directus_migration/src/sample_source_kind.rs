//! Submodule enumerating the different sample_source kinds we can encounter in the Directus db

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleSourceKind {
    Organism,
    Soil,
}