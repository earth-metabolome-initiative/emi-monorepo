//! Submodule defining the queries that can be made to the database.
pub mod public_user;
pub use public_user::PublicUser;
use serde::{Deserialize, Serialize};
use super::Id;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Query {
    PublicUser(Id)
}

impl Query {
    pub fn id(&self) -> Id {
        match self {
            Query::PublicUser(id) => *id
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Answer {
    PublicUser(PublicUser)
}