use crate::api::ws::socket::WebSocket;
use actix::Message;
use actix_web_actors::ws::CloseCode;
use core::fmt::Debug;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::error::Error;
use sqlx::postgres::PgListener;
use sqlx::Pool;
use sqlx::Postgres;
use web_common::api::auth::users::User;
use web_common::api::ws::messages;
use web_common::api::ws::messages::BackendMessage;
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

#[derive(Debug, Serialize, Deserialize, Message, Clone, PartialEq, Eq)]
#[rtype(result = "()")]
pub struct ChannelMessage<Record = serde_json::Value> {
    pub table: messages::Table,
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
                messages::Table::Users => {
                    let message: ChannelMessage<crate::models::User> =
                        value.deserialize_into().unwrap();
                    address.do_send(message);
                }
                table_name => {
                    log::error!(
                        "Received a notification for a table {:?} that is not implemented yet",
                        table_name
                    );
                }
            }
        }
    }
}

impl actix::Handler<ChannelMessage<crate::models::User>> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: ChannelMessage<crate::models::User>, ctx: &mut Self::Context) {
        match msg.operation {
            SQLOperation::Update => {
                ctx.binary(BackendMessage::User(
                    SQLOperation::Update,
                    msg.record.to_web_common_user(),
                ));
            }
            SQLOperation::Insert => {
                unreachable!("We do not expect notifications for insert operations");
            }
            SQLOperation::Delete => {
                // If the current user has been deleted, close the connection
                if let Some(user) = &self.user {
                    if user.id() == msg.record.id() {
                        ctx.close(Some(CloseCode::Policy.into()));
                    }
                }
            }
            SQLOperation::Select => {
                unreachable!("We do not expect notifications for select operations");
            }
        }
    }
}
