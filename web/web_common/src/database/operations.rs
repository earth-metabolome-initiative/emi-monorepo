use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::roles::Role;
use super::selects::Select;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Authorization {
    Editable(Uuid, Vec<Role>),
    LoggedUser,
}

impl Authorization {
    pub fn new(id: Uuid, roles: Vec<Role>) -> Self {
        Self::Editable(id, roles)
    }

    pub fn logged_user() -> Self {
        Self::LoggedUser
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy)]
pub enum PrimaryKey {
    Uuid(Uuid),
    Int(i32),
}

impl From<PrimaryKey> for Uuid {
    fn from(pk: PrimaryKey) -> Self {
        match pk {
            PrimaryKey::Uuid(uuid) => uuid,
            PrimaryKey::Int(int) => unreachable!("Cannot convert PrimaryKey::Int to Uuid: {}", int),
        }
    }
}

impl From<PrimaryKey> for i32 {
    fn from(pk: PrimaryKey) -> Self {
        match pk {
            PrimaryKey::Uuid(uuid) => unreachable!("Cannot convert PrimaryKey::Uuid to i32: {}", uuid),
            PrimaryKey::Int(int) => int,
        }
    }
}

impl From<Uuid> for PrimaryKey {
    fn from(uuid: Uuid) -> Self {
        PrimaryKey::Uuid(uuid)
    }
}

impl From<i32> for PrimaryKey {
    fn from(int: i32) -> Self {
        PrimaryKey::Int(int)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Operation {
    // When the frontend selects something to show to
    // the user, it sends a Select message to the backend
    // for a specific view.
    Select(Select),
    // When the frontend wants to delete a row from a table
    // it sends a Delete message to the backend.
    Delete(String, PrimaryKey),
    // When the frontend wants to update a row from a table
    // it sends a variant of the Update enumeration.
    Update(String, Vec<u8>),
    // When the frontend wants to insert a row into a table
    // it sends a variant of the Insert enumeration.
    Insert(String, Vec<u8>),
}

impl Operation {
    pub fn authorizations(&self) -> Vec<Authorization> {
        // match self {
        //     Operation::Select(select) => select.authorizations(),
        //     Operation::Delete(id, _table) => {
        //         vec![Authorization::new(*id, vec![Role::Admin, Role::Creator])]
        //     }
        //     Operation::Update(update) => update.authorizations(),
        //     Operation::Insert(insert) => insert.authorizations(),
        // }
        // TODO!
        vec![]
    }

    /// Returns whether the current operation is an insert.
    pub fn is_insert(&self) -> bool {
        match self {
            Operation::Insert(_, _) => true,
            _ => false,
        }
    }

    /// Returns whether the current operation is a delete.
    pub fn is_delete(&self) -> bool {
        match self {
            Operation::Delete(_, _) => true,
            _ => false,
        }
    }

    /// Returns whether the current operation is an update.
    pub fn is_update(&self) -> bool {
        match self {
            Operation::Update(_, _) => true,
            _ => false,
        }
    }

    pub fn requires_authentication(&self) -> bool {
        if self.is_insert() || self.is_delete() || self.is_update() {
            return true;
        }

        self.authorizations().iter().any(|auth| match auth {
            Authorization::LoggedUser => true,
            Authorization::Editable(_, roles) => !roles.contains(&Role::Anonymous),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: Uuid,
    pub start: chrono::DateTime<chrono::Utc>,
    pub attempts: u8,
    pub operation: Operation,
}

impl From<Operation> for Task {
    fn from(operation: Operation) -> Self {
        Task {
            id: Uuid::new_v4(),
            start: chrono::Utc::now(),
            attempts: 0,
            operation,
        }
    }
}

impl Task {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn operation(&self) -> &Operation {
        &self.operation
    }

    pub fn increase_attemps(&mut self) {
        self.attempts += 1;
    }

    /// Returns whether the task should be retried.
    ///
    /// # Implementative details
    /// Depending on whether enough time has passed, determined as
    /// 2^attempts * 5 seconds, the task should be retried.
    pub fn should_retry(&self) -> bool {
        if self.attempts == 0 {
            return true;
        }
        let elapsed = chrono::Utc::now() - self.start;
        let retry_time = (2u32.pow(self.attempts as u32) * 20).max(60*10);
        elapsed.num_seconds() > retry_time as i64
    }
}

