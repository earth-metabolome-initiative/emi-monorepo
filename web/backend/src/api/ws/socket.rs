//! Submodule defining the websocket actor and its message handling.

use std::mem::swap;

use crate::api::ws::users::UserMessage;
use crate::models::Notification;
use crate::models::User;
use crate::nested_models::NestedPublicUser;
use crate::traits::bincode_serialize::BincodeSerialize;
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
use web_common::database::NotificationMessage;
use web_common::database::Table;

use super::projects::ProjectMessage;
use super::samples::SampleMessage;
use super::teams::TeamMessage;

#[derive(Debug, Deserialize, Serialize)]
struct NotificationRecord {
    /// The notification's metadata.
    notification: Notification,
    /// The unserialized record, which we cannot de-serialize
    /// a priori before deserializing the notification above.
    record: String
}

pub struct WebSocket {
    notifications_handler: Option<SpawnHandle>,
    pub(crate) user: Option<(User, AccessToken)>,
    diesel: DBPool,
    pub(crate) diesel_connection: DieselConn,
    redis: redis::Client,
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
            redis,
            sqlx,
            continuation_bytes: Vec::new(),
        }
    }

    fn user(&self) -> Option<&User> {
        self.user.as_ref().map(|(user, _)| user)
    }

    fn access_token(&self) -> Option<&AccessToken> {
        self.user.as_ref().map(|(_, access_token)| access_token)
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
                                            ).unwrap()
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
                    ctx.address().do_send(BackendMessage::RefreshToken((
                        user.into(),
                        self.access_token().unwrap().clone(),
                    )));
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
                    FrontendMessage::Task(task) => {
                        if task.requires_authentication() && self.user.is_none() {
                            ctx.address().do_send(InternalMessage::Unauthorized);
                            return;
                        }
                        match task.operation() {
                            web_common::database::Operation::Insert(insert) => match insert {
                                web_common::database::Insert::Project(new_project) => {
                                    ctx.address().do_send(ProjectMessage::NewProject(
                                        task.id(),
                                        new_project.clone(),
                                    ));
                                }
                                web_common::database::Insert::Sample(new_sample) => {
                                    ctx.address().do_send(SampleMessage::NewSample(
                                        task.id(),
                                        new_sample.clone(),
                                    ));
                                }
                                web_common::database::Insert::Team(new_team) => {
                                    ctx.address().do_send(TeamMessage::NewTeam(
                                        task.id(),
                                        new_team.clone(),
                                    ));
                                }
                            },
                            web_common::database::Operation::Update(update) => match update {
                                web_common::database::Update::CompleteProfile(profile) => {
                                    ctx.address().do_send(UserMessage::CompleteProfile(
                                        task.id(),
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
                                    let table: web_common::database::Table = match table_name
                                        .as_str()
                                        .try_into()
                                    {
                                        Ok(table) => table,
                                        Err(err) => {
                                            ctx.address().do_send(BackendMessage::TaskResult(
                                                task.id(),
                                                Err(ApiError::BadRequest(vec![err.to_string()])),
                                            ));
                                            return;
                                        }
                                    };
                                    ctx.address().do_send(BackendMessage::SearchTable(
                                        task.id(),
                                        match table {
                                            web_common::database::Table::Projects => {
                                                crate::nested_models::NestedProject::similarity_search(
                                                    &query,
                                                    Some(*number_of_results as i32),
                                                    &mut self.diesel_connection,
                                                )
                                                .bincode_serialize()
                                            }
                                            web_common::database::Table::ProjectStates => {
                                                crate::models::ProjectState::similarity_search(
                                                    &query,
                                                    Some(*number_of_results as i32),
                                                    &mut self.diesel_connection,
                                                )
                                                .bincode_serialize()
                                            }
                                            web_common::database::Table::SampleStates => {
                                                crate::models::SampleState::similarity_search(
                                                    &query,
                                                    Some(*number_of_results as i32),
                                                    &mut self.diesel_connection,
                                                )
                                                .bincode_serialize()
                                            }
                                            web_common::database::Table::PublicUsers => {
                                                crate::nested_models::NestedPublicUser::similarity_search(
                                                    &query,
                                                    Some(*number_of_results as i32),
                                                    &mut self.diesel_connection,
                                                )
                                                .bincode_serialize()
                                            }
                                            web_common::database::Table::BioOttTaxonItems => {
                                                crate::nested_models::NestedBioOttTaxonItem::strict_word_similarity_search(
                                                    &query,
                                                    Some(*number_of_results as i32),
                                                    &mut self.diesel_connection,
                                                )
                                                .bincode_serialize()
                                            }
                                            web_common::database::Table::SamplingProcedures => {
                                                crate::models::SamplingProcedure::similarity_search(
                                                    &query,
                                                    Some(*number_of_results as i32),
                                                    &mut self.diesel_connection,
                                                )
                                                .bincode_serialize()
                                            }
                                            _ => {
                                                unimplemented!("Table not implemented: {:?}", table)
                                            }
                                        },
                                    ));
                                }
                                _ => {
                                    unimplemented!(
                                        "Operation not implemented: {:?}",
                                        task.operation()
                                    )
                                }
                            },
                            _ => {
                                unimplemented!("Operation not implemented: {:?}", task.operation())
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
