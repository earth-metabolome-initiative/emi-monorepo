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

pub async fn start_listening<C>(
    pool: &Pool<Postgres>,
    channels: Vec<C>,
    address: actix::prelude::Addr<WebSocket>,
) -> Result<(), Error>
where
    C: Channel,
    WebSocket: actix_web::Handler<C::Message>,
{
    let channel_names = channels
        .iter()
        .map(|channel| channel.to_string())
        .collect::<Vec<String>>();

    let channel_references = channel_names
        .iter()
        .map(|channel_name| channel_name.as_str())
        .collect::<Vec<&str>>();

    // Initiate the logger.
    let mut listener = PgListener::connect_with(pool).await.unwrap();
    listener.listen_all(channel_references).await?;
    loop {
        while let Some(notification) = listener.try_recv().await? {
            let notification_payload: String = notification.payload().to_owned();
            let value: C::Message = serde_json::from_str(&notification_payload).unwrap();
            address.do_send(value);
        }
    }
}
