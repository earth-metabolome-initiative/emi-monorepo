//! Submodule providing the OperationMessage enum and the Operation trait.

use common_traits::prelude::basic;

use super::NoOp;

#[basic]
/// Enumeration of all possible operations.
pub enum OperationMessage {
    /// No-op operation.
    NoOp(NoOp),
}
