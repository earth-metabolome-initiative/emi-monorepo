//! Submodule providing methods to generate Roles tables for all editable tables.
//!
//! Editable tables are the ones characterized by created_by and updated_by columns.

use crate::Table;

impl Table {
    fn requires_roles_table(&self) {
        
    }
}