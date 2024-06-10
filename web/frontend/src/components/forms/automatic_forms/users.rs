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
use web_common::types::JPEG;
use yew::prelude::*;
use yewdux::Dispatch;
use yewdux::{Reducer, Store};
#[derive(Store, Eq, PartialEq, PartialOrd, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserBuilder {
    pub id: Option<i32>,
    pub first_name: Option<Rc<String>>,
    pub middle_name: Option<Rc<String>>,
    pub last_name: Option<Rc<String>>,
    pub description: Option<Rc<String>>,
    pub picture: Option<Rc<JPEG>>,
    pub organization: Option<Rc<NestedOrganization>>,
    pub errors_first_name: Vec<ApiError>,
    pub errors_middle_name: Vec<ApiError>,
    pub errors_last_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
    pub errors_picture: Vec<ApiError>,
    pub errors_organization: Vec<ApiError>,
    pub form_updated_at: chrono::NaiveDateTime,
}

unsafe impl Send for UserBuilder {}
unsafe impl Sync for UserBuilder {}
impl Default for UserBuilder {
    fn default() -> Self {
        Self {
            id: None,
            first_name: None,
            middle_name: None,
            last_name: None,
            description: None,
            picture: None,
            organization: Default::default(),
            errors_first_name: Default::default(),
            errors_middle_name: Default::default(),
            errors_last_name: Default::default(),
            errors_description: Default::default(),
            errors_picture: Default::default(),
            errors_organization: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(Eq, PartialEq, PartialOrd, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) enum UserActions {
    SetFirstName(Option<String>),
    SetMiddleName(Option<String>),
    SetLastName(Option<String>),
    SetDescription(Option<String>),
    SetPicture(Option<Rc<JPEG>>),
    SetOrganization(Option<Rc<NestedOrganization>>),
}

impl FromOperation for UserActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "organization" => {
                UserActions::SetOrganization(Some(bincode::deserialize(&row).unwrap()))
            }
            operation_name => {
                unreachable!("The operation name '{}' is not supported.", operation_name)
            }
        }
    }
}

impl Reducer<UserBuilder> for UserActions {
    fn apply(self, mut state: std::rc::Rc<UserBuilder>) -> std::rc::Rc<UserBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            UserActions::SetFirstName(first_name) => 'first_name: {
                state_mut.errors_first_name.clear();
                if first_name.is_none() {
                    state_mut.errors_first_name.push(ApiError::BadRequest(vec![
                        "The First name field is required.".to_string(),
                    ]));
                    state_mut.first_name = None;
                    break 'first_name;
                }
                if let Some(value) = first_name.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_first_name.push(ApiError::BadRequest(vec![
                            "The First name field cannot be left empty.".to_string(),
                        ]));
                        state_mut.first_name = None;
                        break 'first_name;
                    }
                }
                state_mut.first_name = first_name.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'first_name;
            }
            UserActions::SetMiddleName(middle_name) => 'middle_name: {
                state_mut.errors_middle_name.clear();
                if let Some(value) = middle_name.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_middle_name.push(ApiError::BadRequest(vec![
                            "The Middle name field cannot be left empty.".to_string(),
                        ]));
                        state_mut.middle_name = None;
                        break 'middle_name;
                    }
                }
                state_mut.middle_name = middle_name.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'middle_name;
            }
            UserActions::SetLastName(last_name) => 'last_name: {
                state_mut.errors_last_name.clear();
                if last_name.is_none() {
                    state_mut.errors_last_name.push(ApiError::BadRequest(vec![
                        "The Last name field is required.".to_string(),
                    ]));
                    state_mut.last_name = None;
                    break 'last_name;
                }
                if let Some(value) = last_name.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_last_name.push(ApiError::BadRequest(vec![
                            "The Last name field cannot be left empty.".to_string(),
                        ]));
                        state_mut.last_name = None;
                        break 'last_name;
                    }
                }
                state_mut.last_name = last_name.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'last_name;
            }
            UserActions::SetDescription(description) => 'description: {
                state_mut.errors_description.clear();
                if let Some(value) = description.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_description.push(ApiError::BadRequest(vec![
                            "The Description field cannot be left empty.".to_string(),
                        ]));
                        state_mut.description = None;
                        break 'description;
                    }
                }
                state_mut.description = description.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'description;
            }
            UserActions::SetPicture(picture) => 'picture: {
                state_mut.errors_picture.clear();
                if picture.is_none() {
                    state_mut.errors_picture.push(ApiError::BadRequest(vec![
                        "The Picture field is required.".to_string(),
                    ]));
                    state_mut.picture = None;
                    break 'picture;
                }
                state_mut.picture = picture.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'picture;
            }
            UserActions::SetOrganization(organization) => 'organization: {
                state_mut.errors_organization.clear();
                state_mut.organization = organization.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'organization;
            }
        }
        state
    }
}
impl FormBuilder for UserBuilder {
    type Actions = UserActions;

