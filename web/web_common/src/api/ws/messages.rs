//! Module providing the websocket messages used in the application.
use std::fmt::Debug;

use common_traits::prelude::*;

use super::{operations::OperationMessage, outcomes::OutcomeMessage};

#[basic]
/// Structure representing the reason for closing a websocket connection.
pub struct CloseReason {
    pub code: u16,
    pub description: Option<String>,
}

impl From<actix_ws::CloseReason> for CloseReason {
    fn from(reason: actix_ws::CloseReason) -> Self {
        Self {
            code: reason.code.into(),
            description: reason.description,
        }
    }
}

#[basic]
/// Enumeration for websocket messages sent from the frontend to the backend.
pub enum FrontendMessage {
    /// Close the websocket connection.
    Close(Option<CloseReason>),
    /// An operation to be executed by the backend.
    Operation(OperationMessage),
}

#[basic]
/// Enumeration for websocket messages sent from the backend to the frontend.
pub enum BackendMessage {
    /// Close the websocket connection.
    Close(Option<CloseReason>),
    /// Outcome of an operation executed by the backend.
    Outcome(OutcomeMessage),
}

#[cfg(feature = "backend")]
impl actix::Message for BackendMessage {
    type Result = ();
}

impl From<BackendMessage> for bytes::Bytes {
    fn from(msg: BackendMessage) -> Self {
        bincode::serialize(&msg).unwrap().into()
    }
}

impl From<actix_ws::Message> for FrontendMessage {
    fn from(actix_message: actix_ws::Message) -> Self {
        match actix_message {
            actix_ws::Message::Text(text) => {
                log::error!("Unexpected text message from backend: {:?}", text);
                unreachable!("Unexpected text message from backend");
            }
            actix_ws::Message::Binary(bin) => bincode::deserialize(&bin).unwrap(),
            actix_ws::Message::Ping(_) => {
                log::error!("Unexpected ping message from backend");
                unreachable!("Unexpected ping message from backend");
            }
            actix_ws::Message::Pong(_) => {
                log::error!("Unexpected pong message from backend");
                unreachable!("Unexpected pong message from backend");
            }
            actix_ws::Message::Close(reason) => FrontendMessage::Close(reason.map(Into::into)),
            actix_ws::Message::Continuation(_) => {
                log::error!("Unexpected continuation message from backend");
                unreachable!("Unexpected continuation message from backend");
            }
            actix_ws::Message::Nop => {
                log::error!("Unexpected nop message from backend");
                unreachable!("Unexpected nop message from backend");
            }
        }
    }
}

#[cfg(feature = "frontend")]
impl From<gloo_net::websocket::Message> for BackendMessage {

    fn from(value: gloo_net::websocket::Message) -> Self {
        match value {
            gloo_net::websocket::Message::Text(text) => {
                log::error!("Unexpected text message from frontend: {:?}", text);
                unreachable!("Unexpected text message from frontend");
            }
            gloo_net::websocket::Message::Bytes(bin) => {
                bincode::deserialize(&bin).unwrap()
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
