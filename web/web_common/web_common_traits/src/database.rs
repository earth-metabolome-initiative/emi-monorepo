//! Module with the traits for primitive operation on the database

mod deletable;
mod foreign;
mod loadable;
mod insertable;

pub use deletable::Deletable;
pub use foreign::Foreign;
pub use loadable::Loadable;
