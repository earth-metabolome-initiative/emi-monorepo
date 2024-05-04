//! Submodule defining the websocket actor and its message handling.

use std::mem::swap;

use crate::api::ws::users::UserMessage;
use crate::models::Notification;
use crate::models::User;
use crate::nested_models::NestedPublicUser;
use crate::table_enumeration::*;
use crate::DBPool;
use crate::DieselConn;
use actix::ActorContext;
use actix::AsyncContext;
use actix::Message;
use actix::SpawnHandle;
use actix::WrapFuture;
use actix::{Actor, StreamHandler};
use actix_web::web::Bytes;
use actix_web_actors::ws;
use serde::Deserialize;
use serde::Serialize;
use sqlx::postgres::PgListener;
use sqlx::{Pool as SQLxPool, Postgres};
use web_common::api::oauth::jwt_cookies::AccessToken;
use web_common::api::ws::messages::{BackendMessage, FrontendMessage};
use web_common::api::ApiError;
use web_common::database::NestedProject;
use web_common::database::NotificationMessage;
use web_common::database::Table;

#[derive(Debug, Deserialize, Serialize)]
struct NotificationRecord {
    /// The notification's metadata.
    notification: Notification,
    /// The unserialized record, which we cannot de-serialize
    /// a priori before deserializing the notification above.
    record: String,
}

pub struct WebSocket {
    notifications_handler: Option<SpawnHandle>,
    pub(crate) user: Option<(User, AccessToken)>,
    diesel: DBPool,
    pub(crate) diesel_connection: DieselConn,
    _redis: redis::Client,
    sqlx: SQLxPool<Postgres>,
    continuation_bytes: Vec<u8>,
}

impl WebSocket {
    pub fn new(diesel: DBPool, sqlx: SQLxPool<Postgres>, redis: redis::Client) -> Self {
        Self {
            notifications_handler: None,
            user: None,
            diesel_connection: diesel.get().unwrap(),
            diesel,
            _redis: redis,
            sqlx,
            continuation_bytes: Vec::new(),
        }
    }

