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
pub struct ProjectsTeamsRoleInvitationBuilder {
    pub table: Option<Rc<web_common::database::nested_variants::NestedProject>>,
    pub team: Option<Rc<web_common::database::nested_variants::NestedTeam>>,
    pub role: Option<Rc<web_common::database::nested_variants::NestedRole>>,
    pub errors_table: Vec<ApiError>,
    pub errors_team: Vec<ApiError>,
    pub errors_role: Vec<ApiError>,
    pub form_updated_at: chrono::NaiveDateTime,
}

unsafe impl Send for ProjectsTeamsRoleInvitationBuilder {}
unsafe impl Sync for ProjectsTeamsRoleInvitationBuilder {}
impl Default for ProjectsTeamsRoleInvitationBuilder {
    fn default() -> Self {
        Self {
            table: Default::default(),
            team: Default::default(),
            role: Default::default(),
            errors_table: Default::default(),
            errors_team: Default::default(),
            errors_role: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) enum ProjectsTeamsRoleInvitationActions {
    SetTable(Option<Rc<web_common::database::nested_variants::NestedProject>>),
    SetTeam(Option<Rc<web_common::database::nested_variants::NestedTeam>>),
    SetRole(Option<Rc<web_common::database::nested_variants::NestedRole>>),
}

impl FromOperation for ProjectsTeamsRoleInvitationActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "table" => ProjectsTeamsRoleInvitationActions::SetTable(Some(
                bincode::deserialize(&row).unwrap(),
            )),
            "team" => ProjectsTeamsRoleInvitationActions::SetTeam(Some(
                bincode::deserialize(&row).unwrap(),
            )),
            "role" => ProjectsTeamsRoleInvitationActions::SetRole(Some(
                bincode::deserialize(&row).unwrap(),
            )),
            operation_name => {
                unreachable!("The operation name '{}' is not supported.", operation_name)
            }
        }
    }
}

impl Reducer<ProjectsTeamsRoleInvitationBuilder> for ProjectsTeamsRoleInvitationActions {
    fn apply(
        self,
        mut state: std::rc::Rc<ProjectsTeamsRoleInvitationBuilder>,
    ) -> std::rc::Rc<ProjectsTeamsRoleInvitationBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            ProjectsTeamsRoleInvitationActions::SetTable(table) => 'table: {
                state_mut.errors_table.clear();
                if table.is_none() {
                    state_mut.errors_table.push(ApiError::BadRequest(vec![
                        "The Table field is required.".to_string(),
                    ]));
                    state_mut.table = None;
                    break 'table;
                }
                state_mut.table = table.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'table;
            }
            ProjectsTeamsRoleInvitationActions::SetTeam(team) => 'team: {
                state_mut.errors_team.clear();
                if team.is_none() {
                    state_mut.errors_team.push(ApiError::BadRequest(vec![
                        "The Team field is required.".to_string(),
                    ]));
                    state_mut.team = None;
                    break 'team;
                }
                state_mut.team = team.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'team;
            }
            ProjectsTeamsRoleInvitationActions::SetRole(role) => 'role: {
                state_mut.errors_role.clear();
                if role.is_none() {
                    state_mut.errors_role.push(ApiError::BadRequest(vec![
                        "The Role field is required.".to_string(),
                    ]));
                    state_mut.role = None;
                    break 'role;
                }
                state_mut.role = role.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'role;
            }
        }
        state
    }
}
impl FormBuilder for ProjectsTeamsRoleInvitationBuilder {
    type Actions = ProjectsTeamsRoleInvitationActions;

    type RichVariant = NestedProjectsTeamsRoleInvitation;

    fn has_errors(&self) -> bool {
        !self.errors_table.is_empty()
            || !self.errors_team.is_empty()
            || !self.errors_role.is_empty()
    }

    fn update(
        dispatcher: &Dispatch<Self>,
        richest_variant: Self::RichVariant,
    ) -> Vec<ComponentMessage> {
        dispatcher.apply(ProjectsTeamsRoleInvitationActions::SetTable(
            Some(richest_variant.table).map(Rc::from),
        ));
        dispatcher.apply(ProjectsTeamsRoleInvitationActions::SetTeam(
            Some(richest_variant.team).map(Rc::from),
        ));
        dispatcher.apply(ProjectsTeamsRoleInvitationActions::SetRole(
            Some(richest_variant.role).map(Rc::from),
        ));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors() && self.table.is_some() && self.team.is_some() && self.role.is_some()
    }
}

impl From<ProjectsTeamsRoleInvitationBuilder> for NewProjectsTeamsRoleInvitation {
    fn from(builder: ProjectsTeamsRoleInvitationBuilder) -> Self {
        Self {
            table_id: builder.table.as_ref().map(|table| table.inner.id).unwrap(),
            team_id: builder.team.as_ref().map(|team| team.inner.id).unwrap(),
            role_id: builder.role.as_deref().cloned().unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewProjectsTeamsRoleInvitation {
    type Builder = ProjectsTeamsRoleInvitationBuilder;
    fn title() -> &'static str {
        "Projects teams role invitation"
    }
    fn task_target() -> &'static str {
        "Projects teams role invitation"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateProjectsTeamsRoleInvitationFormProp {
    #[prop_or_default]
    pub table_id: Option<i32>,
    #[prop_or_default]
    pub team_id: Option<i32>,
    #[prop_or_default]
    pub role_id: Option<i32>,
}

#[function_component(CreateProjectsTeamsRoleInvitationForm)]
pub fn create_projects_teams_role_invitation_form(
    props: &CreateProjectsTeamsRoleInvitationFormProp,
) -> Html {
    let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) =
        yewdux::use_store::<ProjectsTeamsRoleInvitationBuilder>();
    if let Some(table_id) = props.table_id {
        named_requests.push(ComponentMessage::get_named::<&str, Project>(
            "table",
            table_id.into(),
        ));
    }
    if let Some(team_id) = props.team_id {
        named_requests.push(ComponentMessage::get_named::<&str, Team>(
            "team",
            team_id.into(),
        ));
    }
    if let Some(role_id) = props.role_id {
        named_requests.push(ComponentMessage::get_named::<&str, Role>(
            "role",
            role_id.into(),
        ));
    }
    let set_table = builder_dispatch.apply_callback(
        |table: Option<Rc<web_common::database::nested_variants::NestedProject>>| {
            ProjectsTeamsRoleInvitationActions::SetTable(table)
        },
    );
    let set_team = builder_dispatch.apply_callback(
        |team: Option<Rc<web_common::database::nested_variants::NestedTeam>>| {
            ProjectsTeamsRoleInvitationActions::SetTeam(team)
        },
    );
    let set_role = builder_dispatch.apply_callback(
        |role: Option<Rc<web_common::database::nested_variants::NestedRole>>| {
            ProjectsTeamsRoleInvitationActions::SetRole(role)
        },
    );
    html! {
        <BasicForm<NewProjectsTeamsRoleInvitation>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Datalist<web_common::database::nested_variants::NestedProject, true> builder={set_table} optional={false} errors={builder_store.errors_table.clone()} value={builder_store.table.clone()} label="Table" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedTeam, true> builder={set_team} optional={false} errors={builder_store.errors_team.clone()} value={builder_store.team.clone()} label="Team" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedRole, false> builder={set_role} optional={false} errors={builder_store.errors_role.clone()} value={builder_store.role.clone()} label="Role" scanner={false} />
        </BasicForm<NewProjectsTeamsRoleInvitation>>
    }
}
