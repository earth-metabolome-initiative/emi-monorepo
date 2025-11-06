//! Submodule containing metadata structs.

mod table_metadata;
pub use table_metadata::TableMetadata;
mod table_attribute;
pub use table_attribute::TableAttribute;
mod index_metadata;
pub use index_metadata::UniqueIndexMetadata;
mod check_metadata;
pub use check_metadata::CheckMetadata;
