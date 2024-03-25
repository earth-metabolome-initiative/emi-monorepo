//! Submodule providing structs relative to the database.
pub mod roles;
pub mod rows;
pub mod new_rows;
pub mod operations;
pub mod selects;
pub mod updates;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Id {
    Uuid(uuid::Uuid),
    LoggedInUser,
}

impl From<uuid::Uuid> for Id {
    fn from(uuid: uuid::Uuid) -> Self {
        Id::Uuid(uuid)
    }
}

impl From<Id> for uuid::Uuid {
    fn from(id: Id) -> Self {
        match id {
            Id::Uuid(uuid) => uuid,
            Id::LoggedInUser => panic!("Cannot convert LoggedInUser to Uuid"),
        }
    }
}