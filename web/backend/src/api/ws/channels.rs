use crate::api::ws::socket::WebSocket;

use serde::{Deserialize, Serialize};
use sqlx::error::Error;
use sqlx::postgres::PgListener;
use sqlx::Pool;
use sqlx::Postgres;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use uuid::Uuid;
use web_common::database::operations::Operation;
use web_common::api::ws::messages::BackendMessage;

pub trait Channel {
    type Message;

    /// Returns the operation that this channel listens to.
    fn operation(&self) -> Operation;

    /// Returns the primary key that this channel listens to.
    fn primary_key(&self) -> PrimaryKey;
}

impl<C> Display for C
where
    C: Channel,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let key = self.primary_key();
        write!(f, "{}_{}_{}", self.operation(), key.table_name(), key.id())
    }
}
