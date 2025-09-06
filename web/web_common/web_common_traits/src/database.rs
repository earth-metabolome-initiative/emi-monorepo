//! Module with the traits for primitive operation on the database

mod ancestor;
mod connection;
mod deletable;
mod extension;
mod foreign;
mod insertable;
mod read;
mod table_name;
mod tabular;
mod try_insert;
mod updatable;
mod upsertable;

pub use ancestor::{Ancestor, AncestorExists, Descendant};
pub use connection::AnyConnection;
pub use deletable::{Deletable, DeleteError, DeleteFromVec};
pub use extension::ExtensionTable;
pub use foreign::{ForeignKeys, HasForeignKeys};
pub use insertable::{
    IdOrBuilder, InsertError, Insertable, InsertableVariant, MostConcreteTable, SetPrimaryKey,
};
pub use read::{BoundedRead, BoundedReadDispatch, Read, ReadDispatch};
pub use table_name::TableName;
pub use tabular::{Row, Rows, StaticTabular, Tabular};
pub use try_insert::TryInsertGeneric;
pub use updatable::Updatable;
pub use upsertable::{UpsertVec, Upsertable};
