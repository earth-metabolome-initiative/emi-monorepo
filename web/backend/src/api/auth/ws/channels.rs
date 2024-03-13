use actix_web::web;
use core::fmt::Debug;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use sqlx::Postgres;
use sqlx::Pool;
use sqlx::error::Error;
use sqlx::postgres::PgListener;


pub trait Channel: ToString {
    type Payload: DeserializeOwned + Debug;
}

// pub struct CommentsChannel;

// impl ToString for CommentsChannel {
//     fn to_string(&self) -> String {
//         "comments".to_string()
//     }
// }

// impl Channel for CommentsChannel {
//     type Payload = CommentsPayload;
// }

// pub struct CommentsUserChannel {
//     pub user: User,
// }

// impl CommentsUserChannel {
//     pub fn new(user: User) -> Self {
//         Self { user }
//     }
// }

// impl ToString for CommentsUserChannel {
//     fn to_string(&self) -> String {
//         format!("comments_{}", self.user.id)
//     }
// }

// impl Channel for CommentsUserChannel {
//     type Payload = CommentsPayload;
// }

// #[derive(Deserialize, Debug)]
// pub struct CommentsPayload {
//     pub action_type: ActionType,
//     pub id: i32,
//     pub user_id: i32,
//     pub body: String,
// }

// impl Into<commons::comments::Comment> for CommentsPayload {
//     fn into(self) -> commons::comments::Comment {
//         commons::comments::Comment {
//             id: self.id,
//             user_id: self.user_id,
//             body: self.body,
//         }
//     }
// }

pub async fn start_listening<Ch: Channel>(
    pool: &Pool<Postgres>,
    channel: Ch,
    mut call_back: impl FnMut(Ch::Payload),
) -> Result<(), Error> {
    // Initiate the logger.
    let mut listener = PgListener::connect_with(pool).await.unwrap();
    listener
        .listen_all(vec![channel.to_string().as_str()])
        .await?;
    log::info!("Listening to channel: {}", channel.to_string());
    loop {
        while let Some(notification) = listener.try_recv().await? {
            log::info!(
                "Getting notification with payload: {:?} from channel {:?}",
                notification.payload(),
                notification.channel()
            );

            let notification_payload: String = notification.payload().to_owned();

            call_back(serde_json::from_str::<Ch::Payload>(&notification_payload).unwrap());
        }
    }
}
