//! Submodule providing the enumeration of messages from a component to the
//! `SQLite` database web-worker.

use core_structures::tables::read::F2BReadAll;
use serde::{Deserialize, Serialize};
use web_common_traits::crud::ReadAll;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Messages from the frontend to the web-worker.
pub enum C2DBMessage {
    ReadAll(F2BReadAll),
}

impl From<F2BReadAll> for C2DBMessage {
    fn from(msg: F2BReadAll) -> Self {
        C2DBMessage::ReadAll(msg)
    }
}

impl<T> From<ReadAll<T>> for C2DBMessage
where
    ReadAll<T>: Into<F2BReadAll>,
{
    fn from(msg: ReadAll<T>) -> Self {
        C2DBMessage::ReadAll(msg.into())
    }
}
