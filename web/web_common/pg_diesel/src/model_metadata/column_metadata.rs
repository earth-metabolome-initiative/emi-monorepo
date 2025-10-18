//! Submodule providing the `ColumnMetadata` struct for a [`Column`] model.

use std::rc::Rc;

use crate::models::{PgDescription, Table};

#[derive(Clone, Debug)]
/// Metadata about a Postgres column.
pub struct ColumnMetadata {
    /// The table the column belongs to.
    table: Rc<Table>,
    /// The description of the column, if any.
    description: Option<PgDescription>,
}

impl ColumnMetadata {
    /// Creates a new `ColumnMetadata` instance.
    pub fn new(table: Rc<Table>, description: Option<PgDescription>) -> Self {
        Self { table, description }
    }

    /// Returns the table the column belongs to.
    pub fn table(&self) -> &Table {
        self.table.as_ref()
    }

    /// Returns the description of the column, if any.
    pub fn description(&self) -> Option<&PgDescription> {
        self.description.as_ref()
    }
}
