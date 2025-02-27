//! Module with the traits for primitive operation on the database

mod deletable;
mod loadable;
mod foreign;

pub use deletable::Deletable;
pub use loadable::Loadable;
pub use foreign::Foreign;
