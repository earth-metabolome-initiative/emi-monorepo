//! Submodule defining traits which describe different properties of structs derived
//! from database tables.

mod supports;
pub use supports::Supports;
mod rust_type;