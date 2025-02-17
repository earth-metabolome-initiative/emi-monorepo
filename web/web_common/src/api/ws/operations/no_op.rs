//! Test operation that does nothing.

use common_traits::prelude::basic;
use web_common_traits::prelude::Operation;

use crate::api::ws::{operation_errors::InfallibleOperationError, outcomes::NoOpOutcome};

#[basic]
pub struct NoOp {
    id: uuid::Uuid,
}

impl Operation for NoOp {
    type Outcome = NoOpOutcome;
    type Error = InfallibleOperationError<Self>;

    fn id(&self) -> uuid::Uuid {
        self.id
    }

    #[cfg(feature = "backend")]
    async fn execute(
        self,
        _connection: &mut diesel::PgConnection,
    ) -> Result<Self::Outcome, Self::Error> {
        Ok(NoOpOutcome::from(self))
    }
}
