//! Submodule defining the websocket actor and its message handling.

use crate::models::User as DBUser;
use actix::ActorContext;
use actix::AsyncContext;
use actix::Message;
use actix::SpawnHandle;
use actix::WrapFuture;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use sqlx::{Pool as SQLxPool, Postgres};
use std::collections::HashMap;
use web_common::api::ws::messages::FormAction;
use web_common::api::ws::messages::SQLOperation;
use web_common::api::ApiError;

use web_common::api::ws::messages::{BackendMessage, FrontendMessage};

use super::channels::*;
use crate::api::ws::users::UserMessage;
use crate::DBPool;
use crate::DieselConn;

pub struct WebSocket {
    pg_handlers: HashMap<String, SpawnHandle>,
    pub(crate) user: Option<DBUser>,
    diesel: DBPool,
    pub(crate) diesel_connection: DieselConn,
    redis: redis::Client,
    sqlx: SQLxPool<Postgres>,
}

impl WebSocket {
    pub fn new(diesel: DBPool, sqlx: SQLxPool<Postgres>, redis: redis::Client) -> Self {
        Self {
            pg_handlers: HashMap::new(),
            user: None,
            diesel_connection: diesel.get().unwrap(),
            diesel,
            redis,
            sqlx,
        }
    }

    /// Ensure that the user is authenticated before allowing any further messages to be processed,
    /// killing the connection if the user is not authenticated.
    fn must_be_authenticated(
        &self,
        _ctx: &mut ws::WebsocketContext<Self>,
    ) -> Result<&DBUser, ApiError> {
        match self.user.as_ref() {
            Some(user) => Ok(user),
            None => Err(ApiError::unauthorized()),
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
    Authenticated(DBUser),
    Unauthorized,
    ExpiredToken,
}

impl actix::Handler<InternalMessage> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: InternalMessage, ctx: &mut Self::Context) {
        match msg {
            InternalMessage::Authenticated(user) => {
                let web_common_user = user.to_web_common_user(&mut self.diesel_connection);
                ctx.binary(BackendMessage::User(
                    SQLOperation::Update,
                    web_common_user.clone(),
                ));
                self.user = Some(user);
                // With the user authenticated, we can start to listen to its channels.
                let sqlx_pool = self.sqlx.clone();
                let recipient = ctx.address();
                let channel = Channel::NotifyUser(web_common_user);
                if !self.pg_handlers.contains_key(&channel.to_string()) {
                    self.pg_handlers.insert(
                        channel.to_string(),
                        ctx.spawn(
                            async move {
                                match start_listening(&sqlx_pool, vec![channel], recipient).await {
                                    Ok(_) => {}
                                    Err(err) => {
                                        log::error!(
                                            "Error starting to listen to channel: {:?}",
                                            err
                                        );
                                    }
                                }
                            }
                            .into_actor(self),
                        ),
                    );
                }
            }
            InternalMessage::Unauthorized => {
                ctx.close(Some(ws::CloseCode::Policy.into()));
            }
            InternalMessage::ExpiredToken => {
                ctx.binary(BackendMessage::ExpiredToken);
            }
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(msg) => {
                let frontend_message: FrontendMessage = msg.into();
                match frontend_message {
                    FrontendMessage::Close(_code) => {
                        ctx.stop();
                    }
                    FrontendMessage::Authentication(token) => {
                        let redis = self.redis.clone();
                        let diesel = self.diesel.clone();
                        let address = ctx.address();
                        ctx.spawn(
                            async move {
                                // If the token provided has only expired, we let the user know that
                                // they need to require a new one, without kicking them out.
                                match DBUser::is_token_expired(token.token()) {
                                    Ok(true) => {
                                        address.do_send(InternalMessage::ExpiredToken);
                                        return;
                                    }
                                    Ok(false) => {}
                                    Err(err) => {
                                        // If the deserilization of the token fails, we consider
                                        // the user completely unauthorized.
                                        log::error!("Error deserializing token: {:?}", err);
                                        address.do_send(InternalMessage::Unauthorized);
                                        return;
                                    }
                                }
                                if let Ok(user) =
                                    DBUser::from_bearer_token(redis, diesel, token.token()).await
                                {
                                    address.do_send(InternalMessage::Authenticated(user));
                                } else {
                                    address.do_send(InternalMessage::Unauthorized);
                                }
                            }
                            .into_actor(self),
                        );
                    }
                    FrontendMessage::Task(task_id, form_action) => {
                        if form_action.requires_authentication() {
                            match self.must_be_authenticated(ctx) {
                                Ok(_user) => match form_action {
                                    FormAction::CompleteProfile(profile) => {
                                        ctx.address().do_send(UserMessage::CompleteProfile(
                                            task_id,
                                            profile,
                                        ));
                                    }
                                },
                                Err(api_error) => {
                                    ctx.address().do_send(BackendMessage::TaskResult(
                                        task_id,
                                        Err(api_error),
                                    ));
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
