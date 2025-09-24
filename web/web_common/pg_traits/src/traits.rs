//! Submodule defining traits which describe different properties of structs
//! derived from database tables.

mod supports;
pub use supports::Supports;
mod associated_type;
pub use associated_type::AssociatedType;
