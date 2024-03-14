use crate::api::ws::socket::WebSocket;
use actix::dev::channel;
use actix::Message;
use actix_web::web;
use core::fmt::Debug;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::error::Error;
use sqlx::postgres::PgListener;
use sqlx::Pool;
use sqlx::Postgres;
use uuid::Uuid;
use web_common::api::auth::users::User;
use web_common::api::ws::messages::SQLOperation;

pub enum Channel {
    NotifyUser(User),
}

impl ToString for Channel {
    fn to_string(&self) -> String {
        match self {
            Channel::NotifyUser(user) => format!("notify_user_{}", user.id()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ChannelTable {
    Users,
}

#[derive(Debug, Serialize, Deserialize, Message, Clone, PartialEq, Eq)]
#[rtype(result = "()")]
pub struct ChannelMessage<Record = serde_json::Value> {
    pub table: ChannelTable,
    pub operation: SQLOperation,
    pub record: Record,
}

impl ChannelMessage {
    pub fn deserialize_into<Record: DeserializeOwned>(
        self,
    ) -> Result<ChannelMessage<Record>, serde_json::Error> {
        let record: Record = serde_json::from_value(self.record)?;
        Ok(ChannelMessage {
            table: self.table,
            operation: self.operation,
            record,
        })
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

            match &value.table {
                ChannelTable::Users => {
                    address.do_send(value.deserialize_into::<User>().unwrap());
                }
                table_name => {
                    unimplemented!(
                        "Received a notification for a table {:?} that is not implemented yet",
                        table_name
                    );
                }
            }
        }
    }
}
