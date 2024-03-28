//! Submodule defining the websocket actor and its message handling.

use crate::models::Notification;
use crate::models::User;
use crate::views::ViewRow;
use crate::DBPool;
use crate::DieselConn;
use actix::ActorContext;
use actix::AsyncContext;
use actix::Message;
use actix::SpawnHandle;
use actix::WrapFuture;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use serde::Deserialize;
use serde::Serialize;
use sqlx::postgres::PgListener;
use sqlx::{Pool as SQLxPool, Postgres};
use web_common::api::oauth::jwt_cookies::AccessToken;
use web_common::api::ws::messages::{BackendMessage, FrontendMessage};
use web_common::database::NotificationMessage;
use web_common::database::View;

pub struct WebSocket {
    notifications_handler: Option<SpawnHandle>,
    pub(crate) user: Option<(User, AccessToken)>,
    diesel: DBPool,
    pub(crate) diesel_connection: DieselConn,
    redis: redis::Client,
    sqlx: SQLxPool<Postgres>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NotificationPayload {
    operation: web_common::database::notification_message::SQLOperation,
    notification: Notification,
}

impl WebSocket {
    pub fn new(diesel: DBPool, sqlx: SQLxPool<Postgres>, redis: redis::Client) -> Self {
        Self {
            notifications_handler: None,
            user: None,
            diesel_connection: diesel.get().unwrap(),
            diesel,
            redis,
            sqlx,
        }
    }

    pub fn authenticated(
        diesel: DBPool,
        sqlx: SQLxPool<Postgres>,
        redis: redis::Client,
        user: User,
        access_token: AccessToken,
    ) -> Self {
        let diesel_connection = diesel.get().unwrap();
        Self {
            notifications_handler: None,
            user: Some((user, access_token)),
            diesel_connection,
            diesel,
            redis,
            sqlx,
        }
    }

    fn listen_for_notifications(&mut self, ctx: &mut <WebSocket as Actor>::Context) {
        // If the handler is stopped or was never started, start it.
        if self.notifications_handler.is_none() {
            if let Some((user, _)) = &self.user {
                let address = ctx.address().clone();
                let channel_name = format!("user_{}", user.id);
                let pool = self.sqlx.clone();
                let mut diesel_connection = self.diesel.get().unwrap();
                self.notifications_handler = Some(
                    ctx.spawn(
                        async move {
                            // Initiate the logger.
                            let mut listener = PgListener::connect_with(&pool).await.unwrap();
                            match listener.listen_all([channel_name.as_str()]).await {
                                Ok(_) => {}
                                Err(err) => {
                                    log::error!("Error listening for notifications: {:?}", err);
                                    return;
                                }
                            }
                            loop {
                                while let Some(postgres_notification) =
                                    listener.try_recv().await.unwrap()
                                {
                                    let notification_payload: String =
                                        postgres_notification.payload().to_owned();
                                    let payload: NotificationPayload =
                                        serde_json::from_str(&notification_payload).unwrap();

                                    let row_id = payload.notification.row_id.clone();
                                    let view: View =
                                        payload.notification.table_name.as_str().into();

                                    let row = if let Some(id) = row_id {
                                        match ViewRow::get(id, &mut diesel_connection, &view) {
                                            Ok(row) => Some(row.into()),
                                            Err(err) => {
                                                log::error!(
                                                    "Error getting row from view {}: {:?}",
                                                    view,
                                                    err
                                                );
                                                return;
                                            }
                                        }
                                    } else {
                                        None
                                    };

                                    address.do_send(BackendMessage::Notification(
                                        NotificationMessage::new(
                                            payload.operation,
                                            payload.notification.into(),
                                            row,
                                        ),
                                    ));
                                }
                            }
                        }
                        .into_actor(self),
                    ),
                );
            }
        }
    }
}

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl actix::Handler<BackendMessage> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: BackendMessage, ctx: &mut Self::Context) {
        ctx.binary(msg);
    }
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
enum InternalMessage {
    Unauthorized,
}

impl actix::Handler<InternalMessage> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: InternalMessage, ctx: &mut Self::Context) {
        match msg {
            InternalMessage::Unauthorized => {
                ctx.close(Some(ws::CloseCode::Policy.into()));
            }
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        self.listen_for_notifications(ctx);

        match msg {
            Ok(msg) => {
                let frontend_message: FrontendMessage = msg.into();
                match frontend_message {
                    FrontendMessage::Close(_code) => {
                        ctx.stop();
                    }
                    FrontendMessage::Task(task) => {
                        if task.requires_authentication() && self.user.is_none() {
                            ctx.address().do_send(InternalMessage::Unauthorized);
                            return;
                        }
                        ctx.address().do_send(InternalMessage::Unauthorized);
                    }
                }
            }
            Err(err) => {
                log::error!("Error reading from WebSocket: {:?}", err);
                ctx.stop();
            }
        }
    }
}
