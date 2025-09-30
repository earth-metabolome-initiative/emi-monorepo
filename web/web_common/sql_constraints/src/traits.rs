//! Submodule defining traits used for SQL schema constraints.

mod schema;
pub use schema::Schema;
mod constrainable_table;
pub use constrainable_table::ConstrainableTable;
mod table_constraint;
pub use table_constraint::TableConstraint;
mod column_constraint;
pub use column_constraint::ColumnConstraint;
mod constrainable_column;
pub use constrainable_column::ConstrainableColumn;
mod constrainable;
pub use constrainable::Constrainable;
mod constraint;
pub use constraint::Constraint;
mod constrainer;
pub use constrainer::{Constrainer, GenericConstrainer};
mod constraint_failure_information;
pub use constraint_failure_information::ConstraintFailureInformation;
