pub mod new_project;
use crate::database::Authorization;
pub use new_project::NewProject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Insert {
    Project(new_project::NewProject),
}

impl Insert {
    pub fn authorizations(&self) -> Vec<Authorization> {
        vec![]
    }

    /// Returns whether the insert can be performed offline.
    pub fn offline(&self) -> bool {
        false
    }
}

impl From<Insert> for crate::database::Operation {
    fn from(update: Insert) -> Self {
        crate::database::Operation::Insert(update)
    }
}
