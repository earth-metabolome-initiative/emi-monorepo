//! Submodule defining the outcome of a no-op operation.

use web_common_traits::prelude::Outcome;

use crate::api::ws::operations::NoOp;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
