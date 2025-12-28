#![doc = include_str!("../README.md")]

pub mod database;
pub mod impls;
pub mod models;
pub mod schema;
pub mod traits;
pub use database::PgDatabase;
pub mod model_metadata;

/// Prelude module re-exporting commonly used items.
pub mod prelude {
	pub use crate::database::*;
	pub use crate::traits::*;
	pub use crate::models::*;
	pub use sql_traits::prelude::*;
}