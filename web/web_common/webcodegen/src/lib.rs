#![doc = include_str!("../README.md")]
extern crate prettyplease;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

pub mod codegen;
pub mod errors;
mod schema;
mod sql_functions;
mod syngen;
mod traits;

pub use codegen::Codegen;
pub use traits::{ColumnLike, TableLike};