    type RichVariant = NestedUser;

    fn has_errors(&self) -> bool {
        !self.errors_first_name.is_empty()
            || !self.errors_middle_name.is_empty()
            || !self.errors_last_name.is_empty()
            || !self.errors_description.is_empty()
            || !self.errors_picture.is_empty()
            || !self.errors_organization.is_empty()
    }

    fn update(
        dispatcher: &Dispatch<Self>,
        richest_variant: Self::RichVariant,
    ) -> Vec<ComponentMessage> {
        dispatcher.reduce_mut(|state| {
            state.id = Some(richest_variant.inner.id);
        });
        dispatcher.apply(UserActions::SetFirstName(Some(
            richest_variant.inner.first_name.to_string(),
        )));
        dispatcher.apply(UserActions::SetMiddleName(
            richest_variant
                .inner
                .middle_name
                .as_ref()
                .map(|middle_name| middle_name.to_string()),
        ));
        dispatcher.apply(UserActions::SetLastName(Some(
            richest_variant.inner.last_name.to_string(),
        )));
        dispatcher.apply(UserActions::SetDescription(
            richest_variant
                .inner
                .description
                .as_ref()
                .map(|description| description.to_string()),
        ));
        dispatcher.apply(UserActions::SetPicture(
            Some(richest_variant.inner.as_ref().clone().picture).map(Rc::from),
        ));
        dispatcher.apply(UserActions::SetOrganization(
            richest_variant.organization.map(Rc::from),
        ));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
            && self.first_name.is_some()
            && self.last_name.is_some()
            && self.picture.is_some()
    }
}

impl From<UserBuilder> for UpdateUser {
    fn from(builder: UserBuilder) -> Self {
        Self {
            id: builder.id.unwrap(),
            first_name: builder.first_name.as_deref().cloned().unwrap(),
            middle_name: builder.middle_name.as_deref().cloned(),
            last_name: builder.last_name.as_deref().cloned().unwrap(),
            description: builder.description.as_deref().cloned(),
            organization_id: builder
                .organization
                .as_deref()
                .cloned()
                .map(|organization| organization.inner.id),
            picture: builder.picture.as_deref().cloned().unwrap(),
        }
    }
}
impl FormBuildable for UpdateUser {
    type Builder = UserBuilder;
    fn title() -> &'static str {
        "User"
    }
    fn task_target() -> &'static str {
        "User"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct UpdateUserFormProp {
    pub id: i32,
}

#[function_component(UpdateUserForm)]
pub fn update_user_form(props: &UpdateUserFormProp) -> Html {
    let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = yewdux::use_store::<UserBuilder>();
    // We push the ID of the row to the named requests.
    let props = props.clone();
    named_requests.push(ComponentMessage::get::<UpdateUser>(props.id.into()));
    let set_first_name = builder_dispatch
        .apply_callback(|first_name: Option<String>| UserActions::SetFirstName(first_name));
    let set_middle_name = builder_dispatch
        .apply_callback(|middle_name: Option<String>| UserActions::SetMiddleName(middle_name));
    let set_last_name = builder_dispatch
        .apply_callback(|last_name: Option<String>| UserActions::SetLastName(last_name));
    let set_description = builder_dispatch
        .apply_callback(|description: Option<String>| UserActions::SetDescription(description));
    let set_picture =
        builder_dispatch.apply_callback(|picture: Option<Rc<web_common::types::JPEG>>| {
            UserActions::SetPicture(picture.clone())
        });
    let set_organization =
        builder_dispatch.apply_callback(|organization: Option<Rc<NestedOrganization>>| {
            UserActions::SetOrganization(organization)
        });
    html! {
        <BasicForm<UpdateUser>
            method={FormMethod::PUT}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="First name" optional={false} errors={builder_store.errors_first_name.clone()} builder={set_first_name} value={builder_store.first_name.clone()} />
            <BasicInput<String> label="Middle name" optional={true} errors={builder_store.errors_middle_name.clone()} builder={set_middle_name} value={builder_store.middle_name.clone()} />
            <BasicInput<String> label="Last name" optional={false} errors={builder_store.errors_last_name.clone()} builder={set_last_name} value={builder_store.last_name.clone()} />
            <BasicInput<String> label="Description" optional={true} errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
            <FileInput<web_common::types::JPEG> label="Picture" optional={false} errors={builder_store.errors_picture.clone()} builder={set_picture} file={builder_store.picture.clone()} />
            <Datalist<NestedOrganization, false> builder={set_organization} optional={true} errors={builder_store.errors_organization.clone()} value={builder_store.organization.clone()} label="Organization" scanner={false} />
        </BasicForm<UpdateUser>>
    }
}
