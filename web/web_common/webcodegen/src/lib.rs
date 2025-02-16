#![doc = include_str!("../README.md")]
extern crate prettyplease;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

mod custom_schema_constraints;
pub mod errors;
mod meta_sql;
mod schema;
mod sql_functions;
mod table_metadata;
mod codegen;
mod postgis;
mod syngen;

pub use postgis::{GeometryColumn, GeographyColumn};
pub use table_metadata::{
    CheckConstraint, Column, ConstraintColumnUsage, ConstraintTableUsage, DomainConstraint,
    KeyColumnUsage, ReferentialConstraint, SQLFunction, SQLOperator, Table,
    TableConstraint, PgType, PgAttribute, PGClass, PgEnum, PgIndex
};
pub use custom_schema_constraints::{
    CompulsorySiblingColumnConstraint, ConstraintError, CustomColumnConstraint,
    CustomTableConstraint, IsForeignKeyConstraint, LowercaseColumnConstraint,
    LowercaseTableConstraint, NotNullColumnConstraint, HasSpecificTypeConstraint,
    CompatibleForeignTypeConstraint,
};
pub use codegen::Codegen;
pub use meta_sql::AuthorizationFunctionBuilder;