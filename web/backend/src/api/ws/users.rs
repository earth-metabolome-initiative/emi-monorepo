pub mod update_user_name;
use crate::api::ws::socket::WebSocket;
use actix::Message;
use validator::Validate;
use web_common::{
    api::ws::messages::BackendMessage,
    custom_validators::validation_errors::ValidationErrorToString,
};

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub(crate) enum UserMessage {
    CompleteProfile(
        uuid::Uuid,
        uuid::Uuid,
        web_common::api::auth::users::CompleteProfile,
    ),
}

impl actix::Handler<UserMessage> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: UserMessage, ctx: &mut Self::Context) {
        match msg {
            UserMessage::CompleteProfile(uuid, user_id, profile) => {
                // ctx.binary(BackendMessage::TaskResult(
                //     uuid,
                //     FormAction::CompleteProfile,
                //     update_user_name(
                //         &mut self.diesel_connection,
                //         self.user.as_ref().unwrap().clone(),
                //         user_id,
                //         name,
                //     ),
                // ));
                if let Err(e) = profile.validate() {
                    ctx.binary(BackendMessage::TaskResult(
                        uuid,
                        Err(web_common::api::ApiError::BadRequest(e.convert_to_string())),
                    ));
                } else {
                    ctx.binary(BackendMessage::TaskResult(
                        uuid,
                        Err(web_common::api::ApiError::BadRequest(vec![
                            "Test Error".to_string()
                        ])),
                    ));
                }
            }
        }
    }
}
