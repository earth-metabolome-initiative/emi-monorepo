//! Submodule defining the trait `RequiredCrates` which provides a method to return
//! the crates required by a struct derived from a database table.

/// Trait to be implemented by structs derived from database tables to indicate
/// which crates they require.
pub trait RequiredCrates {}