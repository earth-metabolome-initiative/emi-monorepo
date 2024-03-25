use serde::{Deserialize, Serialize};
use super::Id;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Row {}

impl Row {
    /// Returns the primary key Uuid of the row.
    pub fn id(&self) -> Option<Id> {
        None
    }
}