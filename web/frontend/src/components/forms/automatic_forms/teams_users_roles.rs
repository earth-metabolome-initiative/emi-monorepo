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
#[derive(Store, Eq, PartialEq, PartialOrd, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TeamsUsersRoleBuilder {
    pub table: Option<Rc<NestedTeam>>,
    pub user: Option<Rc<NestedUser>>,
    pub role: Option<Rc<NestedRole>>,
    pub errors_table: Vec<ApiError>,
    pub errors_user: Vec<ApiError>,
    pub errors_role: Vec<ApiError>,
    pub form_updated_at: chrono::NaiveDateTime,
}

unsafe impl Send for TeamsUsersRoleBuilder {}
unsafe impl Sync for TeamsUsersRoleBuilder {}
impl Default for TeamsUsersRoleBuilder {
    fn default() -> Self {
        Self {
            table: Default::default(),
            user: Default::default(),
            role: Default::default(),
            errors_table: Default::default(),
            errors_user: Default::default(),
            errors_role: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) enum TeamsUsersRoleActions {
    SetTable(Option<Rc<NestedTeam>>),
    SetUser(Option<Rc<NestedUser>>),
    SetRole(Option<Rc<NestedRole>>),
}

impl FromOperation for TeamsUsersRoleActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "table" => TeamsUsersRoleActions::SetTable(Some(bincode::deserialize(&row).unwrap())),
            "user" => TeamsUsersRoleActions::SetUser(Some(bincode::deserialize(&row).unwrap())),
            "role" => TeamsUsersRoleActions::SetRole(Some(bincode::deserialize(&row).unwrap())),
            operation_name => {
                unreachable!("The operation name '{}' is not supported.", operation_name)
            }
        }
    }
}

impl Reducer<TeamsUsersRoleBuilder> for TeamsUsersRoleActions {
    fn apply(
        self,
        mut state: std::rc::Rc<TeamsUsersRoleBuilder>,
    ) -> std::rc::Rc<TeamsUsersRoleBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            TeamsUsersRoleActions::SetTable(table) => 'table: {
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
            TeamsUsersRoleActions::SetUser(user) => 'user: {
                state_mut.errors_user.clear();
                if user.is_none() {
                    state_mut.errors_user.push(ApiError::BadRequest(vec![
                        "The User field is required.".to_string(),
                    ]));
                    state_mut.user = None;
                    break 'user;
                }
                state_mut.user = user.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'user;
            }
            TeamsUsersRoleActions::SetRole(role) => 'role: {
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
impl FormBuilder for TeamsUsersRoleBuilder {
    type Actions = TeamsUsersRoleActions;

    type RichVariant = NestedTeamsUsersRole;

    fn has_errors(&self) -> bool {
        !self.errors_table.is_empty()
            || !self.errors_user.is_empty()
            || !self.errors_role.is_empty()
    }

    fn update(
        dispatcher: &Dispatch<Self>,
        richest_variant: Self::RichVariant,
    ) -> Vec<ComponentMessage> {
        dispatcher.apply(TeamsUsersRoleActions::SetTable(
            Some(richest_variant.table).map(Rc::from),
        ));
        dispatcher.apply(TeamsUsersRoleActions::SetUser(
            Some(richest_variant.user).map(Rc::from),
        ));
        dispatcher.apply(TeamsUsersRoleActions::SetRole(
            Some(richest_variant.role).map(Rc::from),
        ));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors() && self.table.is_some() && self.user.is_some() && self.role.is_some()
    }
}

impl From<TeamsUsersRoleBuilder> for NewTeamsUsersRole {
    fn from(builder: TeamsUsersRoleBuilder) -> Self {
        Self {
            table_id: builder.table.as_ref().map(|table| table.inner.id).unwrap(),
            user_id: builder.user.as_ref().map(|user| user.inner.id).unwrap(),
            role_id: builder.role.as_deref().cloned().unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewTeamsUsersRole {
    type Builder = TeamsUsersRoleBuilder;
    fn title() -> &'static str {
        "Teams users role"
    }
    fn task_target() -> &'static str {
        "Teams users role"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateTeamsUsersRoleFormProp {
    #[prop_or_default]
    pub table_id: Option<i32>,
    #[prop_or_default]
    pub user_id: Option<i32>,
    #[prop_or_default]
    pub role_id: Option<i32>,
}

#[function_component(CreateTeamsUsersRoleForm)]
pub fn create_teams_users_role_form(props: &CreateTeamsUsersRoleFormProp) -> Html {
    let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = yewdux::use_store::<TeamsUsersRoleBuilder>();
    if let Some(table_id) = props.table_id {
        named_requests.push(ComponentMessage::get_named::<&str, Team>(
            "table",
            table_id.into(),
        ));
    }
    if let Some(user_id) = props.user_id {
        named_requests.push(ComponentMessage::get_named::<&str, User>(
            "user",
            user_id.into(),
        ));
    }
    if let Some(role_id) = props.role_id {
        named_requests.push(ComponentMessage::get_named::<&str, Role>(
            "role",
            role_id.into(),
        ));
    }
    let set_table = builder_dispatch
        .apply_callback(|table: Option<Rc<NestedTeam>>| TeamsUsersRoleActions::SetTable(table));
    let set_user = builder_dispatch
        .apply_callback(|user: Option<Rc<NestedUser>>| TeamsUsersRoleActions::SetUser(user));
    let set_role = builder_dispatch
        .apply_callback(|role: Option<Rc<NestedRole>>| TeamsUsersRoleActions::SetRole(role));
    html! {
        <BasicForm<NewTeamsUsersRole>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Datalist<NestedTeam, true> builder={set_table} optional={false} errors={builder_store.errors_table.clone()} value={builder_store.table.clone()} label="Table" scanner={false} />
            <Datalist<NestedUser, false> builder={set_user} optional={false} errors={builder_store.errors_user.clone()} value={builder_store.user.clone()} label="User" scanner={false} />
            <Datalist<NestedRole, false> builder={set_role} optional={false} errors={builder_store.errors_role.clone()} value={builder_store.role.clone()} label="Role" scanner={false} />
        </BasicForm<NewTeamsUsersRole>>
    }
}
