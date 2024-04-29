pub mod new_team;
use crate::api::ws::socket::WebSocket;
use actix::Message;
use web_common::api::ws::messages::BackendMessage;

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub(crate) enum TeamMessage {
    NewTeam(uuid::Uuid, web_common::database::inserts::NewTeam),
}

impl actix::Handler<TeamMessage> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: TeamMessage, ctx: &mut Self::Context) {
        match msg {
            TeamMessage::NewTeam(task_id, new_team) => {
                ctx.binary(BackendMessage::TaskResult(
                    task_id,
                    new_team::handle_new_team(new_team),
                ));
            }
        }
    }
}
