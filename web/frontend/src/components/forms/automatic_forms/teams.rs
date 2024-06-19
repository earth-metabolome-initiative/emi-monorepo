//! This module contains the forms for the frontend.
//!
//! This module is automatically generated. Do not write anything here.

use crate::components::forms::*;
use crate::workers::ws_worker::ComponentMessage;
use std::ops::Deref;
use std::rc::Rc;
use web_common::api::form_traits::FormMethod;
use web_common::api::ApiError;
use web_common::database::*;
use yew::prelude::*;
use yewdux::Dispatch;
use yewdux::{Reducer, Store};
#[derive(Store, PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct TeamBuilder {
    pub id: Option<i32>,
    pub name: Option<Rc<String>>,
    pub description: Option<Rc<String>>,
    pub icon: Option<Rc<web_common::database::flat_variants::FontAwesomeIcon>>,
    pub color: Option<Rc<web_common::database::flat_variants::Color>>,
    pub state: Option<Rc<web_common::database::nested_variants::NestedTeamState>>,
    pub parent_team: Option<Rc<web_common::database::nested_variants::NestedTeam>>,
    pub errors_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
    pub errors_icon: Vec<ApiError>,
    pub errors_color: Vec<ApiError>,
    pub errors_state: Vec<ApiError>,
    pub errors_parent_team: Vec<ApiError>,
    pub form_updated_at: chrono::NaiveDateTime,
}

unsafe impl Send for TeamBuilder {}
unsafe impl Sync for TeamBuilder {}
impl Default for TeamBuilder {
    fn default() -> Self {
        Self {
            id: None,
            name: None,
            description: None,
            icon: Default::default(),
            color: Default::default(),
            state: Default::default(),
            parent_team: Default::default(),
            errors_name: Default::default(),
            errors_description: Default::default(),
            errors_icon: Default::default(),
            errors_color: Default::default(),
            errors_state: Default::default(),
            errors_parent_team: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) enum TeamActions {
    SetName(Option<String>),
    SetDescription(Option<String>),
    SetIcon(Option<Rc<web_common::database::flat_variants::FontAwesomeIcon>>),
    SetColor(Option<Rc<web_common::database::flat_variants::Color>>),
    SetState(Option<Rc<web_common::database::nested_variants::NestedTeamState>>),
    SetParentTeam(Option<Rc<web_common::database::nested_variants::NestedTeam>>),
}

impl FromOperation for TeamActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "icon" => TeamActions::SetIcon(Some(bincode::deserialize(&row).unwrap())),
            "color" => TeamActions::SetColor(Some(bincode::deserialize(&row).unwrap())),
            "state" => TeamActions::SetState(Some(bincode::deserialize(&row).unwrap())),
            "parent_team" => TeamActions::SetParentTeam(Some(bincode::deserialize(&row).unwrap())),
            operation_name => {
                unreachable!("The operation name '{}' is not supported.", operation_name)
            }
        }
    }
}

impl Reducer<TeamBuilder> for TeamActions {
    fn apply(self, mut state: std::rc::Rc<TeamBuilder>) -> std::rc::Rc<TeamBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            TeamActions::SetName(name) => 'name: {
                state_mut.errors_name.clear();
                if name.is_none() {
                    state_mut.errors_name.push(ApiError::BadRequest(vec![
                        "The Name field is required.".to_string(),
                    ]));
                    state_mut.name = None;
                    break 'name;
                }
                if let Some(value) = name.as_ref() {
                    if value.is_empty() {
                        state_mut
                            .errors_name
                            .push(ApiError::Empty("Name".to_string()));
                        state_mut.name = None;
                        break 'name;
                    }
                }
                state_mut.name = name.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'name;
            }
            TeamActions::SetDescription(description) => 'description: {
                state_mut.errors_description.clear();
                if description.is_none() {
                    state_mut.errors_description.push(ApiError::BadRequest(vec![
                        "The Description field is required.".to_string(),
                    ]));
                    state_mut.description = None;
                    break 'description;
                }
                if let Some(value) = description.as_ref() {
                    if value.is_empty() {
                        state_mut
                            .errors_description
                            .push(ApiError::Empty("Description".to_string()));
                        state_mut.description = None;
                        break 'description;
                    }
                }
                state_mut.description = description.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'description;
            }
            TeamActions::SetIcon(icon) => 'icon: {
                state_mut.errors_icon.clear();
                if icon.is_none() {
                    state_mut.errors_icon.push(ApiError::BadRequest(vec![
                        "The Icon field is required.".to_string(),
                    ]));
                    state_mut.icon = None;
                    break 'icon;
                }
                state_mut.icon = icon.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'icon;
            }
            TeamActions::SetColor(color) => 'color: {
                state_mut.errors_color.clear();
                if color.is_none() {
                    state_mut.errors_color.push(ApiError::BadRequest(vec![
                        "The Color field is required.".to_string(),
                    ]));
                    state_mut.color = None;
                    break 'color;
                }
                state_mut.color = color.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'color;
            }
            TeamActions::SetState(state) => 'state: {
                state_mut.errors_state.clear();
                if state.is_none() {
                    state_mut.errors_state.push(ApiError::BadRequest(vec![
                        "The State field is required.".to_string(),
                    ]));
                    state_mut.state = None;
                    break 'state;
                }
                state_mut.state = state.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'state;
            }
            TeamActions::SetParentTeam(parent_team) => 'parent_team: {
                state_mut.errors_parent_team.clear();
                match parent_team.as_ref() {
                    Some(parent_team) => {
                        if state_mut.id.map_or(false, |id| id == parent_team.inner.id) {
                            state_mut.errors_parent_team.push(ApiError::BadRequest(vec![
                                "The Parent team field must be distinct from the current value."
                                    .to_string(),
                            ]));
                            break 'parent_team;
                        }
                    }
                    None => (),
                }
                state_mut.parent_team = parent_team.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'parent_team;
            }
        }
        state
    }
}
impl FormBuilder for TeamBuilder {
    type Actions = TeamActions;

