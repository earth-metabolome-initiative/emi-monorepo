//! Traits to facilitate the generation of `*KeySettable` traits for SynQL
//! buildable models.

pub mod table_buildable_key_settable_like;
pub(crate) use table_buildable_key_settable_like::TRAIT_MODULE_NAME;
pub use table_buildable_key_settable_like::TableBuildableKeySettableLike;
pub mod column_buildable_key_settable_like;
pub use column_buildable_key_settable_like::ColumnBuildableKeySettableLike;
