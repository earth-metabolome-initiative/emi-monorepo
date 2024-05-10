//! Module providing the websocket messages used in the application.
use std::fmt::Debug;

use crate::api::ApiError;
use crate::database::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CloseReason {
    code: u16,
    reason: Option<String>,
}

impl CloseReason {
    pub fn new<S: ToString>(code: u16, reason: Option<S>) -> Self {
        Self {
            code,
            reason: reason.map(|s| s.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FrontendMessage {
    Close(Option<CloseReason>),
    Task(Uuid, Operation),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BackendMessage {
    Close(Option<CloseReason>),
    RefreshUser(User),
    Notification(NotificationMessage),
    SearchTable(uuid::Uuid, Vec<Vec<u8>>),
    GetTable(uuid::Uuid, Option<String>, Vec<u8>),
    AllTable(uuid::Uuid, Vec<Vec<u8>>),
    Completed(uuid::Uuid),
    Error(uuid::Uuid, ApiError),
}

#[cfg(feature = "backend")]
impl actix::Message for BackendMessage {
    type Result = ();
}

#[cfg(feature = "backend")]
impl From<BackendMessage> for bytes::Bytes {
    fn from(msg: BackendMessage) -> Self {
        bincode::serialize(&msg).unwrap().into()
    }
}

#[cfg(feature = "backend")]
impl From<actix_web_actors::ws::Message> for FrontendMessage {
    fn from(actix_message: actix_web_actors::ws::Message) -> Self {
        match actix_message {
            actix_web_actors::ws::Message::Text(text) => {
                log::error!("Unexpected text message from backend: {:?}", text);
                unreachable!("Unexpected text message from backend");
            }
            actix_web_actors::ws::Message::Binary(bin) => bincode::deserialize(&bin).unwrap(),
            actix_web_actors::ws::Message::Ping(_) => {
                log::error!("Unexpected ping message from backend");
                unreachable!("Unexpected ping message from backend");
            }
            actix_web_actors::ws::Message::Pong(_) => {
                log::error!("Unexpected pong message from backend");
                unreachable!("Unexpected pong message from backend");
            }
            actix_web_actors::ws::Message::Close(reason) => {
                FrontendMessage::Close(reason.map(|r: actix_web_actors::ws::CloseReason| {
                    CloseReason {
                        code: r.code.into(),
                        reason: r.description,
                    }
                }))
            }
            actix_web_actors::ws::Message::Continuation(_) => {
                log::error!("Unexpected continuation message from backend");
                unreachable!("Unexpected continuation message from backend");
            }
            actix_web_actors::ws::Message::Nop => {
                log::error!("Unexpected nop message from backend");
                unreachable!("Unexpected nop message from backend");
            }
        }
    }
}

#[cfg(feature = "frontend")]
impl TryFrom<gloo_net::websocket::Message> for BackendMessage {
    type Error = ApiError;

    fn try_from(value: gloo_net::websocket::Message) -> Result<Self, ApiError> {
        match value {
            gloo_net::websocket::Message::Text(text) => {
                log::error!("Unexpected text message from frontend: {:?}", text);
                unreachable!("Unexpected text message from frontend");
            }
            gloo_net::websocket::Message::Bytes(bin) => {
                bincode::deserialize(&bin).map_err(ApiError::from)
            }
        }
    }
}

#[cfg(feature = "frontend")]
impl From<FrontendMessage> for gloo_net::websocket::Message {
    fn from(msg: FrontendMessage) -> Self {
        gloo_net::websocket::Message::Bytes(bincode::serialize(&msg).unwrap())
    }
}
