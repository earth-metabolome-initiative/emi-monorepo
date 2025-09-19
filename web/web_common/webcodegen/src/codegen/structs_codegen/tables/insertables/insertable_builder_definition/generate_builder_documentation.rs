//! Submodule defining the documentation for the insertable builder struct for a
//! table.

use quote::quote;

use crate::{
    Table, TableLike,
    codegen::{CODEGEN_INSERTABLES_PATH, CODEGEN_TABLES_PATH},
    errors::WebCodeGenError,
};

impl Table {
    pub(super) fn generate_builder_documentation(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<String>, WebCodeGenError> {
        let ancestral_insertable_columns = self.ancestral_insertable_columns(conn)?;
        let struct_name = self.struct_name()?;
        let snake_case_table_name = self.singular_snake_case_name()?;
        let has_created_by = self.has_created_by_column(true, conn)?;

        // We determine the set of traits which are required
        // to call the setters for the columns.
        let mut required_imports = vec![
            "web_common_traits::database::Insertable".to_owned(),
            "web_common_traits::database::InsertableVariant".to_owned(),
            format!("core_structures::{struct_name}"),
        ];
        for column in &ancestral_insertable_columns {
            let column_table = column.table(conn)?;
            let setter_trait = column_table.setter_trait_name()?;
            required_imports.push(format!(
                "core_structures::{CODEGEN_TABLES_PATH}::{CODEGEN_INSERTABLES_PATH}::{setter_trait}"
            ));
        }

        required_imports.sort_unstable_by_key(|t| quote! { #t }.to_string());
        required_imports.dedup_by_key(|t| quote! { #t }.to_string());

        // We group the columns in 3 groups:
        //
        // 1. Required columns (i.e. non-nullable columns without default values)
        // 2. Columns with default values (i.e. columns which have a SQL default value)
        // 3. Optional columns (i.e. nullable columns without default values)

        let mut mandatory_columns = Vec::new();
        let mut defaulted_columns = Vec::new();
        let mut optional_columns = Vec::new();

        for column in ancestral_insertable_columns {
            if column.has_default() {
                defaulted_columns.push(column);
            } else if column.is_nullable() {
                optional_columns.push(column);
            } else {
                mandatory_columns.push(column);
            }
        }

        // Generate documentation for the builder struct

        let mut documentation = Vec::new();

        documentation.push(format!(
            "Builder for creating and inserting a new [`{}`]({}).",
            self.struct_name()?,
            self.import_struct_path_str()?
        ));
        documentation.push(String::new());
        documentation.push("# Implementation details".to_string());
        documentation.push("While this builder implements several methods, a reasonably complete **basic** usage example (*which may not apply to your own specific use case, please adapt accordingly*) is as follows:".to_string());
        documentation.push(String::new());
        documentation.push("```rust,ignore".to_string());
        for required_import in &required_imports {
            documentation.push(format!("use {required_import};"));
        }

        documentation.push(String::new());
        documentation.push(format!("let {snake_case_table_name} = {struct_name}::new()"));
        if !mandatory_columns.is_empty() {
            documentation.push("    // Set mandatory fields".to_string());
        }
        for column in &mandatory_columns {
            let setter_name = column.getter_name()?;
            let column_name = column.snake_case_name()?;

            if column.is_updated_by(conn)? && has_created_by {
                documentation.push(format!(
                    "    // Note: `{setter_name}` is automatically set by the `created by` column."
                ));
            }

            documentation.push(format!("    .{setter_name}({column_name})?"));
        }
        if !defaulted_columns.is_empty() {
            documentation.push("    // Optionally set fields with default values".to_string());
        }
        for column in &defaulted_columns {
            let setter_name = column.getter_name()?;
            let column_name = column.snake_case_name()?;
            documentation.push(format!("    .{setter_name}({column_name})?"));
        }
        if !optional_columns.is_empty() {
            documentation.push("    // Optionally set optional fields".to_string());
        }
        for column in &optional_columns {
            let setter_name = column.getter_name()?;
            let column_name = column.snake_case_name()?;
            documentation.push(format!("    .{setter_name}({column_name})?"));
        }
        documentation.push("    // Finally, insert the new record in the database".to_string());
        documentation.push("    .insert(user.id, conn)?;".to_string());
        documentation.push("```".to_string());

        Ok(documentation)
    }
}
