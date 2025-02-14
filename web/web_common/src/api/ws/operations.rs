//! Submodule providing the variants for the operation messages.
pub mod operation;
pub mod no_op;

pub use operation::OperationMessage;
pub use no_op::NoOp;