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
                match new_project::handle_new_project(new_project) {
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
