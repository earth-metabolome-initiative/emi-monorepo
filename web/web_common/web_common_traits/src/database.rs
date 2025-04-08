//! Module with the traits for primitive operation on the database

mod deletable;
mod foreign;
mod insertable;
mod loadable;
mod updatable;

pub use deletable::{Deletable, DeleteError};
pub use foreign::Foreign;
pub use insertable::{InsertError, Insertable, InsertableBuilder, InsertableVariant};
#[cfg(feature = "backend")]
pub use insertable::BackendInsertableVariant;
pub use loadable::Loadable;
pub use updatable::Updatable;
