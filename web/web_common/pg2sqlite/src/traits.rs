//! Submodule for traits used in the translation between `PostgreSQL` and
//! `SQLite`.

pub mod translator;
pub use translator::Translator;
pub mod schema;
pub use schema::Schema;
pub mod translation_options;
pub use translation_options::TranslationOptions;
pub mod table_like;
pub use table_like::TableLike;
pub mod column_like;
pub use column_like::ColumnLike;
