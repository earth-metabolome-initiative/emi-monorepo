//! Submodule providing an error enumeration for procedure code generation.

use webcodegen::Table;

#[derive(Debug)]
/// Errors which may occur during procedure code generation.
pub enum ProcedureError {
    /// The specified table is not a valid procedure table.
    NotAProcedureTable(Box<Table>),
}

impl std::fmt::Display for ProcedureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcedureError::NotAProcedureTable(table) => {
                write!(
                    f,
                    "Table `{}.{}` is not a valid procedure table",
                    table.table_schema, table.table_name
                )
            }
        }
    }
}

impl core::error::Error for ProcedureError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        None
    }
}
