#![doc = include_str!("../README.md")]
extern crate prettyplease;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

pub mod codegen;
mod custom_schema_constraints;
pub mod errors;
mod schema;
mod sql_functions;
mod syngen;
mod table_extension_network;
mod traits;
mod utils;

pub use codegen::Codegen;
pub use custom_schema_constraints::{
    CompatibleForeignTypeConstraint, ConstraintError, CustomColumnConstraint,
    CustomTableConstraint, DuplicatedCheckConstraint, DuplicatedUniqueIndexConstraint,
    HasSpecificTypeConstraint, IsForeignKeyConstraint, LowercaseColumnConstraint,
    LowercaseTableConstraint, NotNullColumnConstraint, WordDeprecationConstraint,
};
pub use table_extension_network::TableExtensionNetwork;
pub use traits::{ColumnLike, TableLike};