    fn user(&self) -> Option<&User> {
        self.user.as_ref().map(|(user, _)| user)
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
            _redis: redis,
            sqlx,
            continuation_bytes: Vec::new(),
        }
    }

    fn is_authenticated(&self) -> bool {
        self.user.is_some()
    }

    fn listen_for_notifications(&mut self, ctx: &mut <WebSocket as Actor>::Context) {
        // If the handler is stopped or was never started, start it.
        if self.notifications_handler.is_none() {
            if let Some((user, _)) = &self.user {
                log::info!("Starting notifications handler for user {}", user.id);
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
                                    let payload: NotificationRecord =
                                        serde_json::from_str(&notification_payload).unwrap();

                                    let table: Table = payload
                                        .notification
                                        .table_name
                                        .as_str()
                                        .try_into()
                                        .unwrap();

                                    let serialized_record: Vec<u8> = match table {
                                        Table::Users => {
                                            let record: User =
                                                serde_json::from_str(&payload.record)
                                                    .expect("Error deserializing User");
                                            bincode::serialize(
                                                &NestedPublicUser::get(
                                                    record.id,
                                                    &mut diesel_connection,
                                                )
                                                .unwrap(),
                                            )
                                            .unwrap()
                                        }
                                        Table::Projects => {
                                            let flat_struct: crate::models::Project =
                                                serde_json::from_str(&payload.record)
                                                    .expect("Error deserializing Project");
                                            let frontend_variant: web_common::database::NestedProject = crate::nested_models::NestedProject::from_flat(
                                                flat_struct,
                                                &mut diesel_connection,
                                            ).unwrap().into();
                                            bincode::serialize(
                                                &frontend_variant,
                                            )
                                            .unwrap()
                                        }
                                        _ => {
                                            unimplemented!("Table not implemented: {:?}", table)
                                        }
                                    };

                                    address.do_send(BackendMessage::Notification(
                                        NotificationMessage::new(
                                            payload.notification.into(),
                                            serialized_record,
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

    fn started(&mut self, ctx: &mut Self::Context) {
        self.listen_for_notifications(ctx);
        if self.is_authenticated() {
            log::info!("Sending refresh token message");
            match NestedPublicUser::get(self.user().unwrap().id, &mut self.diesel_connection) {
                Ok(user) => {
                    ctx.address()
                        .do_send(BackendMessage::RefreshUser(user.into()));
                }
                Err(err) => {
                    log::error!("Error getting user: {:?}", err);
                    ctx.close(Some(ws::CloseCode::Error.into()));
                }
            }
        }
    }
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
        match msg {
            Ok(msg) => {
                if let ws::Message::Continuation(item) = msg {
                    match item {
                        actix_http::ws::Item::FirstBinary(bytes)
                        | actix_http::ws::Item::FirstText(bytes) => {
                            self.continuation_bytes = bytes.to_vec();
                        }
                        actix_http::ws::Item::Continue(bytes) => {
                            self.continuation_bytes.extend_from_slice(bytes.as_ref());
                        }
                        actix_http::ws::Item::Last(bytes) => {
                            self.continuation_bytes.extend_from_slice(bytes.as_ref());
                            let mut continuation_bytes = Vec::new();
                            swap(&mut self.continuation_bytes, &mut continuation_bytes);
                            let complete_message: Result<ws::Message, ws::ProtocolError> =
                                Ok(ws::Message::Binary(Bytes::from(continuation_bytes)));
                            self.continuation_bytes.clear();
                            self.handle(complete_message, ctx);
                        }
                    }
                    return;
                }
                let frontend_message: FrontendMessage = msg.into();
                match frontend_message {
                    FrontendMessage::Close(_code) => {
                        ctx.stop();
                    }
                    FrontendMessage::Task(task_id, operation) => {
                        if operation.requires_authentication() && self.user.is_none() {
                            ctx.address()
                                .do_send(BackendMessage::Error(task_id, ApiError::Unauthorized));
                            return;
                        }

                        match operation {
                            web_common::database::Operation::Insert(table_name, new_row) => {
                                let table: web_common::database::Table =
                                    match table_name.as_str().try_into() {
                                        Ok(table) => table,
                                        Err(err) => {
                                            ctx.address().do_send(BackendMessage::Error(
                                                task_id,
                                                ApiError::BadRequest(vec![err.to_string()]),
                                            ));
                                            return;
                                        }
                                    };
                                match <Table as InsertableTable>::insert(
                                    &table,
                                    new_row,
                                    self.user().map(|user| user.id).unwrap(),
                                    &mut self.diesel_connection,
                                ) {
                                    Ok(_) => {
                                        ctx.address().do_send(BackendMessage::Completed(task_id));
                                    }
                                    Err(err) => {
                                        ctx.address().do_send(BackendMessage::Error(task_id, err));
                                    }
                                }
                            }
                            web_common::database::Operation::Update(update) => match update {
                                web_common::database::Update::CompleteProfile(profile) => {
                                    ctx.address().do_send(UserMessage::CompleteProfile(
                                        task_id,
                                        profile.clone(),
                                    ));
                                }
                            },
                            web_common::database::Operation::Select(select) => match select {
                                web_common::database::selects::Select::SearchTable {
                                    table_name,
                                    query,
                                    number_of_results,
                                } => {
                                    let table: web_common::database::Table =
                                        match table_name.as_str().try_into() {
                                            Ok(table) => table,
                                            Err(err) => {
                                                ctx.address().do_send(BackendMessage::Error(
                                                    task_id,
                                                    ApiError::BadRequest(vec![err.to_string()]),
                                                ));
                                                return;
                                            }
                                        };
                                    match table.strict_word_similarity_search(
                                        &query,
                                        Some(number_of_results as i32),
                                        &mut self.diesel_connection,
                                    ) {
                                        Ok(records) => {
                                            ctx.address().do_send(BackendMessage::SearchTable(
                                                task_id, records,
                                            ));
                                        }
                                        Err(err) => {
                                            ctx.address()
                                                .do_send(BackendMessage::Error(task_id, err));
                                        }
                                    }
                                }
                                web_common::database::Select::Id {
                                    table_name,
                                    primary_key,
                                } => {
                                    let table: web_common::database::Table =
                                        match table_name.as_str().try_into() {
                                            Ok(table) => table,
                                            Err(err) => {
                                                ctx.address().do_send(BackendMessage::Error(
                                                    task_id,
                                                    ApiError::BadRequest(vec![err.to_string()]),
                                                ));
                                                return;
                                            }
                                        };
                                    match <Table as IdentifiableTable>::get(
                                        &table,
                                        primary_key,
                                        &mut self.diesel_connection,
                                    ) {
                                        Ok(record) => {
                                            ctx.address()
                                                .do_send(BackendMessage::GetTable(task_id, record));
                                        }
                                        Err(err) => {
                                            ctx.address()
                                                .do_send(BackendMessage::Error(task_id, err));
                                        }
                                    }
                                }
                                web_common::database::Select::All {
                                    table_name,
                                    limit,
                                    offset,
                                } => {
                                    let table: web_common::database::Table =
                                        match table_name.as_str().try_into() {
                                            Ok(table) => table,
                                            Err(err) => {
                                                ctx.address().do_send(BackendMessage::Error(
                                                    task_id,
                                                    ApiError::BadRequest(vec![err.to_string()]),
                                                ));
                                                return;
                                            }
                                        };

                                    if !self.is_authenticated() && limit > 20 {
                                        ctx.address().do_send(BackendMessage::Error(
                                            task_id,
                                            ApiError::Unauthorized,
                                        ));
                                        return;
                                    }

                                    if self.is_authenticated() && limit > 100 {
                                        ctx.address().do_send(BackendMessage::Error(
                                            task_id,
                                            ApiError::BadRequest(vec![
                                                "Limit cannot exceed 100".to_string()
                                            ]),
                                        ));
                                        return;
                                    }

                                    ctx.address().do_send(BackendMessage::AllTable(
                                        task_id,
                                        <Table as AllTable>::all(
                                            &table,
                                            Some(limit),
                                            Some(offset),
                                            &mut self.diesel_connection,
                                        )
                                        .unwrap(),
                                    ));
                                }
                            },
                            web_common::database::Operation::Delete(table_name, primary_key) => {
                                let table: web_common::database::Table =
                                    match table_name.as_str().try_into() {
                                        Ok(table) => table,
                                        Err(err) => {
                                            ctx.address().do_send(BackendMessage::Error(
                                                task_id,
                                                ApiError::BadRequest(vec![err.to_string()]),
                                            ));
                                            return;
                                        }
                                    };

                                match <Table as DeletableTable>::delete(
                                    &table,
                                    primary_key,
                                    &mut self.diesel_connection,
                                ) {
                                    Ok(_) => {
                                        ctx.address().do_send(BackendMessage::Completed(task_id));
                                    }
                                    Err(err) => {
                                        ctx.address().do_send(BackendMessage::Error(task_id, err));
                                    }
                                }
                            }
                        }
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
