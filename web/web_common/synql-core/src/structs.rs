//! Submodule defining structs used throughout the crate.

mod internal_crate;
pub use internal_crate::InternalCrate;
mod workspace;
pub use workspace::Workspace;
mod internal_struct;
pub use internal_struct::InternalStruct;
mod internal_module;
pub use internal_module::InternalModule;
mod internal_enum;
pub use internal_enum::InternalEnum;
mod internal_data;
pub use internal_data::{InternalData, InternalDataRef, InternalModuleRef};
mod publicness;
pub use publicness::Publicness;
mod external_crate;
pub use external_crate::ExternalCrate;
mod external_type;
pub use external_type::ExternalType;
mod external_macro;
pub use external_macro::ExternalMacro;
mod internal_token;
pub use internal_token::InternalToken;
mod external_trait;
pub use external_trait::ExternalTrait;
