//! Submodule defining structs used to generate Rust code from SQL schema.

mod synql;
pub use synql::{SynQL, SynQLBuilder};
mod workspace;
pub use workspace::{Workspace, WorkspaceBuilder};
mod external_crate;
pub use external_crate::{ExternalCrate, ExternalCrateBuilder};
mod external_type;
pub use external_type::{ExternalType, ExternalTypeBuilder};
