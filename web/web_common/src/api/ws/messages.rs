//! Module providing the websocket messages used in the application.
use std::{fmt::Debug, str::FromStr};

use crate::api::{
    auth::users::{CompleteProfile, User},
    oauth::jwt_cookies::AccessToken,
    ApiError,
};
use crate::api::form_traits::FormResult;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CloseReason {
    code: u16,
    reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Table {
    #[serde(rename = "users")]
    Users,
    #[serde(rename = "teams")]
    Teams,
}

impl Table {
    pub fn is_users(&self) -> bool {
        match self {
            Table::Users => true,
            _ => false,
        }
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Table::Users => write!(f, "users"),
            Table::Teams => write!(f, "teams"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum Role {
    Admin,
    Moderator,
    User,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "admin"),
            Role::Moderator => write!(f, "moderator"),
            Role::User => write!(f, "user"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum SQLOperation {
    #[serde(rename = "INSERT")]
    Insert,
    #[serde(rename = "UPDATE")]
    Update,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "SELECT")]
    Select,
}

impl SQLOperation {
    pub fn is_insert(&self) -> bool {
        match self {
            SQLOperation::Insert => true,
            _ => false,
        }
    }

    pub fn is_update(&self) -> bool {
        match self {
            SQLOperation::Update => true,
            _ => false,
        }
    }

    pub fn is_delete(&self) -> bool {
        match self {
            SQLOperation::Delete => true,
            _ => false,
        }
    }

    pub fn is_select(&self) -> bool {
        match self {
            SQLOperation::Select => true,
            _ => false,
        }
    }
}

impl FromStr for SQLOperation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "insert" => Ok(SQLOperation::Insert),
            "INSERT" => Ok(SQLOperation::Insert),
            "update" => Ok(SQLOperation::Update),
            "UPDATE" => Ok(SQLOperation::Update),
            "delete" => Ok(SQLOperation::Delete),
            "DELETE" => Ok(SQLOperation::Delete),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum FormAction {
    CompleteProfile(CompleteProfile),
}

impl FormAction {
    pub fn requires_authentication(&self) -> bool {
        match self {
            FormAction::CompleteProfile(_) => CompleteProfile::requires_authentication(),
        }
    }
}

impl From<CompleteProfile> for FormAction {
    fn from(profile: CompleteProfile) -> Self {
        FormAction::CompleteProfile(profile)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FrontendMessage {
    Close(Option<CloseReason>),
    Authentication(AccessToken),
    Task(uuid::Uuid, FormAction),
}

impl FrontendMessage {
    pub fn submit(id: uuid::Uuid, action: FormAction) -> Self {
        FrontendMessage::Task(id, action)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BackendMessage {
    Close(Option<CloseReason>),
    User(SQLOperation, User),
    TaskResult(uuid::Uuid, Result<(), ApiError>),
    ExpiredToken,
    Authenticated,
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
impl From<gloo_net::websocket::Message> for BackendMessage {
    fn from(msg: gloo_net::websocket::Message) -> Self {
        match msg {
            gloo_net::websocket::Message::Text(text) => {
                log::error!("Unexpected text message from frontend: {:?}", text);
                unreachable!("Unexpected text message from frontend");
            }
            gloo_net::websocket::Message::Bytes(bin) => bincode::deserialize(&bin).unwrap(),
        }
    }
}

#[cfg(feature = "frontend")]
impl From<FrontendMessage> for gloo_net::websocket::Message {
    fn from(msg: FrontendMessage) -> Self {
        gloo_net::websocket::Message::Bytes(bincode::serialize(&msg).unwrap())
    }
}
