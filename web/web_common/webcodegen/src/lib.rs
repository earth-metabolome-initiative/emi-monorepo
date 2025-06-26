#![doc = include_str!("../README.md")]
extern crate prettyplease;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

mod codegen;
mod column_same_as_network;
mod custom_schema_constraints;
pub mod errors;
mod postgis;
mod schema;
mod sql_functions;
mod syngen;
mod table_metadata;
mod utils;

pub use codegen::Codegen;
pub use column_same_as_network::ColumnSameAsNetwork;
pub use custom_schema_constraints::{
    CompatibleForeignTypeConstraint, CompulsorySiblingColumnConstraint, ConstraintError,
    CustomColumnConstraint, CustomTableConstraint, HasSpecificTypeConstraint,
    IsForeignKeyConstraint, LowercaseColumnConstraint, LowercaseTableConstraint,
    NotNullColumnConstraint,
};
pub use postgis::{GeographyColumn, GeometryColumn};
pub use table_metadata::{
    CheckConstraint, Column, ConstraintColumnUsage, ConstraintTableUsage, DomainConstraint,
    KeyColumnUsage, PGClass, PgAttribute, PgDepend, PgEnum, PgExtension, PgIndex, PgProc,
    PgSetting, PgStatStatement, PgType, ReferentialConstraint, Table, TableConstraint,
};
