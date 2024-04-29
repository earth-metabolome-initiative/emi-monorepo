pub mod new_project;
pub mod new_sample;
use crate::database::Authorization;
pub use new_project::NewProject;
pub use new_sample::NewSample;
pub mod new_team;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Insert {
    Project(new_project::NewProject),
    Sample(new_sample::NewSample),
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
