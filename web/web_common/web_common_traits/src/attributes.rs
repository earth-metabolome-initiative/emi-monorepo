//! Attributes relative to special columns of tables and attributes of structs.

/// Trait for a type that has a description.
pub trait Described {
    /// Returns the description of the implementing type.
    fn description(&self) -> Option<&str>;
}
