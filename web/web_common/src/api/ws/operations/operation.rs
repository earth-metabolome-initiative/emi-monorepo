//! Submodule providing the OperationMessage enum and the Operation trait.

use super::NoOp;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// Enumeration of all possible operations.
pub enum OperationMessage {
    /// No-op operation.
    NoOp(NoOp),
}
