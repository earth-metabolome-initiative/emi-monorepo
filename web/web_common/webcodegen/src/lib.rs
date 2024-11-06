#![doc = include_str!("../README.md")]
// #![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(unused_macro_rules)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]
#![deny(unused_import_braces)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod table_metadata;
mod schema;

pub use table_metadata::Table;