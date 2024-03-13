//! Submodule defining the websocket actor and its message handling.

use crate::models::User as DBUser;
use actix::ActorContext;
use actix::AsyncContext;
use actix::SpawnHandle;
use actix::WrapFuture;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use sqlx::{Pool as SQLxPool, Postgres};
use std::collections::HashMap;
use web_common::api::auth::ws::messages::{BackendMessage, FrontendMessage};

use super::channels::*;
use crate::DieselConn;

pub struct WebSocket {
    pg_handlers: HashMap<String, SpawnHandle>,
    user: DBUser,
    diesel: DieselConn,
    sqlx: SQLxPool<Postgres>,
}

impl WebSocket {
    pub fn new(user: DBUser, diesel: DieselConn, sqlx: SQLxPool<Postgres>) -> Self {
        Self {
            pg_handlers: HashMap::new(),
            user,
            diesel,
            sqlx,
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

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(msg) => {
                log::info!("Got message from WebSocket: {:?}", msg);
                let frontend_message: FrontendMessage = msg.into();
                match frontend_message {
                    FrontendMessage::Close(code) => {
                        ctx.stop();
                    }
                    _ => {
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
