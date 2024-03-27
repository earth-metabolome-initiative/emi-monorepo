use crate::database::Authorization;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Insert {}

impl Insert {
    pub fn authorizations(&self) -> Vec<Authorization> {
        vec![]
    }
}
