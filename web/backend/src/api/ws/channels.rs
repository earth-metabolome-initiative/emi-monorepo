use crate::api::ws::socket::WebSocket;
use actix::Message;
use uuid::Uuid;
use core::fmt::Debug;
use serde::{Deserialize, Serialize};
use sqlx::error::Error;
use sqlx::postgres::PgListener;
use sqlx::Pool;
use sqlx::Postgres;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use web_common::api::database::operations::Operation;
use web_common::api::ws::messages::BackendMessage;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Channel {
    NotifyUser(Uuid),
}

impl Display for Channel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Channel::NotifyUser(user_id) => write!(f, "notify_user_{}", user_id),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Message, Clone, PartialEq, Eq)]
#[rtype(result = "()")]
pub struct ChannelMessage {
    pub table: Operation,
}

impl ChannelMessage {
    pub fn into_view(self) -> Operation {
        self.table
    }
}

pub async fn start_listening(
    pool: &Pool<Postgres>,
    channels: Vec<Channel>,
    address: actix::prelude::Addr<WebSocket>,
) -> Result<(), Error> {
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
            log::info!(
                "Getting notification with payload: {:?} from channel {:?}",
                notification.payload(),
                notification.channel()
            );

            let notification_payload: String = notification.payload().to_owned();

            let value: ChannelMessage = serde_json::from_str(&notification_payload).unwrap();

            address.do_send(value);
        }
    }
}

impl actix::Handler<ChannelMessage> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: ChannelMessage, ctx: &mut Self::Context) {
        ctx.binary(BackendMessage::Operation(msg.into_view()));
    }
}
