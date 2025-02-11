#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(unused_macro_rules)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]
#![deny(unused_import_braces)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

pub mod operation;
pub mod outcome;
pub mod connection;

/// Main prelude for the library.
pub mod prelude {
    pub use crate::connection::Connection;
    pub use crate::operation::Operation;
    pub use crate::outcome::Outcome;
}