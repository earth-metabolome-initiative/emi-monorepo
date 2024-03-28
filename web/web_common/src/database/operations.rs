use crate::api::ws::messages::FrontendMessage;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::inserts::Insert;
use super::roles::Role;
use super::tables::Table;
use super::updates::Update;
use super::views::View;

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

impl View {
    pub fn roles(&self) -> Vec<Role> {
        match self {
            View::DocumentsView => vec![Role::Viewer],
            View::PublicUser => vec![Role::Anonymous],
            View::EditsView => vec![Role::Editor],
            View::LastEditsView => vec![Role::Editor],
            View::FormatsView => vec![Role::Viewer],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Operation {
    // When the frontend selects something to show to
    // the user, it sends a Select message to the backend
    // for a specific view.
    Select(uuid::Uuid, View),
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
            Operation::Select(id, view) => {
                vec![Authorization::new(*id, view.roles())]
            }
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

impl Task {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn operation(&self) -> &Operation {
        &self.operation
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
        let retry_time = 2u32.pow(self.attempts as u32) * 5;
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
