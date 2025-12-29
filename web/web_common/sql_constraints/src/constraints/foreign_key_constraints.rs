//! Submodule providing constraint structs that can be applied to foreign keys.

mod compatible_foreign_key;
pub use compatible_foreign_key::CompatibleForeignKey;
mod lowercase_foreign_key_name;
pub use lowercase_foreign_key_name::LowercaseForeignKeyName;
mod no_rust_keyword_foreign_key_name;
pub use no_rust_keyword_foreign_key_name::NoRustKeywordForeignKeyName;
mod references_unique_index;
pub use references_unique_index::ReferencesUniqueIndex;
mod primary_key_reference_ends_with_id;
pub use primary_key_reference_ends_with_id::PrimaryKeyReferenceEndsWithId;
mod extension_foreign_key_on_delete_cascade;
pub use extension_foreign_key_on_delete_cascade::ExtensionForeignKeyOnDeleteCascade;
