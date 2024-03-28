use crate::api::form_traits::FormResult;
use crate::custom_validators::*;
use crate::database::Insert;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

pub type NewProjectName = NoSpecialCharacters<MustBeCapitalized<NotEmpty>>;

#[derive(Debug, Clone, PartialEq, Eq, Validate, Serialize, Deserialize)]
pub struct NewProject {
    #[validate]
    pub name: NewProjectName,
    #[validate]
    pub description: NotEmpty,
    pub public: bool,
    // pub state_id: Uuid,
    // pub parent_project_id: Option<Uuid>,
}

impl NewProject {
    pub fn new(name: String, description: String, public: bool) -> Result<Self, Vec<String>> {
        let new_project = Self {
            name: NewProjectName::try_from(name)?,
            description: NotEmpty::try_from(description)?,
            public,
        };

        Ok(new_project)
    }
}

impl FormResult for NewProject {
    const METHOD: crate::api::form_traits::FormMethod = crate::api::form_traits::FormMethod::POST;

    fn title() -> &'static str {
        "New Project"
    }

    fn task_target() -> &'static str {
        "Project"
    }

    fn description() -> &'static str {
        concat!("Crate a new project.\n",)
    }

    fn requires_authentication() -> bool {
        true
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
