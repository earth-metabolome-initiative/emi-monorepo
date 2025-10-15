//! Submodule defining structs used throughout the crate.

mod internal_crate;
pub use internal_crate::InternalCrate;
mod workspace;
pub use workspace::Workspace;
mod struct_model;
pub use struct_model::Struct;
mod module_model;
pub use module_model::Module;
mod publicness;
pub use publicness::Publicness;
mod external_crate;
pub use external_crate::ExternalCrate;
mod external_type;
pub use external_type::ExternalType;
