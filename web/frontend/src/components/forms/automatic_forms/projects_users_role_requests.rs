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
pub struct ProjectsUsersRoleRequestBuilder {
    pub table: Option<Rc<web_common::database::nested_variants::NestedProject>>,
    pub user: Option<Rc<web_common::database::nested_variants::NestedUser>>,
    pub role: Option<Rc<web_common::database::nested_variants::NestedRole>>,
    pub errors_table: Vec<ApiError>,
    pub errors_user: Vec<ApiError>,
    pub errors_role: Vec<ApiError>,
    pub form_updated_at: chrono::NaiveDateTime,
}

unsafe impl Send for ProjectsUsersRoleRequestBuilder {}
unsafe impl Sync for ProjectsUsersRoleRequestBuilder {}
impl Default for ProjectsUsersRoleRequestBuilder {
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

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) enum ProjectsUsersRoleRequestActions {
    SetTable(Option<Rc<web_common::database::nested_variants::NestedProject>>),
    SetUser(Option<Rc<web_common::database::nested_variants::NestedUser>>),
    SetRole(Option<Rc<web_common::database::nested_variants::NestedRole>>),
}

impl FromOperation for ProjectsUsersRoleRequestActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "table" => {
                ProjectsUsersRoleRequestActions::SetTable(Some(bincode::deserialize(&row).unwrap()))
            }
            "user" => {
                ProjectsUsersRoleRequestActions::SetUser(Some(bincode::deserialize(&row).unwrap()))
            }
            "role" => {
                ProjectsUsersRoleRequestActions::SetRole(Some(bincode::deserialize(&row).unwrap()))
            }
            operation_name => {
                unreachable!("The operation name '{}' is not supported.", operation_name)
            }
        }
    }
}

impl Reducer<ProjectsUsersRoleRequestBuilder> for ProjectsUsersRoleRequestActions {
    fn apply(
        self,
        mut state: std::rc::Rc<ProjectsUsersRoleRequestBuilder>,
    ) -> std::rc::Rc<ProjectsUsersRoleRequestBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            ProjectsUsersRoleRequestActions::SetTable(table) => 'table: {
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
            ProjectsUsersRoleRequestActions::SetUser(user) => 'user: {
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
            ProjectsUsersRoleRequestActions::SetRole(role) => 'role: {
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
impl FormBuilder for ProjectsUsersRoleRequestBuilder {
    type Actions = ProjectsUsersRoleRequestActions;

    type RichVariant = NestedProjectsUsersRoleRequest;

    fn has_errors(&self) -> bool {
        !self.errors_table.is_empty()
            || !self.errors_user.is_empty()
            || !self.errors_role.is_empty()
    }

    fn update(
        dispatcher: &Dispatch<Self>,
        richest_variant: Self::RichVariant,
    ) -> Vec<ComponentMessage> {
        dispatcher.apply(ProjectsUsersRoleRequestActions::SetTable(
            Some(richest_variant.table).map(Rc::from),
        ));
        dispatcher.apply(ProjectsUsersRoleRequestActions::SetUser(
            Some(richest_variant.user).map(Rc::from),
        ));
        dispatcher.apply(ProjectsUsersRoleRequestActions::SetRole(
            Some(richest_variant.role).map(Rc::from),
        ));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors() && self.table.is_some() && self.user.is_some() && self.role.is_some()
    }
}

impl From<ProjectsUsersRoleRequestBuilder> for NewProjectsUsersRoleRequest {
    fn from(builder: ProjectsUsersRoleRequestBuilder) -> Self {
        Self {
            table_id: builder.table.as_ref().map(|table| table.inner.id).unwrap(),
            user_id: builder.user.as_ref().map(|user| user.inner.id).unwrap(),
            role_id: builder.role.as_deref().cloned().unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewProjectsUsersRoleRequest {
    type Builder = ProjectsUsersRoleRequestBuilder;
    fn title() -> &'static str {
        "Projects users role request"
    }
    fn task_target() -> &'static str {
        "Projects users role request"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateProjectsUsersRoleRequestFormProp {
    #[prop_or_default]
    pub table_id: Option<i32>,
    #[prop_or_default]
    pub user_id: Option<i32>,
    #[prop_or_default]
    pub role_id: Option<i32>,
}

#[function_component(CreateProjectsUsersRoleRequestForm)]
pub fn create_projects_users_role_request_form(
    props: &CreateProjectsUsersRoleRequestFormProp,
) -> Html {
    let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = yewdux::use_store::<ProjectsUsersRoleRequestBuilder>();
    if let Some(table_id) = props.table_id {
        named_requests.push(ComponentMessage::get_named::<&str, Project>(
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
    let set_table = builder_dispatch.apply_callback(
        |table: Option<Rc<web_common::database::nested_variants::NestedProject>>| {
            ProjectsUsersRoleRequestActions::SetTable(table)
        },
    );
    let set_user = builder_dispatch.apply_callback(
        |user: Option<Rc<web_common::database::nested_variants::NestedUser>>| {
            ProjectsUsersRoleRequestActions::SetUser(user)
        },
    );
    let set_role = builder_dispatch.apply_callback(
        |role: Option<Rc<web_common::database::nested_variants::NestedRole>>| {
            ProjectsUsersRoleRequestActions::SetRole(role)
        },
    );
    html! {
        <BasicForm<NewProjectsUsersRoleRequest>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Datalist<web_common::database::nested_variants::NestedProject, true> builder={set_table} optional={false} errors={builder_store.errors_table.clone()} value={builder_store.table.clone()} label="Table" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedUser, false> builder={set_user} optional={false} errors={builder_store.errors_user.clone()} value={builder_store.user.clone()} label="User" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedRole, false> builder={set_role} optional={false} errors={builder_store.errors_role.clone()} value={builder_store.role.clone()} label="Role" scanner={false} />
        </BasicForm<NewProjectsUsersRoleRequest>>
    }
}
