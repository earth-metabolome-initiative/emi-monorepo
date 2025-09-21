//! Submodule defining traits which describe different properties of structs derived
//! from database tables.

mod diesel_type;
pub use diesel_type::DieselType;
mod required_crates;
pub use required_crates::RequiredCrates;
mod supports;
pub use supports::Supports;
mod rust_type;
pub use rust_type::RustType;