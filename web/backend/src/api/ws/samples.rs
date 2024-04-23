pub mod new_sample;
use crate::api::ws::socket::WebSocket;
use actix::Message;
use web_common::api::ws::messages::BackendMessage;

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub(crate) enum SampleMessage {
    NewSample(uuid::Uuid, web_common::database::inserts::NewSample),
}

impl actix::Handler<SampleMessage> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: SampleMessage, ctx: &mut Self::Context) {
        match msg {
            SampleMessage::NewSample(task_id, new_sample) => {
                ctx.binary(BackendMessage::TaskResult(
                    task_id,
                    new_sample::handle_new_sample(new_sample),
                ));
            }
        }
    }
}
