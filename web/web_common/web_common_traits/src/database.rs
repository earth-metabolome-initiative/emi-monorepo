//! Module with the traits for primitive operation on the database

mod ancestor;
mod deletable;
mod extension;
mod foreign;
mod from_extension;
mod insertable;
mod most_concrete_variant;
mod primary_key_like;
mod read;
mod table_name;
mod tabular;
mod try_insert;
mod updatable;
mod upsertable;

pub use ancestor::{Ancestor, AncestorExists, Descendant};
pub use deletable::{Deletable, DeleteError, DeleteFromVec};
pub use extension::ExtensionTable;
pub use foreign::{ForeignKeys, HasForeignKeys};
pub use from_extension::FromExtension;
#[cfg(feature = "backend")]
pub use insertable::BackendInsertableVariant;
pub use insertable::{
    DispatchableInsertVariantMetadata, DispatchableInsertableVariant, IdOrBuilder, InsertError,
    Insertable, InsertableVariant, InsertableVariantMetadata, MostConcreteTable, SetPrimaryKey,
};
pub use most_concrete_variant::{MostConcreteVariant, MostConcreteVariantMetadata};
pub use primary_key_like::{MaybePrimaryKeyLike, PrimaryKeyLike};
pub use read::{BoundedRead, BoundedReadDispatch, Read, ReadDispatch};
pub use table_name::{HasTableType, TableAttributed, TableEnum, TableField, TableName};
pub use tabular::{Row, Rows, StaticTabular, Tabular};
pub use try_insert::TryInsertGeneric;
pub use updatable::Updatable;
pub use upsertable::{UpsertVec, Upsertable};
