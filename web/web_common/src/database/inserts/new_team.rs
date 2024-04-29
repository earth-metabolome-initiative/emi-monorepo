use crate::{custom_validators::*, database::TeamState};
use crate::database::{Insert, NestedTeam};
use serde::{Deserialize, Serialize};
use validator::Validate;

pub type NewTeamName = NoSpecialCharacters<MustBeCapitalized<NotEmpty>>;

#[derive(Debug, Clone, PartialEq, Eq, Validate, Serialize, Deserialize)]
pub struct NewTeam {
    #[validate]
    pub name: NewTeamName,
    #[validate]
    pub description: NotEmpty,
    pub team_state: TeamState,
    pub parent_team: Option<NestedTeam>,
}

impl NewTeam {
    pub fn new(
        name: String,
        description: String,
        team_state: TeamState,
        parent_team: Option<NestedTeam>,
    ) -> Result<Self, Vec<String>> {
        let new_team = Self {
            name: NewTeamName::try_from(name)?,
            description: NotEmpty::try_from(description)?,
            team_state,
            parent_team,
        };

        Ok(new_team)
    }
}

impl From<NewTeam> for Insert {
    fn from(team: NewTeam) -> Self {
        Insert::Team(team)
    }
}

impl From<NewTeam> for crate::database::Operation {
    fn from(team: NewTeam) -> Self {
        Insert::from(team).into()
    }
}

impl From<NewTeam> for crate::database::Task {
    fn from(team: NewTeam) -> Self {
        crate::database::Operation::from(team).into()
    }
}

impl From<NewTeam> for crate::api::ws::messages::FrontendMessage {
    fn from(team: NewTeam) -> Self {
        crate::api::ws::messages::FrontendMessage::Task(team.into())
    }
}
