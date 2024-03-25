use std::fmt::Display;
use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum Role {
    Admin,
    Moderator,
    Editor,
    Creator,
    Viewer,
    /// A user that is not logged in. This is the lowest possible role requirement.
    Anonymous,
}

impl Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::Admin => write!(f, "Admin"),
            Role::Moderator => write!(f, "Moderator"),
            Role::Editor => write!(f, "Editor"),
            Role::Creator => write!(f, "Creator"),
            Role::Viewer => write!(f, "Viewer"),
            Role::Anonymous => write!(f, "Anonymous"),
        }
    }
}