//! Submodule providing the Notification Message struct.

use crate::database::Notification;
use serde::{Deserialize, Serialize};

use super::ViewRow;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
/// Struct representing a notification message.
pub struct NotificationMessage {
    operation: SQLOperation,
    notification: Notification,
    view_row: Option<ViewRow>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum SQLOperation {
    #[serde(rename = "INSERT")]
    Insert,
    #[serde(rename = "UPDATE")]
    Update,
    #[serde(rename = "DELETE")]
    Delete,
}

impl NotificationMessage {
    /// Create a new notification message.
    pub fn new(
        operation: SQLOperation,
        notification: Notification,
        view_row: Option<ViewRow>,
    ) -> Self {
        Self {
            operation,
            notification,
            view_row,
        }
    }
}
