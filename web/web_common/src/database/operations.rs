use crate::api::ws::messages::FrontendMessage;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::inserts::Insert;
use super::roles::Role;
use super::selects::Select;
use super::Table;
use super::updates::Update;

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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Operation {
    // When the frontend selects something to show to
    // the user, it sends a Select message to the backend
    // for a specific view.
    Select(Select),
    // When the frontend wants to delete a row from a table
    // it sends a Delete message to the backend. It cannot
    // delete a view because views are not real tables.
    Delete(uuid::Uuid, Table),
    // When the frontend wants to update a row from a table
    // it sends a variant of the Update enumeration.
    Update(Update),
    // When the frontend wants to insert a row into a table
    // it sends a variant of the Insert enumeration.
    Insert(Insert),
}

impl Operation {
    pub fn authorizations(&self) -> Vec<Authorization> {
        match self {
            Operation::Select(select) => select.authorizations(),
            Operation::Delete(id, _table) => {
                vec![Authorization::new(*id, vec![Role::Admin, Role::Creator])]
            }
            Operation::Update(update) => update.authorizations(),
            Operation::Insert(insert) => insert.authorizations(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Task {
    id: Uuid,
    start: chrono::DateTime<chrono::Utc>,
    attempts: u8,
    operation: Operation,
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

impl From<Operation> for FrontendMessage {
    fn from(operation: Operation) -> Self {
        let task: Task = operation.into();
        task.into()
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

    pub fn authorizations(&self) -> Vec<Authorization> {
        self.operation.authorizations()
    }

    pub fn requires_authentication(&self) -> bool {
        self.authorizations().iter().any(|auth| match auth {
            Authorization::LoggedUser => true,
            Authorization::Editable(_, roles) => !roles.contains(&Role::Anonymous),
        })
    }
}

impl From<Task> for FrontendMessage {
    fn from(task: Task) -> Self {
        FrontendMessage::Task(task)
    }
}
