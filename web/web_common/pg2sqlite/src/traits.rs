//! Submodule for traits used in the translation between `PostgreSQL` and `SQLite`.

pub mod translator;
pub use translator::Translator;
pub mod schema;
pub use schema::Schema;
