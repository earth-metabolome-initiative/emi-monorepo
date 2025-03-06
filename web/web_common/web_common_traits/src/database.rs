//! Module with the traits for primitive operation on the database

mod deletable;
mod foreign;
mod insertable;
mod loadable;

pub use deletable::Deletable;
pub use foreign::Foreign;
pub use insertable::{InsertError, Insertable, InsertableBuilder, InsertableVariant};
pub use loadable::Loadable;
