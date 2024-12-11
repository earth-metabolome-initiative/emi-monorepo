//! Submodule defining diesel-based structs for Postgres core metadata tables.

mod check_constraint;
mod column;
mod constraint_column_usage;
mod constraint_table_usage;
mod domain_constraint;
mod key_column_usage;
mod pg_index;
mod referential_constraint;
mod sql_function;
mod sql_operator;
mod table;
mod table_constraint;
mod pg_type;
mod pg_attribute;
mod pg_class;
mod pg_enum;

pub use check_constraint::CheckConstraint;
pub use column::Column;
pub use constraint_column_usage::ConstraintColumnUsage;
pub use constraint_table_usage::ConstraintTableUsage;
pub use domain_constraint::DomainConstraint;
pub use key_column_usage::KeyColumnUsage;
pub use pg_index::Index;
pub use referential_constraint::ReferentialConstraint;
pub use sql_function::SQLFunction;
pub use sql_operator::SQLOperator;
pub use table::Table;
pub use table_constraint::TableConstraint;
pub use pg_type::PgType;
pub use pg_attribute::PgAttribute;
pub use pg_class::PGClass;
pub use pg_enum::PgEnum;