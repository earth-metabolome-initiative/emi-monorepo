//! Submodule providing the Notification Message struct.

use crate::database::Notification;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
/// Struct representing a notification message.
pub struct NotificationMessage {
    pub id: i32,
    pub operation: String,
    pub table_name: String,
    pub record: Vec<u8>,
    pub read: bool,
}

impl NotificationMessage {
    /// Create a new notification message.
    pub fn new(notification: Notification, row: Vec<u8>) -> Self {
        Self {
            id: notification.id,
            operation: notification.operation,
            table_name: notification.table_name,
            record: row,
            read: false,
        }
    }
}
