//! Submodule defining diesel-based structs for Postgres core metadata tables.

mod check_constraint;
mod column;
mod constraint_column_usage;
mod constraint_table_usage;
mod domain_constraint;
mod key_column_usage;
mod pg_attribute;
mod pg_class;
mod pg_constraint;
mod pg_depend;
mod pg_enum;
mod pg_extension;
mod pg_index;
mod pg_operator;
mod pg_proc;
mod pg_setting;
mod pg_stat_statements;
mod pg_trigger;
mod pg_type;
mod referential_constraint;
mod table;
mod table_constraint;

pub use check_constraint::CheckConstraint;
pub use column::Column;
pub use constraint_column_usage::ConstraintColumnUsage;
pub use constraint_table_usage::ConstraintTableUsage;
pub use domain_constraint::DomainConstraint;
pub use key_column_usage::KeyColumnUsage;
pub use pg_attribute::PgAttribute;
pub use pg_class::PGClass;
pub use pg_constraint::PgConstraint;
pub use pg_depend::PgDepend;
pub use pg_enum::PgEnum;
pub use pg_extension::PgExtension;
pub use pg_index::PgIndex;
pub use pg_operator::PgOperator;
pub use pg_proc::PgProc;
pub use pg_setting::PgSetting;
pub use pg_stat_statements::PgStatStatement;
pub use pg_trigger::PgTrigger;
pub use pg_type::PgType;
pub use referential_constraint::ReferentialConstraint;
pub use table::Table;
pub use table_constraint::TableConstraint;
