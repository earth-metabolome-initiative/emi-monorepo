//! Module with the traits for primitive operation on the database

mod deletable;
mod foreign;
mod insertable;
mod read;
mod tabular;
mod updatable;
mod upsertable;

pub use deletable::{Deletable, DeleteError, DeleteFromVec};
pub use foreign::{Foreign, ForeignKeys, HasForeignKeys};
#[cfg(feature = "backend")]
pub use insertable::BackendInsertableVariant;
pub use insertable::{InsertError, Insertable, InsertableBuilder, InsertableVariant};
#[cfg(feature = "diesel-async")]
pub use read::{AsyncBoundedRead, AsyncBoundedReadDispatch, AsyncRead, AsyncReadDispatch};
pub use read::{BoundedRead, BoundedReadDispatch, Read, ReadDispatch};
pub use tabular::{Row, Rows, StaticTabular, Tabular};
pub use updatable::Updatable;
pub use upsertable::{UpsertVec, Upsertable};
