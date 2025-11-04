//! Traits to facilitate the generation of `*ValueSettable` traits for SynQL
//! models.

pub mod table_value_settable_like;
pub(crate) use table_value_settable_like::TRAIT_MODULE_NAME;
pub use table_value_settable_like::TableValueSettableLike;
