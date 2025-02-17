//! Submodule providing the variants for the operation messages.
pub mod no_op;
pub mod operation;

pub use no_op::NoOp;
pub use operation::OperationMessage;
