use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub id: u64,
}
