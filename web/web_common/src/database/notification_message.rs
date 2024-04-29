//! Submodule providing the Notification Message struct.

use crate::database::Notification;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
/// Struct representing a notification message.
pub struct NotificationMessage {
    notification: Notification,
    /// The bytes composing the row that was inserted, updated, or deleted.
    row: Vec<u8>,
}

impl NotificationMessage {
    /// Create a new notification message.
    pub fn new(notification: Notification, row: Vec<u8>) -> Self {
        Self { notification, row }
    }

    /// Get the notification.
    pub fn notification(&self) -> &Notification {
        &self.notification
    }

    /// Get the row.
    pub fn row(&self) -> &[u8] {
        &self.row
    }
}