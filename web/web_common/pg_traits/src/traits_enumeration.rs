//! Submodule providing an enumeration of traits which may be supported by the
//! structs derived from database tables.

/// Enumeration of traits which may be supported by the structs derived from
/// database tables.
pub enum TraitEnum {
	/// Whether the struct can implement `Copy`.
	Copy,
	/// Whether the struct can implement `Clone`.
	Clone
}