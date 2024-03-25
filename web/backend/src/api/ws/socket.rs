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
use web_common::api::database::operations::Operation;
use web_common::api::oauth::jwt_cookies::AccessToken;
use web_common::api::ws::messages::FormAction;

use web_common::api::ws::messages::{BackendMessage, FrontendMessage};

use super::channels::*;
use crate::api::ws::users::UserMessage;
use crate::DBPool;
use crate::DieselConn;

pub struct WebSocket {
    pg_handlers: HashMap<Vec<Channel>, SpawnHandle>,
    pub(crate) user: Option<(DBUser, AccessToken)>,
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

    pub fn authenticated(
        diesel: DBPool,
        sqlx: SQLxPool<Postgres>,
        redis: redis::Client,
        user: DBUser,
        access_token: AccessToken,
    ) -> Self {
        let diesel_connection = diesel.get().unwrap();
        Self {
            pg_handlers: HashMap::new(),
            user: Some((user, access_token)),
            diesel_connection,
            diesel,
            redis,
            sqlx,
        }
    }

    pub fn listen_to_channel(
        &mut self,
        channel: Vec<Channel>,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        if !self.pg_handlers.contains_key(&channel) {
            let sqlx_pool = self.sqlx.clone();
            let recipient = ctx.address();
            let channel_clone = channel.clone();

            let channel_handler = ctx.spawn(
                async move {
                    match start_listening(&sqlx_pool, channel_clone, recipient).await {
                        Ok(_) => {}
                        Err(err) => {
                            log::error!("Error starting to listen to channel: {:?}", err);
                        }
                    }
                }
                .into_actor(self),
            );

            self.pg_handlers.insert(channel, channel_handler);
        }
    }

    pub fn get_default_channels(&self) -> Vec<Vec<Channel>> {
        if let Some((user, _)) = &self.user {
            vec![vec![Channel::NotifyUser(user.id)]]
        } else {
            Vec::new()
        }
    }

    pub fn enable_default_channels(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        for channel in self.get_default_channels() {
            self.listen_to_channel(channel, ctx);
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
    ExpiredToken,
}

impl actix::Handler<InternalMessage> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: InternalMessage, ctx: &mut Self::Context) {
        match msg {
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
        self.enable_default_channels(ctx);
        
        match msg {
            Ok(msg) => {
                let frontend_message: FrontendMessage = msg.into();
                match frontend_message {
                    FrontendMessage::Close(_code) => {
                        ctx.stop();
                    }
                    FrontendMessage::Task(task_id, form_action) => {
                        if form_action.requires_authentication() {
                            match self.must_be_authenticated(ctx) {
                                Ok(_user) => match form_action {
                                    FormAction::CompleteProfile(profile) => {
                                        ctx.address().do_send(UserMessage::CompleteProfile(
                                            task_id, profile,
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
