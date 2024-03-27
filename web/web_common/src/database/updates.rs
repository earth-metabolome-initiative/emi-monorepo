pub mod update_profile;
use crate::database::Authorization;
use serde::{Deserialize, Serialize};
pub use update_profile::CompleteProfile;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Update {
    CompleteProfile(CompleteProfile),
}

impl Update {
    pub fn authorizations(&self) -> Vec<Authorization> {
        match self {
            Update::CompleteProfile(_) => {
                vec![Authorization::logged_user()]
            }
        }
    }
}

impl From<Update> for crate::database::Operation {
    fn from(update: Update) -> Self {
        crate::database::Operation::Update(update)
    }
}
