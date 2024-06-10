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
pub struct UsersUsersRoleInvitationBuilder {
    pub table: Option<Rc<NestedUser>>,
    pub user: Option<Rc<NestedUser>>,
    pub role: Option<Rc<NestedRole>>,
    pub errors_table: Vec<ApiError>,
    pub errors_user: Vec<ApiError>,
    pub errors_role: Vec<ApiError>,
    pub form_updated_at: chrono::NaiveDateTime,
}

unsafe impl Send for UsersUsersRoleInvitationBuilder {}
unsafe impl Sync for UsersUsersRoleInvitationBuilder {}
impl Default for UsersUsersRoleInvitationBuilder {
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
pub(crate) enum UsersUsersRoleInvitationActions {
    SetTable(Option<Rc<NestedUser>>),
    SetUser(Option<Rc<NestedUser>>),
    SetRole(Option<Rc<NestedRole>>),
}

impl FromOperation for UsersUsersRoleInvitationActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "table" => {
                UsersUsersRoleInvitationActions::SetTable(Some(bincode::deserialize(&row).unwrap()))
            }
            "user" => {
                UsersUsersRoleInvitationActions::SetUser(Some(bincode::deserialize(&row).unwrap()))
            }
            "role" => {
                UsersUsersRoleInvitationActions::SetRole(Some(bincode::deserialize(&row).unwrap()))
            }
            operation_name => {
                unreachable!("The operation name '{}' is not supported.", operation_name)
            }
        }
    }
}

impl Reducer<UsersUsersRoleInvitationBuilder> for UsersUsersRoleInvitationActions {
    fn apply(
        self,
        mut state: std::rc::Rc<UsersUsersRoleInvitationBuilder>,
    ) -> std::rc::Rc<UsersUsersRoleInvitationBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            UsersUsersRoleInvitationActions::SetTable(table) => 'table: {
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
            UsersUsersRoleInvitationActions::SetUser(user) => 'user: {
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
            UsersUsersRoleInvitationActions::SetRole(role) => 'role: {
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
impl FormBuilder for UsersUsersRoleInvitationBuilder {
    type Actions = UsersUsersRoleInvitationActions;

    type RichVariant = NestedUsersUsersRoleInvitation;

    fn has_errors(&self) -> bool {
        !self.errors_table.is_empty()
            || !self.errors_user.is_empty()
            || !self.errors_role.is_empty()
    }

    fn update(
        dispatcher: &Dispatch<Self>,
        richest_variant: Self::RichVariant,
    ) -> Vec<ComponentMessage> {
        dispatcher.apply(UsersUsersRoleInvitationActions::SetTable(
            Some(richest_variant.table).map(Rc::from),
        ));
        dispatcher.apply(UsersUsersRoleInvitationActions::SetUser(
            Some(richest_variant.user).map(Rc::from),
        ));
        dispatcher.apply(UsersUsersRoleInvitationActions::SetRole(
            Some(richest_variant.role).map(Rc::from),
        ));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors() && self.table.is_some() && self.user.is_some() && self.role.is_some()
    }
}

impl From<UsersUsersRoleInvitationBuilder> for NewUsersUsersRoleInvitation {
    fn from(builder: UsersUsersRoleInvitationBuilder) -> Self {
        Self {
            table_id: builder.table.as_ref().map(|table| table.inner.id).unwrap(),
            user_id: builder.user.as_ref().map(|user| user.inner.id).unwrap(),
            role_id: builder.role.as_deref().cloned().unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewUsersUsersRoleInvitation {
    type Builder = UsersUsersRoleInvitationBuilder;
    fn title() -> &'static str {
        "Users users role invitation"
    }
    fn task_target() -> &'static str {
        "Users users role invitation"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateUsersUsersRoleInvitationFormProp {
    #[prop_or_default]
    pub table_id: Option<i32>,
    #[prop_or_default]
    pub user_id: Option<i32>,
    #[prop_or_default]
    pub role_id: Option<i32>,
}

#[function_component(CreateUsersUsersRoleInvitationForm)]
pub fn create_users_users_role_invitation_form(
    props: &CreateUsersUsersRoleInvitationFormProp,
) -> Html {
    let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = yewdux::use_store::<UsersUsersRoleInvitationBuilder>();
    if let Some(table_id) = props.table_id {
        named_requests.push(ComponentMessage::get_named::<&str, User>(
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
    let set_table = builder_dispatch.apply_callback(|table: Option<Rc<NestedUser>>| {
        UsersUsersRoleInvitationActions::SetTable(table)
    });
    let set_user = builder_dispatch.apply_callback(|user: Option<Rc<NestedUser>>| {
        UsersUsersRoleInvitationActions::SetUser(user)
    });
    let set_role = builder_dispatch.apply_callback(|role: Option<Rc<NestedRole>>| {
        UsersUsersRoleInvitationActions::SetRole(role)
    });
    html! {
        <BasicForm<NewUsersUsersRoleInvitation>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Datalist<NestedUser, false> builder={set_table} optional={false} errors={builder_store.errors_table.clone()} value={builder_store.table.clone()} label="Table" scanner={false} />
            <Datalist<NestedUser, false> builder={set_user} optional={false} errors={builder_store.errors_user.clone()} value={builder_store.user.clone()} label="User" scanner={false} />
            <Datalist<NestedRole, false> builder={set_role} optional={false} errors={builder_store.errors_role.clone()} value={builder_store.role.clone()} label="Role" scanner={false} />
        </BasicForm<NewUsersUsersRoleInvitation>>
    }
}
