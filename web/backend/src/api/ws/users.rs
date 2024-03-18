pub mod update_user_name;
use crate::api::ws::socket::WebSocket;
use actix::Message;
pub use update_user_name::update_user_name;
use web_common::api::ws::messages::BackendMessage;
use web_common::api::ws::messages::FormAction;

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub(crate) enum UserMessage {
    UpdateName(
        uuid::Uuid,
        uuid::Uuid,
        web_common::api::auth::users::name::Name,
    ),
}

impl actix::Handler<UserMessage> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: UserMessage, ctx: &mut Self::Context) {
        match msg {
            UserMessage::UpdateName(uuid, user_id, name) => {
                // ctx.binary(BackendMessage::TaskResult(
                //     uuid,
                //     FormAction::UpdateName,
                //     update_user_name(
                //         &mut self.diesel_connection,
                //         self.user.as_ref().unwrap().clone(),
                //         user_id,
                //         name,
                //     ),
                // ));
                ctx.binary(BackendMessage::TaskResult(
                    uuid,
                    FormAction::UpdateName,
                    Err(web_common::api::ApiError::BadRequest(vec!["Test Error".to_string()]))
                ));
            }
        }
    }
}
