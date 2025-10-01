//! Submodule providing a trait for describing SQL Table-like entities.

/// A trait for types that can be treated as SQL tables.
pub trait TableLike {
	/// Returns the name of the table.
	fn table_name(&self) -> &str;
}
