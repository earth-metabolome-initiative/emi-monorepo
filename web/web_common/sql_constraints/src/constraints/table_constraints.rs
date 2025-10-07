//! Submodule providing constraint structs that can be applied to tables.

mod lowercase_table_name;
pub use lowercase_table_name::LowercaseTableName;
mod unique_check_constraint;
pub use unique_check_constraint::UniqueCheckConstraint;
