//! Submodule defining the outcome of a no-op operation.

use common_traits::prelude::basic;
use web_common_traits::prelude::Outcome;

use crate::api::ws::operations::NoOp;

#[basic]
pub struct NoOpOutcome {
    /// The operation that was executed.
    pub operation: NoOp,
}

impl From<NoOp> for NoOpOutcome {
	fn from(operation: NoOp) -> Self {
		Self { operation }
	}
}

impl Outcome for NoOpOutcome {
    type Operation = NoOp;

    fn operation(&self) -> &Self::Operation {
        &self.operation
    }
}
