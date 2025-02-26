//! Module with the traits for primitive operation on the database

mod deletable;
mod loadable;

pub use deletable::Deletable;
pub use loadable::Loadable;