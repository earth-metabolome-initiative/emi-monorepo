//! Submodule providing the OutcomeMessage enum and the Outcome trait.

use web_common_traits::prelude::Operation;

use crate::api::ws::operations::NoOp;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// Enumeration of all possible operations.
pub enum OutcomeMessage {
    /// No-op operation.
    NoOp(Result<<NoOp as Operation>::Outcome, <NoOp as Operation>::Error>),
}
