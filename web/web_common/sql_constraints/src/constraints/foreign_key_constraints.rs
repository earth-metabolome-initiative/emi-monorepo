//! Submodule providing constraint structs that can be applied to foreign keys.

mod compatible_foreign_key;
pub use compatible_foreign_key::CompatibleForeignKey;
mod lowercase_foreign_key_name;
pub use lowercase_foreign_key_name::LowercaseForeignKeyName;
mod references_unique_index;
pub use references_unique_index::ReferencesUniqueIndex;
