pub mod complete_profile;
use crate::api::ws::socket::WebSocket;
use actix::Message;
use complete_profile::*;
use web_common::api::ws::messages::BackendMessage;

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub(crate) enum UserMessage {
    CompleteProfile(uuid::Uuid, web_common::api::auth::users::CompleteProfile),
}

impl actix::Handler<UserMessage> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: UserMessage, ctx: &mut Self::Context) {
        match msg {
            UserMessage::CompleteProfile(task_id, profile) => {
                ctx.binary(BackendMessage::TaskResult(
                    task_id,
                    complete_profile(
                        &mut self.diesel_connection,
                        self.user.as_ref().unwrap(),
                        profile,
                    ),
                ));
            }
        }
    }
}
