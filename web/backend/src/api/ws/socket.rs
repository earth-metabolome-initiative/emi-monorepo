//! Submodule defining the websocket actor and its message handling.

use std::mem::swap;

use crate::database::*;
use actix::ActorContext;
use actix::AsyncContext;
use actix::SpawnHandle;
use actix::WrapFuture;
use actix::{Actor, StreamHandler};
use actix_web::web::Bytes;
use actix_web_actors::ws;
use sqlx::postgres::PgListener;
use sqlx::{Pool as SQLxPool, Postgres};
use web_common::api::oauth::jwt_cookies::AccessToken;
use web_common::api::ws::messages::{BackendMessage, FrontendMessage};
use web_common::api::ApiError;
use web_common::database::NotificationMessage;
use web_common::database::Table;

pub struct WebSocket {
    notifications_handler: Option<SpawnHandle>,
    pub(crate) user: Option<(NestedUser, AccessToken)>,
    diesel: DBPool,
    pub(crate) diesel_connection: DieselConn,
    _redis: redis::Client,
    sqlx: SQLxPool<Postgres>,
    continuation_bytes: Vec<u8>,
}

impl WebSocket {
    pub fn new(
        diesel: DBPool,
        sqlx: SQLxPool<Postgres>,
        redis: redis::Client,
    ) -> Result<Self, ApiError> {
        Ok(Self {
            notifications_handler: None,
            user: None,
            diesel_connection: diesel.get().map_err(ApiError::from)?,
            diesel,
            _redis: redis,
            sqlx,
            continuation_bytes: Vec::new(),
        })
    }

    fn user(&self) -> Option<&NestedUser> {
        self.user.as_ref().map(|(user, _)| user)
    }

