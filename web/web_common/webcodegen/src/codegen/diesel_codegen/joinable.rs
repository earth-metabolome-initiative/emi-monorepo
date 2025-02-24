//! Submodule implementing code relative to diesel's [`joinable`](https://docs.rs/diesel/latest/diesel/macro.joinable.html) macro.


use std::path::Path;

use diesel::PgConnection;

use crate::Table;

use super::Codegen;

impl<'a> Codegen<'a> {
    /// Generate implementations of the `joinable` diesel macro.
	/// 
	/// # Arguments
	/// 
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
	/// * `conn` - A mutable reference to a `PgConnection`.
	/// 
    pub(crate) fn generate_joinable_macro(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
		// https://github.com/earth-metabolome-initiative/emi-monorepo/issues/36
		todo!("See issue #36")
    }
}
