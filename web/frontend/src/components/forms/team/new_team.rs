//! Component for the form requiring user name and surname.

use crate::components::forms::*;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, rc::Rc};
use web_common::api::form_traits::FormMethod;
use web_common::custom_validators::NotEmpty;
use web_common::database::inserts::new_team::NewTeamName;
use web_common::database::inserts::NewTeam;
use web_common::database::{NestedTeam, TeamState};
use yew::prelude::*;
use yewdux::{use_store, Reducer, Store};

#[derive(Debug, PartialEq, Clone, Default, Store, Serialize, Deserialize)]
#[store(storage = "session")]
pub struct NewTeamBuilder {
    pub name: Option<NewTeamName>,
    pub description: Option<NotEmpty>,
    pub parent_team: Option<NestedTeam>,
    pub team_state: Option<TeamState>,
}

impl FormBuilder for NewTeamBuilder {
    type Data = NewTeam;
    type Actions = NewTeamBuilderActions;

    fn buildable(&self) -> Result<(), web_common::api::ApiError> {
        // self.name.validate()?;
        if self.name.is_none() {
            return Err(web_common::api::ApiError::BadRequest(vec![
                "Name is required.".to_string(),
            ]));
        }
        if self.description.is_none() {
            return Err(web_common::api::ApiError::BadRequest(vec![
                "Description is required.".to_string(),
            ]));
        }
        //self.description.validate()?;
        if self.team_state.is_none() {
            return Err(web_common::api::ApiError::BadRequest(vec![
                "Team state is required.".to_string(),
            ]));
        }
        Ok(())
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }

    fn build(&self) -> Self::Data {
        NewTeam::new(
            self.name.clone().unwrap().to_string(),
            self.description.clone().unwrap().to_string(),
            self.team_state.clone().unwrap(),
            self.parent_team.clone(),
        ).unwrap()
    }
}

pub enum NewTeamBuilderActions {
    SetName(NewTeamName),
    SetDescription(NotEmpty),
    SetParentTeam(Option<NestedTeam>),
    SetTeamState(Option<TeamState>),
}

impl Reducer<NewTeamBuilder> for NewTeamBuilderActions {
    fn apply(self, mut state: std::rc::Rc<NewTeamBuilder>) -> std::rc::Rc<NewTeamBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NewTeamBuilderActions::SetName(name) => {
                state_mut.name = Some(name);
            }
            NewTeamBuilderActions::SetDescription(description) => {
                state_mut.description = Some(description);
            }
            NewTeamBuilderActions::SetParentTeam(parent_team) => {
                state_mut.parent_team = parent_team;
            }
            NewTeamBuilderActions::SetTeamState(team_state) => {
                state_mut.team_state = team_state;
            }
        }
        state
    }
}

impl FormBuildable for NewTeam {
    type Builder = NewTeamBuilder;

    const METHOD: FormMethod = FormMethod::POST;

    fn title() -> &'static str {
        "New Team"
    }

    fn task_target() -> &'static str {
        "Team"
    }

    fn description() -> &'static str {
        concat!("Create a new team.\n",)
    }

    fn requires_authentication() -> bool {
        true
    }
}

#[function_component(NewTeamForm)]
pub fn complete_profile_form() -> Html {
    // The use_reducer hook takes an initialization function which will be called only once.
    let (store, dispatch) = use_store::<NewTeamBuilder>();

    let set_name = dispatch.apply_callback(|name| NewTeamBuilderActions::SetName(name));
    let set_description = dispatch
        .apply_callback(|description| NewTeamBuilderActions::SetDescription(description));
    let set_parent_team = dispatch.apply_callback(|mut teams: Vec<NestedTeam>| {
        NewTeamBuilderActions::SetParentTeam(teams.pop())
    });
    let set_team_state = dispatch.apply_callback(|mut team_states: Vec<TeamState>| {
        NewTeamBuilderActions::SetTeamState(team_states.pop())
    });

    html! {
        <BasicForm<NewTeam> builder={store.deref().clone()}>
            <BasicInput<NewTeamName> label="Name" builder={set_name} value={store.name.clone()} input_type={InputType::Text} />
            <BasicInput<NotEmpty> label="Description" builder={set_description} value={store.description.clone()} input_type={InputType::Textarea} />
            // <Datalist<Editable<web_common::database::NestedTeam>> builder={set_parent_team} value={store.parent_team.clone().map_or_else(|| Vec::new(), |value| vec![value])} label="Team" />
            <Datalist<web_common::database::TeamState> builder={set_team_state} value={store.team_state.clone().map_or_else(|| Vec::new(), |value| vec![value])} label="Team State" />
        </BasicForm<NewTeam>>
    }
}
