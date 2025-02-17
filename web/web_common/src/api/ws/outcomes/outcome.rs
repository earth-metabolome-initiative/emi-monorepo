//! Submodule providing the OutcomeMessage enum and the Outcome trait.

use common_traits::prelude::basic;
use web_common_traits::prelude::Operation;

use crate::api::ws::operations::NoOp;

/// Enumeration of all possible operations.
#[basic]
pub enum OutcomeMessage {
    /// No-op operation.
    NoOp(Result<<NoOp as Operation>::Outcome, <NoOp as Operation>::Error>),
}
