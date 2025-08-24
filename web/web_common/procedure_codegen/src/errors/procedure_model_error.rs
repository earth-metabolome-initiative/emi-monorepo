//! Submodule providing an error enumeration for procedure model code
//! generation.

use webcodegen::Table;

#[derive(Debug)]
/// Errors which may occur during procedure model code generation.
pub enum ProcedureModelError {
    /// The specified table is not a valid procedure model table.
    NotAProcedureModelTable(Box<Table>),
}

impl std::fmt::Display for ProcedureModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcedureModelError::NotAProcedureModelTable(table) => {
                write!(f, "Table '{}' is not a valid procedure model table", table.table_name)
            }
        }
    }
}

impl core::error::Error for ProcedureModelError {}
