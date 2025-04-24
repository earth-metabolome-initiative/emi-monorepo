//! Submodule providing the enumeration for the internal messages used
//! in the DB/WebSocket worker.

pub mod db_internal_message;
pub mod ws_internal_message;

pub enum InternalMessage {
    DB(db_internal_message::DBInternalMessage),
    WS(ws_internal_message::WSInternalMessage),
}

impl From<db_internal_message::DBInternalMessage> for InternalMessage {
    fn from(value: db_internal_message::DBInternalMessage) -> Self {
        Self::DB(value)
    }
}

impl From<ws_internal_message::WSInternalMessage> for InternalMessage {
    fn from(value: ws_internal_message::WSInternalMessage) -> Self {
        Self::WS(value)
    }
}
