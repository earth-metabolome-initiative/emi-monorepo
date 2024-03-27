use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
pub enum Role {
    Admin,
    Moderator,
    Editor,
    Creator,
    Viewer,
    /// A user that is not logged in. This is the lowest possible role requirement.
    Anonymous,
}

impl Role {
    pub fn child_roles(&self) -> Vec<Role> {
        match self {
            Role::Admin => vec![
                Role::Moderator,
                Role::Editor,
                Role::Creator,
                Role::Viewer,
                Role::Anonymous,
            ],
            Role::Moderator => vec![Role::Editor, Role::Viewer, Role::Anonymous],
            Role::Editor => vec![Role::Viewer, Role::Anonymous],
            Role::Creator => vec![Role::Viewer, Role::Anonymous],
            Role::Viewer => vec![Role::Anonymous],
            Role::Anonymous => vec![],
        }
    }

    pub fn expand_roles(roles: &[Role]) -> Vec<Role> {
        let mut expanded_roles = vec![];
        for role in roles {
            for child_role in role.child_roles() {
                if expanded_roles.contains(&child_role) {
                    continue;
                }
                if roles.contains(&child_role) {
                    continue;
                }
                expanded_roles.push(child_role);
            }
            expanded_roles.push(*role);
        }

        expanded_roles
    }
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
