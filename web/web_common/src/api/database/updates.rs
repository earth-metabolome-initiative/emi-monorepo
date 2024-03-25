pub mod update_profile;
pub use update_profile::CompleteProfile;
use serde::{Deserialize, Serialize};

use super::Id;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Update {
    Profile(CompleteProfile),
}

impl Update {
    pub fn id(&self) -> Id {
        match self {
            Update::Profile(update) => update.id().into(),
        }
    }
}