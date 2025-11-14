//! Submodule defining traits used for SQL schema constraints.

mod table_constraint;
pub use table_constraint::TableConstraint;
mod column_constraint;
pub use column_constraint::ColumnConstraint;
pub mod constrainer;
pub use constrainer::{Constrainer, DefaultConstrainer, GenericConstrainer};
mod constraint_failure_information;
pub use constraint_failure_information::ConstraintFailureInformation;
mod foreign_key_constraint;
pub use foreign_key_constraint::ForeignKeyConstraint;