    type RichVariant = NestedTeam;

    fn has_errors(&self) -> bool {
        !self.errors_name.is_empty()
            || !self.errors_description.is_empty()
            || !self.errors_icon.is_empty()
            || !self.errors_color.is_empty()
            || !self.errors_state.is_empty()
            || !self.errors_parent_team.is_empty()
    }

    fn update(
        dispatcher: &Dispatch<Self>,
        richest_variant: Self::RichVariant,
    ) -> Vec<ComponentMessage> {
        dispatcher.reduce_mut(|state| {
            state.id = Some(richest_variant.inner.id);
        });
        dispatcher.apply(TeamActions::SetName(Some(
            richest_variant.inner.name.to_string(),
        )));
        dispatcher.apply(TeamActions::SetDescription(Some(
            richest_variant.inner.description.to_string(),
        )));
        dispatcher.apply(TeamActions::SetIcon(
            Some(richest_variant.icon).map(Rc::from),
        ));
        dispatcher.apply(TeamActions::SetColor(
            Some(richest_variant.color).map(Rc::from),
        ));
        dispatcher.apply(TeamActions::SetState(
            Some(richest_variant.state).map(Rc::from),
        ));
        let mut named_requests = Vec::new();
        if let Some(parent_team_id) = richest_variant.inner.parent_team_id {
            named_requests.push(ComponentMessage::get_named::<&str, Team>(
                "parent_team",
                parent_team_id.into(),
            ));
        } else {
            dispatcher.apply(TeamActions::SetParentTeam(None));
        }
        named_requests
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
            && self.name.is_some()
            && self.description.is_some()
            && self.icon.is_some()
            && self.color.is_some()
            && self.state.is_some()
    }
}

impl From<TeamBuilder> for NewTeam {
    fn from(builder: TeamBuilder) -> Self {
        Self {
            name: builder.name.as_deref().cloned().unwrap(),
            description: builder.description.as_deref().cloned().unwrap(),
            icon_id: builder.icon.as_deref().cloned().unwrap().id,
            color_id: builder.color.as_deref().cloned().unwrap().id,
            state_id: builder.state.as_deref().cloned().unwrap().inner.id,
            parent_team_id: builder
                .parent_team
                .as_deref()
                .cloned()
                .map(|parent_team| parent_team.inner.id),
        }
    }
}
impl From<TeamBuilder> for UpdateTeam {
    fn from(builder: TeamBuilder) -> Self {
        Self {
            id: builder.id.unwrap(),
            name: builder.name.as_deref().cloned().unwrap(),
            description: builder.description.as_deref().cloned().unwrap(),
            icon_id: builder.icon.as_deref().cloned().unwrap().id,
            color_id: builder.color.as_deref().cloned().unwrap().id,
            state_id: builder.state.as_deref().cloned().unwrap().inner.id,
            parent_team_id: builder
                .parent_team
                .as_deref()
                .cloned()
                .map(|parent_team| parent_team.inner.id),
        }
    }
}
impl FormBuildable for NewTeam {
    type Builder = TeamBuilder;
    fn title() -> &'static str {
        "Team"
    }
    fn task_target() -> &'static str {
        "Team"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

impl FormBuildable for UpdateTeam {
    type Builder = TeamBuilder;
    fn title() -> &'static str {
        "Team"
    }
    fn task_target() -> &'static str {
        "Team"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateTeamFormProp {
    #[prop_or(1387)]
    pub icon_id: i32,
    #[prop_or(15)]
    pub color_id: i32,
    #[prop_or(1)]
    pub state_id: i32,
    #[prop_or_default]
    pub parent_team_id: Option<i32>,
}

