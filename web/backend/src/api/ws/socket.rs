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

use web_common::api::ws::messages::{BackendMessage, FrontendMessage};

use super::channels::*;
use crate::DBPool;
use crate::DieselConn;

pub struct WebSocket {
    pg_handlers: HashMap<String, SpawnHandle>,
    user: Option<DBUser>,
    diesel: DBPool,
    redis: redis::Client,
    sqlx: SQLxPool<Postgres>,
}

impl WebSocket {
    pub fn new(diesel: DBPool, sqlx: SQLxPool<Postgres>, redis: redis::Client) -> Self {
        Self {
            pg_handlers: HashMap::new(),
            user: None,
            diesel,
            redis,
            sqlx,
        }
    }

    /// Ensure that the user is authenticated before allowing any further messages to be processed,
    /// killing the connection if the user is not authenticated.
    fn must_be_authenticated(
        &self,
        ctx: &mut ws::WebsocketContext<Self>,
    ) -> bool {
        if self.user.is_none() {
            ctx.close(Some(ws::CloseCode::Policy.into()));
        }
        self.user.is_some()
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
}

impl actix::Handler<InternalMessage> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: InternalMessage, ctx: &mut Self::Context) {
        match msg {
            InternalMessage::Authenticated(user) => {
                ctx.binary(BackendMessage::Authenticated(user.to_web_common_user()));
                self.user = Some(user);
            }
            InternalMessage::Unauthorized => {
                ctx.close(Some(ws::CloseCode::Policy.into()));
            }
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        log::debug!("Got message from WebSocket: {:?}", msg);
        match msg {
            Ok(msg) => {
                log::debug!("Got message from WebSocket: {:?}", msg);
                let frontend_message: FrontendMessage = msg.into();
                match frontend_message {
                    FrontendMessage::Close(code) => {
                        ctx.stop();
                    }
                    FrontendMessage::Authentication(token) => {
                        let redis = self.redis.clone();
                        let diesel = self.diesel.clone();
                        let address = ctx.address();
                        ctx.spawn(
                            async move {
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
                    _ => {
                        self.must_be_authenticated(ctx);
                        log::error!("Unhandled message: {:?}", frontend_message);
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
