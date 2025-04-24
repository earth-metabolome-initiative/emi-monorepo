//! Submodule providing the enumeration of messages from the `SQLite` database
//! web-worker to the frontend Yew components.

use core_structures::tables::read::B2FReadAll;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Messages from the `SQLite` database web-worker to the frontend.
pub enum DB2CMessage {
    ReadAll(B2FReadAll),
}

impl From<B2FReadAll> for DB2CMessage {
    fn from(msg: B2FReadAll) -> Self {
        DB2CMessage::ReadAll(msg)
    }
}

impl<T> From<DB2CMessage> for Vec<T>
where
    Vec<T>: TryFrom<B2FReadAll, Error = core_structures::tables::read::ReadError>,
{
    fn from(msg: DB2CMessage) -> Self {
        match msg {
            DB2CMessage::ReadAll(b2f_read_all) => {
                b2f_read_all.try_into().expect("Message conversion failed")
            }
        }
    }
}
