//! Submodule implementing code relative to diesel's [`allow_tables_to_appear_in_same_query`](https://docs.rs/diesel/latest/diesel/macro.allow_tables_to_appear_in_same_query.html) macro.

use std::path::Path;

use diesel::PgConnection;

use crate::Table;

use super::Codegen;

impl<'a> Codegen<'a> {
    /// Generate implementations of the `allow_tables_to_appear_in_same_query` diesel macro.
	/// 
	/// # Arguments
	/// 
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
	/// * `conn` - A mutable reference to a `PgConnection`.
	/// 
    pub(crate) fn generate_allow_tables_to_appear_in_same_query(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        // https://github.com/earth-metabolome-initiative/emi-monorepo/issues/37
		todo!("See issue #37")
    }
}
