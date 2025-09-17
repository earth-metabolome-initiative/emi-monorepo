//! Submodule providing an error enumeration for procedure template code
//! generation.

use webcodegen::Table;

use crate::utils::PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME;

#[derive(Debug)]
/// Errors which may occur during procedure template code generation.
pub enum ProcedureTemplateError {
    /// The specified table is not a valid procedure template table.
    NotAProcedureTemplateTable(Box<Table>),
    /// No procedure associated with the procedure template table was found.
    NoProcedure(Box<Table>),
    /// Multiple procedures associated with the procedure template table were
    /// found.
    MultipleProcedures(Box<Table>, Vec<Table>),
    /// A column which should be a foreign key to `procedure template asset
    /// model` is not.
    ExpectedProcedureTemplateAssetModelForeignKey {
        /// The column which should be a foreign key to `procedure template
        /// asset model`.
        column: Box<webcodegen::Column>,
    },
}

impl std::fmt::Display for ProcedureTemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcedureTemplateError::NotAProcedureTemplateTable(table) => {
                write!(f, "Table '{}' is not a valid procedure template table", table.table_name)
            }
            ProcedureTemplateError::NoProcedure(table) => {
                write!(
                    f,
                    "No procedure associated with procedure template table `{}.{}`",
                    table.table_schema, table.table_name
                )
            }
            ProcedureTemplateError::MultipleProcedures(table, procedures) => {
                let procedure_names: Vec<String> = procedures
                    .iter()
                    .map(|p| format!("`{}.{}`", p.table_schema, p.table_name))
                    .collect();
                write!(
                    f,
                    "Multiple procedures ({}) associated with procedure template table `{}.{}`",
                    procedure_names.join(", "),
                    table.table_schema,
                    table.table_name
                )
            }
            ProcedureTemplateError::ExpectedProcedureTemplateAssetModelForeignKey { column } => {
                write!(
                    f,
                    "Column {column} is expected to be a foreign key to `{PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME}`",
                )
            }
        }
    }
}

impl core::error::Error for ProcedureTemplateError {}
