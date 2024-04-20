use crate::{custom_validators::*, database::ProjectState};
use crate::database::Insert;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub type NewProjectName = NoSpecialCharacters<MustBeCapitalized<NotEmpty>>;

#[derive(Debug, Clone, PartialEq, Eq, Validate, Serialize, Deserialize)]
pub struct NewProject {
    #[validate]
    pub name: NewProjectName,
    #[validate]
    pub description: NotEmpty,
    pub public: bool,
    pub project_state: ProjectState
    // pub parent_project_id: Option<Uuid>,
}

impl NewProject {
    pub fn new(
        name: String,
        description: String,
        public: bool,
        project_state: ProjectState
    ) -> Result<Self, Vec<String>> {
        let new_project = Self {
            name: NewProjectName::try_from(name)?,
            description: NotEmpty::try_from(description)?,
            public,
            project_state
        };

        Ok(new_project)
    }
}

impl From<NewProject> for Insert {
    fn from(project: NewProject) -> Self {
        Insert::Project(project)
    }
}

impl From<NewProject> for crate::database::Operation {
    fn from(project: NewProject) -> Self {
        Insert::from(project).into()
    }
}

impl From<NewProject> for crate::database::Task {
    fn from(project: NewProject) -> Self {
        crate::database::Operation::from(project).into()
    }
}

impl From<NewProject> for crate::api::ws::messages::FrontendMessage {
    fn from(project: NewProject) -> Self {
        crate::api::ws::messages::FrontendMessage::Task(project.into())
    }
}
