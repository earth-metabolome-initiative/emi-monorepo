//! Traits to facilitate the generation of `*KeySettable` traits for SynQL
//! models.

pub mod table_insertable_key_settable_like;
pub(crate) use table_insertable_key_settable_like::TRAIT_MODULE_NAME;
pub use table_insertable_key_settable_like::TableInsertableKeySettableLike;
pub mod column_insertable_key_settable_like;
pub use column_insertable_key_settable_like::ColumnInsertableKeySettableLike;
