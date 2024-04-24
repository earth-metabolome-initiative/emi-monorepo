use crate::api::ws::socket::WebSocket;
use actix::Message;
use web_common::api::ws::messages::BackendMessage;

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub(crate) enum TaxaMessage {
    CompleteProfile(uuid::Uuid, web_common::database::updates::CompleteProfile),
}

impl actix::Handler<TaxaMessage> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: TaxaMessage, ctx: &mut Self::Context) {
        match msg {
            TaxaMessage::CompleteProfile(task_id, profile) => {
                ctx.binary(BackendMessage::TaskResult(
                    task_id,
                    self.user.as_ref().unwrap().0.update_profile(
                        &mut self.diesel_connection,
                        profile,
                    ),
                ));
            }
        }
    }
}
