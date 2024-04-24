use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
use chrono::NaiveDateTime;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "frontend", derive(yew::html::Properties))]
pub struct PublicUser {
    pub id: i32,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub thumbnail_id: Uuid,
    pub picture_id: Uuid,
}
