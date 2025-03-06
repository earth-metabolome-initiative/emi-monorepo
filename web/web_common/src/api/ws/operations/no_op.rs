//! Test operation that does nothing.

use web_common_traits::prelude::Operation;

use crate::api::ws::{operation_errors::InfallibleOperationError, outcomes::NoOpOutcome};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NoOp {
    id: uuid::Uuid,
}

impl Operation for NoOp {
    type Outcome = NoOpOutcome;
    type Error = InfallibleOperationError<Self>;

    fn id(&self) -> uuid::Uuid {
        self.id
    }
}
