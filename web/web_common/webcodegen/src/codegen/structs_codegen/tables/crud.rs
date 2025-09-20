//! Submodule providing the code generation for CRUD operations.

mod row;
mod rows;
mod table_names;
mod table_primary_keys;
pub(crate) use table_names::table_names_enum_path;
