//! Submodule providing primary structs and methods for the Directus migration tool.
pub mod codegen;
pub mod error;
mod migrations;
pub use migrations::insert_missing_brands;
pub use migrations::insert_missing_users;