use crate::api::ws::socket::WebSocket;
use actix::Message;
use web_common::api::ws::messages::BackendMessage;

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub(crate) enum UserMessage {
    CompleteProfile(uuid::Uuid, web_common::database::updates::CompleteProfile),
}

impl actix::Handler<UserMessage> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: UserMessage, ctx: &mut Self::Context) {
        match msg {
            UserMessage::CompleteProfile(task_id, profile) => {
                match self.user.as_ref().unwrap().0.update_profile(
                    &mut self.diesel_connection,
                    profile,
                ) {
                    Ok(_) => {
                        ctx.binary(BackendMessage::Completed(task_id));
                    }
                    Err(e) => {
                        ctx.binary(BackendMessage::Error(task_id, e));
                    }
                }
            }
        }
    }
}
