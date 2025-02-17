//! Submodule providing the variants for the outcome messages.
pub mod no_op_outcome;
pub mod outcome;

pub use no_op_outcome::NoOpOutcome;
pub use outcome::OutcomeMessage;
