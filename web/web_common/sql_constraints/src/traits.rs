//! Submodule defining traits used for SQL schema constraints.

mod dynamic_schema;
pub use dynamic_schema::DynamicSchema;
mod constrainable_table;
pub use constrainable_table::ConstrainableTable;
mod table_constraint;
pub use table_constraint::TableConstraint;
mod constrainable;
pub use constrainable::Constrainable;
mod constraint;
pub use constraint::Constraint;
mod constrainer;
pub use constrainer::Constrainer;
mod constraint_failure_information;
pub use constraint_failure_information::ConstraintFailureInformation;