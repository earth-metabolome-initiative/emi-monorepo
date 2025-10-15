//! Submodule providing a trait for describing SQL Function-like entities.

/// A trait for describing SQL Function-like entities.
pub trait FunctionLike {
	/// The name of the function.
	///
	/// # Example
	///
	/// ```rust
	/// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
	/// use sql_traits::prelude::*;
	///
	/// let db = ParserDB::try_from(
	///     r#"
	/// CREATE FUNCTION add_one(x INT) RETURNS INT AS 'SELECT x + 1;';
	/// "#,
	/// )?;
	/// let function = db.functions().next().expect("Function should exist");
	/// assert_eq!(function.name(), "add_one");
	/// # Ok(())
	/// # }
	/// ```
	fn name(&self) -> &str;
}