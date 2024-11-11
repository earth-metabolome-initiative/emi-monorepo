//! Submodule defining diesel-based structs for Postgres core metadata tables.

mod check_constraint;
mod column;
mod constraint_column;
mod domain_constraint;
mod key_column_usage;
mod pg_index;
mod referential_constraint;
mod table;
mod table_constraint;

pub use check_constraint::CheckConstraint;
pub use column::Column;
pub use constraint_column::ConstraintColumnUsage;
pub use domain_constraint::DomainConstraint;
pub use key_column_usage::KeyColumnUsage;
pub use pg_index::Index;
pub use referential_constraint::ReferentialConstraint;
pub use table::Table;
pub use table_constraint::TableConstraint;