#[function_component(CreateTeamForm)]
pub fn create_team_form(props: &CreateTeamFormProp) -> Html {
    let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = yewdux::use_store::<TeamBuilder>();
    named_requests.push(ComponentMessage::get_named::<&str, FontAwesomeIcon>(
        "icon",
        props.icon_id.into(),
    ));
    named_requests.push(ComponentMessage::get_named::<&str, Color>(
        "color",
        props.color_id.into(),
    ));
    named_requests.push(ComponentMessage::get_named::<&str, TeamState>(
        "state",
        props.state_id.into(),
    ));
    if let Some(parent_team_id) = props.parent_team_id {
        named_requests.push(ComponentMessage::get_named::<&str, Team>(
            "parent_team",
            parent_team_id.into(),
        ));
    }
    let set_name =
        builder_dispatch.apply_callback(|name: Option<String>| TeamActions::SetName(name));
    let set_description = builder_dispatch
        .apply_callback(|description: Option<String>| TeamActions::SetDescription(description));
    let set_icon = builder_dispatch.apply_callback(
        |icon: Option<Rc<web_common::database::flat_variants::FontAwesomeIcon>>| {
            TeamActions::SetIcon(icon)
        },
    );
    let set_color = builder_dispatch.apply_callback(
        |color: Option<Rc<web_common::database::flat_variants::Color>>| {
            TeamActions::SetColor(color)
        },
    );
    let set_state = builder_dispatch.apply_callback(
        |state: Option<Rc<web_common::database::nested_variants::NestedTeamState>>| {
            TeamActions::SetState(state)
        },
    );
    let set_parent_team = builder_dispatch.apply_callback(
        |parent_team: Option<Rc<web_common::database::nested_variants::NestedTeam>>| {
            TeamActions::SetParentTeam(parent_team)
        },
    );
    html! {
        <BasicForm<NewTeam>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Name" optional={false} errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<String> label="Description" optional={false} errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
            <Datalist<web_common::database::flat_variants::FontAwesomeIcon, false> builder={set_icon} optional={false} errors={builder_store.errors_icon.clone()} value={builder_store.icon.clone()} label="Icon" scanner={false} />
            <Datalist<web_common::database::flat_variants::Color, false> builder={set_color} optional={false} errors={builder_store.errors_color.clone()} value={builder_store.color.clone()} label="Color" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedTeamState, false> builder={set_state} optional={false} errors={builder_store.errors_state.clone()} value={builder_store.state.clone()} label="State" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedTeam, true> builder={set_parent_team} optional={true} errors={builder_store.errors_parent_team.clone()} value={builder_store.parent_team.clone()} label="Parent team" scanner={false} />
        </BasicForm<NewTeam>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateTeamFormProp {
    pub id: i32,
}

#[function_component(UpdateTeamForm)]
pub fn update_team_form(props: &UpdateTeamFormProp) -> Html {
    let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = yewdux::use_store::<TeamBuilder>();
    // We push the ID of the row to the named requests.
    let props = props.clone();
    named_requests.push(ComponentMessage::get::<UpdateTeam>(props.id.into()));
    let set_name =
        builder_dispatch.apply_callback(|name: Option<String>| TeamActions::SetName(name));
    let set_description = builder_dispatch
        .apply_callback(|description: Option<String>| TeamActions::SetDescription(description));
    let set_icon = builder_dispatch.apply_callback(
        |icon: Option<Rc<web_common::database::flat_variants::FontAwesomeIcon>>| {
            TeamActions::SetIcon(icon)
        },
    );
    let set_color = builder_dispatch.apply_callback(
        |color: Option<Rc<web_common::database::flat_variants::Color>>| {
            TeamActions::SetColor(color)
        },
    );
    let set_state = builder_dispatch.apply_callback(
        |state: Option<Rc<web_common::database::nested_variants::NestedTeamState>>| {
            TeamActions::SetState(state)
        },
    );
    let set_parent_team = builder_dispatch.apply_callback(
        |parent_team: Option<Rc<web_common::database::nested_variants::NestedTeam>>| {
            TeamActions::SetParentTeam(parent_team)
        },
    );
    html! {
        <BasicForm<UpdateTeam>
            method={FormMethod::PUT}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Name" optional={false} errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<String> label="Description" optional={false} errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
            <Datalist<web_common::database::flat_variants::FontAwesomeIcon, false> builder={set_icon} optional={false} errors={builder_store.errors_icon.clone()} value={builder_store.icon.clone()} label="Icon" scanner={false} />
            <Datalist<web_common::database::flat_variants::Color, false> builder={set_color} optional={false} errors={builder_store.errors_color.clone()} value={builder_store.color.clone()} label="Color" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedTeamState, false> builder={set_state} optional={false} errors={builder_store.errors_state.clone()} value={builder_store.state.clone()} label="State" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedTeam, true> builder={set_parent_team} optional={true} errors={builder_store.errors_parent_team.clone()} value={builder_store.parent_team.clone()} label="Parent team" scanner={false} />
        </BasicForm<UpdateTeam>>
    }
}
