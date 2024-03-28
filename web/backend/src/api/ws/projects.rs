pub mod new_project;
use crate::api::ws::socket::WebSocket;
use actix::Message;
use web_common::api::ws::messages::BackendMessage;

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub(crate) enum ProjectMessage {
    NewProject(uuid::Uuid, web_common::database::inserts::NewProject),
}

impl actix::Handler<ProjectMessage> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: ProjectMessage, ctx: &mut Self::Context) {
        match msg {
            ProjectMessage::NewProject(task_id, new_project) => {
                ctx.binary(BackendMessage::TaskResult(
                    task_id,
                    new_project::handle_new_project(new_project),
                ));
            }
        }
    }
}
