#![doc = include_str!("../README.md")]
// #![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(unused_macro_rules)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]
#![deny(unused_import_braces)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
extern crate prettyplease;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

mod schema;
mod sql_functions;
mod table_metadata;

pub use table_metadata::{
    CheckConstraint, Column, ConstraintColumnUsage, DomainConstraint, Index, KeyColumnUsage,
    ReferentialConstraint, SQLFunction, SQLOperator, SQLType, Table, TableConstraint, ConstraintTableUsage,
};
