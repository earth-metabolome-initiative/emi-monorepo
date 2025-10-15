//! Submodule defining traits used to generate Rust code from SQL schema.

mod table;
pub use table::TableSyn;
mod crate_toml;
pub use crate_toml::CrateToml;
mod workspace_toml;
pub use workspace_toml::WorkspaceToml;
