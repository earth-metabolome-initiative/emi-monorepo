use serde::{Deserialize, Serialize};
use crate::api::database::roles::Role;
use crate::api::database::rows::Row;
use crate::api::database::new_rows::NewRow;
use crate::api::database::selects::Query;

use super::updates::Update;
use super::Id;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Operation {
    #[serde(rename = "INSERT")]
    Insert(NewRow),
    #[serde(rename = "UPDATE")]
    Update(Update),
    #[serde(rename = "DELETE")]
    Delete(Row),
    #[serde(rename = "SELECT")]
    Select(Query),
}

impl Operation {
    pub fn is_insert(&self) -> bool {
        match self {
            Operation::Insert(_) => true,
            _ => false,
        }
    }

    pub fn is_update(&self) -> bool {
        match self {
            Operation::Update(_) => true,
            _ => false,
        }
    }

    pub fn is_delete(&self) -> bool {
        match self {
            Operation::Delete(_) => true,
            _ => false,
        }
    }

    pub fn is_select(&self) -> bool {
        match self {
            Operation::Select(_) => true,
            _ => false,
        }
    }

    pub fn default_roles_for_operation(&self) -> Vec<Role> {
        match self {
            Operation::Insert(_) => vec![],
            Operation::Update(_) => {
                vec![Role::Creator, Role::Editor, Role::Moderator, Role::Admin]
            }
            Operation::Delete(_) => {
                vec![Role::Creator, Role::Editor, Role::Moderator, Role::Admin]
            }
            Operation::Select(_) => vec![
                Role::Viewer,
                Role::Creator,
                Role::Editor,
                Role::Moderator,
                Role::Admin,
            ],
        }
    }

    pub fn id(&self) -> Option<Id> {
        match self {
            Operation::Insert(_) => None,
            Operation::Update(row) => Some(row.id()),
            Operation::Delete(row) => row.id(),
            Operation::Select(row) => Some(row.id()),
        }
    }
}