    pub fn authenticated(
        diesel: DBPool,
        sqlx: SQLxPool<Postgres>,
        redis: redis::Client,
        user: NestedUser,
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

    fn listen_for_notifications(
        &mut self,
        ctx: &mut <WebSocket as Actor>::Context,
    ) -> Result<(), ApiError> {
        // If the handler is stopped or was never started, start it.
        if self.notifications_handler.is_none() {
            if let Some((user, _)) = &self.user {
                log::info!("Starting notifications handler for user {}", user.inner.id);
                let address = ctx.address().clone();
                let channel_name = format!("user_{}", user.inner.id);
                let pool = self.sqlx.clone();
                let mut diesel_connection = self.diesel.get().map_err(ApiError::from)?;
                let maybe_user_id = self.user().map(|user| user.inner.id);
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
                                    let notification: Notification =
                                        serde_json::from_str(&notification_payload).unwrap();

                                    let table: Table =
                                        notification.table_name.as_str().try_into().unwrap();

                                    let serialized_record: Vec<u8> = table
                                        .from_flat_str(
                                            &notification.record,
                                            maybe_user_id,
                                            &mut diesel_connection,
                                        )
                                        .unwrap();

                                    address.do_send(BackendMessage::Notification(
                                        NotificationMessage::new(
                                            notification.into(),
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
        Ok(())
    }
}

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let _ = self.listen_for_notifications(ctx);
        if self.is_authenticated() {
            ctx.address().do_send(BackendMessage::RefreshUser(
                self.user().unwrap().clone().into(),
            ));
        }
    }
}

impl actix::Handler<BackendMessage> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: BackendMessage, ctx: &mut Self::Context) {
        ctx.binary(msg);
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
                                    self.user().map(|user| user.inner.id).unwrap(),
                                    &mut self.diesel_connection,
                                ) {
                                    Ok(row) => {
                                        ctx.address()
                                            .do_send(BackendMessage::Completed(task_id, Some(row)));
                                    }
                                    Err(err) => {
                                        ctx.address().do_send(BackendMessage::Error(task_id, err));
                                    }
                                }
                            }
                            web_common::database::Operation::Update(table_name, row_to_update) => {
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
                                match <Table as UpdatableTable>::update(
                                    &table,
                                    row_to_update,
                                    self.user().map(|user| user.inner.id).unwrap(),
                                    &mut self.diesel_connection,
                                ) {
                                    Ok(row) => {
                                        ctx.address()
                                            .do_send(BackendMessage::Completed(task_id, Some(row)));
                                    }
                                    Err(err) => {
                                        ctx.address().do_send(BackendMessage::Error(task_id, err));
                                    }
                                }
                            }
                            web_common::database::Operation::Select(select) => match select {
                                web_common::database::Select::SearchAll { query, limit } => {
                                    match crate::search_all::search_all(
                                        self.user().map(|user| user.inner.id),
                                        &query,
                                        Some(limit),
                                        &mut self.diesel_connection,
                                    ) {
                                        Ok(results) => {
                                            ctx.address().do_send(BackendMessage::SearchTable(
                                                task_id,
                                                bincode::serialize(&results).unwrap(),
                                            ));
                                        }
                                        Err(err) => {
                                            ctx.address()
                                                .do_send(BackendMessage::Error(task_id, err));
                                        }
                                    }
                                }
                                web_common::database::Select::CanView {
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
                                    match <Table as BackendTable>::can_view_by_id(
                                        &table,
                                        primary_key,
                                        self.user().map(|user| user.inner.id),
                                        &mut self.diesel_connection,
                                    ) {
                                        Ok(can_view) => {
                                            ctx.address().do_send(BackendMessage::CanView(
                                                task_id, can_view,
                                            ));
                                        }
                                        Err(err) => {
                                            ctx.address()
                                                .do_send(BackendMessage::Error(task_id, err));
                                        }
                                    }
                                }
                                web_common::database::Select::CanUpdate {
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
                                    match <Table as BackendTable>::can_update_by_id(
                                        &table,
                                        primary_key,
                                        self.user().map(|user| user.inner.id).unwrap(),
                                        &mut self.diesel_connection,
                                    ) {
                                        Ok(can_update) => {
                                            ctx.address().do_send(BackendMessage::CanUpdate(
                                                task_id, can_update,
                                            ));
                                        }
                                        Err(err) => {
                                            ctx.address()
                                                .do_send(BackendMessage::Error(task_id, err));
                                        }
                                    }
                                }
                                web_common::database::Select::CanDelete {
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
                                    match <Table as BackendTable>::can_admin_by_id(
                                        &table,
                                        primary_key,
                                        self.user().map(|user| user.inner.id).unwrap(),
                                        &mut self.diesel_connection,
                                    ) {
                                        Ok(can_admin) => {
                                            ctx.address().do_send(BackendMessage::CanDelete(
                                                task_id, can_admin,
                                            ));
                                        }
                                        Err(err) => {
                                            ctx.address()
                                                .do_send(BackendMessage::Error(task_id, err));
                                        }
                                    }
                                }
                                web_common::database::selects::Select::SearchTable {
                                    filter,
                                    table_name,
                                    query,
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
                                    match table.strict_word_similarity_search_viewable(
                                        filter,
                                        self.user().map(|user| user.inner.id),
                                        &query,
                                        Some(limit),
                                        Some(offset),
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
                                web_common::database::Select::SearchEditableTable {
                                    filter,
                                    table_name,
                                    query,
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
                                    match table.strict_word_similarity_search_updatable(
                                        filter,
                                        self.user().map(|user| user.inner.id).unwrap(),
                                        &query,
                                        Some(limit),
                                        Some(offset),
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
                                    operation_name,
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
                                    match <Table as BackendTable>::get(
                                        &table,
                                        primary_key,
                                        self.user().map(|user| user.inner.id),
                                        &mut self.diesel_connection,
                                    ) {
                                        Ok(record) => {
                                            ctx.address().do_send(BackendMessage::GetTable(
                                                task_id,
                                                operation_name,
                                                record,
                                            ));
                                        }
                                        Err(err) => {
                                            ctx.address()
                                                .do_send(BackendMessage::Error(task_id, err));
                                        }
                                    }
                                }
                                web_common::database::Select::All {
                                    table_name,
                                    filter,
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

                                    if !self.is_authenticated() && limit > 50 {
                                        ctx.address().do_send(BackendMessage::Error(
                                            task_id,
                                            ApiError::Unauthorized,
                                        ));
                                        return;
                                    }

                                    if self.is_authenticated() && limit > 500 {
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
                                        <Table as BackendTable>::all_viewable(
                                            &table,
                                            filter,
                                            self.user().map(|user| user.inner.id),
                                            Some(limit),
                                            Some(offset),
                                            &mut self.diesel_connection,
                                        )
                                        .unwrap(),
                                    ));
                                }
                                web_common::database::Select::AllByUpdatedAt {
                                    table_name,
                                    filter,
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

                                    if !self.is_authenticated() && limit > 50 {
                                        ctx.address().do_send(BackendMessage::Error(
                                            task_id,
                                            ApiError::Unauthorized,
                                        ));
                                        return;
                                    }

                                    if self.is_authenticated() && limit > 500 {
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
                                        <Table as BackendTable>::all_viewable_sorted(
                                            &table,
                                            filter,
                                            self.user().map(|user| user.inner.id),
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

                                match <Table as BackendTable>::delete_by_id(
                                    &table,
                                    primary_key,
                                    self.user().map(|user| user.inner.id).unwrap(),
                                    &mut self.diesel_connection,
                                ) {
                                    Ok(_) => {
                                        ctx.address()
                                            .do_send(BackendMessage::Completed(task_id, None));
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
