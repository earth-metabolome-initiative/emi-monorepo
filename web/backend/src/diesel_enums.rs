#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WebsiteRole {
    Admin,
    Editor,
}

impl std::fmt::Display for WebsiteRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebsiteRole::Admin => write!(f, "admin"),
            WebsiteRole::Editor => write!(f, "editor"),
        }
    }
}

impl From<&str> for WebsiteRole {
    fn from(s: &str) -> Self {
        match s {
            "admin" => WebsiteRole::Admin,
            "editor" => WebsiteRole::Editor,
            _ => unimplemented!("Unknown website role: {}", s),
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProjectUserRole {
    Admin,
    Viewer,
    Editor,
}

impl std::fmt::Display for ProjectUserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectUserRole::Admin => write!(f, "admin"),
            ProjectUserRole::Editor => write!(f, "editor"),
            ProjectUserRole::Viewer => write!(f, "viewer"),
        }
    }
}

impl From<&str> for ProjectUserRole {
    fn from(s: &str) -> Self {
        match s {
            "admin" => ProjectUserRole::Admin,
            "editor" => ProjectUserRole::Editor,
            "viewer" => ProjectUserRole::Viewer,
            _ => unimplemented!("Unknown website role: {}", s),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OrganizationUserRole {
    Admin,
}

impl std::fmt::Display for OrganizationUserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrganizationUserRole::Admin => write!(f, "admin"),
        }
    }
}

impl From<&str> for OrganizationUserRole {
    fn from(s: &str) -> Self {
        match s {
            "admin" => OrganizationUserRole::Admin,
            _ => unimplemented!("Unknown website role: {}", s),
        }
    }
}
