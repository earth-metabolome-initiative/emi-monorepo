//! Module providing the websocket messages used in the application.
use std::{fmt::Debug, str::FromStr};

use crate::api::{auth::users::User, oauth::jwt_cookies::AccessToken, ApiError};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use validator::Validate;

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

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum FormMethod {
    GET,    // Use GET to execute a query
    POST,   // Use POST to CREATE a new resource
    PUT, // Use PUT instead of POST when you want to UPDATE or PATCH a resource and you have an ID
    DELETE, // Use DELETE to remove a resource
    TRACE, // Use TRACE to test the connection
    HEAD, // Use HEAD to retrieve the headers of a resource
    PATCH, // Use PATCH to partially update a resource
}

impl FormMethod {
    #[cfg(feature = "frontend")]
    pub fn to_reqwasm(&self) -> gloo_net::http::Method {
        match self {
            FormMethod::GET => gloo_net::http::Method::GET,
            FormMethod::POST => gloo_net::http::Method::POST,
            FormMethod::PUT => gloo_net::http::Method::PUT,
            FormMethod::DELETE => gloo_net::http::Method::DELETE,
            FormMethod::TRACE => gloo_net::http::Method::TRACE,
            FormMethod::HEAD => gloo_net::http::Method::HEAD,
            FormMethod::PATCH => gloo_net::http::Method::PATCH,
        }
    }

    pub fn to_crud(&self) -> &'static str {
        match self {
            FormMethod::GET => "Retrieve",
            FormMethod::POST => "Create",
            FormMethod::PUT => "Update",
            FormMethod::DELETE => "Delete",
            FormMethod::TRACE => "Trace",
            FormMethod::HEAD => "Head",
            FormMethod::PATCH => "Patch",
        }
    }

    pub fn ongoing_crud(&self) -> &'static str {
        match self {
            FormMethod::GET => "Retrieving",
            FormMethod::POST => "Creating",
            FormMethod::PUT => "Updating",
            FormMethod::DELETE => "Deleting",
            FormMethod::TRACE => "Tracing",
            FormMethod::HEAD => "Heading",
            FormMethod::PATCH => "Patching",
        }
    }

    pub fn lower(&self) -> &'static str {
        match self {
            FormMethod::GET => "get",
            FormMethod::POST => "post",
            FormMethod::PUT => "put",
            FormMethod::DELETE => "delete",
            FormMethod::TRACE => "trace",
            FormMethod::HEAD => "head",
            FormMethod::PATCH => "patch",
        }
    }

    pub fn font_awesome_icon(&self) -> &'static str {
        match self {
            FormMethod::GET => "fas fa-search",
            FormMethod::POST => "fas fa-plus",
            FormMethod::PUT => "fas fa-pen",
            FormMethod::DELETE => "fas fa-trash",
            FormMethod::TRACE => "fas fa-search",
            FormMethod::HEAD => "fas fa-search",
            FormMethod::PATCH => "fas fa-pen",
        }
    }

    pub fn get() -> FormMethod {
        FormMethod::GET
    }

    pub fn post() -> FormMethod {
        FormMethod::POST
    }

    pub fn put() -> FormMethod {
        FormMethod::PUT
    }

    pub fn delete() -> FormMethod {
        FormMethod::DELETE
    }

    pub fn read() -> FormMethod {
        Self::get()
    }

    pub fn create() -> FormMethod {
        Self::post()
    }

    pub fn update() -> FormMethod {
        Self::put()
    }
}

impl Display for FormMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormMethod::GET => write!(f, "GET"),
            FormMethod::POST => write!(f, "POST"),
            FormMethod::PUT => write!(f, "PUT"),
            FormMethod::DELETE => write!(f, "DELETE"),
            FormMethod::TRACE => write!(f, "TRACE"),
            FormMethod::HEAD => write!(f, "HEAD"),
            FormMethod::PATCH => write!(f, "PATCH"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum FormAction {
    UpdateName,
}

impl FormAction {
    pub fn requires_authentication(&self) -> bool {
        match self {
            FormAction::UpdateName => true,
        }
    }

    pub fn method(&self) -> FormMethod {
        match self {
            FormAction::UpdateName => FormMethod::PUT,
        }
    }

    pub fn title(&self) -> String {
        match self {
            FormAction::UpdateName => "Name".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FrontendMessage {
    Close(Option<CloseReason>),
    Authentication(AccessToken),
    Task(uuid::Uuid, FormAction, Vec<u8>),
}

impl FrontendMessage {
    pub fn submit<Payload: Serialize + Validate>(
        id: uuid::Uuid,
        action: FormAction,
        payload: Payload,
    ) -> Self {
        Self::task(id, action, bincode::serialize(&payload).unwrap())
    }

    pub fn task(id: uuid::Uuid, action: FormAction, payload: Vec<u8>) -> Self {
        FrontendMessage::Task(id, action, payload)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BackendMessage {
    Close(Option<CloseReason>),
    User(SQLOperation, User),
    TaskResult(uuid::Uuid, FormAction, Result<(), ApiError>),
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
