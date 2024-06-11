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
pub struct SampleBuilder {
    pub id: Option<uuid::Uuid>,
    pub notes: Option<Rc<String>>,
    pub container: Option<Rc<web_common::database::nested_variants::NestedSampleContainer>>,
    pub project: Option<Rc<web_common::database::nested_variants::NestedProject>>,
    pub sampled_by: Option<Rc<web_common::database::nested_variants::NestedUser>>,
    pub state: Option<Rc<web_common::database::nested_variants::NestedSampleState>>,
    pub errors_notes: Vec<ApiError>,
    pub errors_container: Vec<ApiError>,
    pub errors_project: Vec<ApiError>,
    pub errors_sampled_by: Vec<ApiError>,
    pub errors_state: Vec<ApiError>,
    pub form_updated_at: chrono::NaiveDateTime,
}

unsafe impl Send for SampleBuilder {}
unsafe impl Sync for SampleBuilder {}
impl Default for SampleBuilder {
    fn default() -> Self {
        Self {
            id: None,
            notes: None,
            container: Default::default(),
            project: Default::default(),
            sampled_by: None,
            state: Default::default(),
            errors_notes: Default::default(),
            errors_container: Default::default(),
            errors_project: Default::default(),
            errors_sampled_by: Default::default(),
            errors_state: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) enum SampleActions {
    SetNotes(Option<String>),
    SetContainer(Option<Rc<web_common::database::nested_variants::NestedSampleContainer>>),
    SetProject(Option<Rc<web_common::database::nested_variants::NestedProject>>),
    SetSampledBy(Option<Rc<web_common::database::nested_variants::NestedUser>>),
    SetState(Option<Rc<web_common::database::nested_variants::NestedSampleState>>),
}

impl FromOperation for SampleActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "container" => SampleActions::SetContainer(Some(bincode::deserialize(&row).unwrap())),
            "project" => SampleActions::SetProject(Some(bincode::deserialize(&row).unwrap())),
            "sampled_by" => SampleActions::SetSampledBy(Some(bincode::deserialize(&row).unwrap())),
            "state" => SampleActions::SetState(Some(bincode::deserialize(&row).unwrap())),
            operation_name => {
                unreachable!("The operation name '{}' is not supported.", operation_name)
            }
        }
    }
}

impl Reducer<SampleBuilder> for SampleActions {
    fn apply(self, mut state: std::rc::Rc<SampleBuilder>) -> std::rc::Rc<SampleBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            SampleActions::SetNotes(notes) => 'notes: {
                state_mut.errors_notes.clear();
                if let Some(value) = notes.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_notes.push(ApiError::BadRequest(vec![
                            "The Notes field cannot be left empty.".to_string(),
                        ]));
                        state_mut.notes = None;
                        break 'notes;
                    }
                }
                state_mut.notes = notes.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'notes;
            }
            SampleActions::SetContainer(container) => 'container: {
                state_mut.errors_container.clear();
                if container.is_none() {
                    state_mut.errors_container.push(ApiError::BadRequest(vec![
                        "The Container field is required.".to_string(),
                    ]));
                    state_mut.container = None;
                    break 'container;
                }
                state_mut.container = container.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'container;
            }
            SampleActions::SetProject(project) => 'project: {
                state_mut.errors_project.clear();
                if project.is_none() {
                    state_mut.errors_project.push(ApiError::BadRequest(vec![
                        "The Project field is required.".to_string(),
                    ]));
                    state_mut.project = None;
                    break 'project;
                }
                state_mut.project = project.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'project;
            }
            SampleActions::SetSampledBy(sampled_by) => 'sampled_by: {
                state_mut.errors_sampled_by.clear();
                if sampled_by.is_none() {
                    state_mut.errors_sampled_by.push(ApiError::BadRequest(vec![
                        "The Sampled by field is required.".to_string(),
                    ]));
                    state_mut.sampled_by = None;
                    break 'sampled_by;
                }
                state_mut.sampled_by = sampled_by.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'sampled_by;
            }
            SampleActions::SetState(state) => 'state: {
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
        }
        state
    }
}
impl FormBuilder for SampleBuilder {
    type Actions = SampleActions;

    type RichVariant = NestedSample;

    fn has_errors(&self) -> bool {
        !self.errors_notes.is_empty()
            || !self.errors_container.is_empty()
            || !self.errors_project.is_empty()
            || !self.errors_sampled_by.is_empty()
            || !self.errors_state.is_empty()
    }

    fn update(
        dispatcher: &Dispatch<Self>,
        richest_variant: Self::RichVariant,
    ) -> Vec<ComponentMessage> {
        dispatcher.reduce_mut(|state| {
            state.id = Some(richest_variant.inner.id);
        });
        dispatcher.apply(SampleActions::SetNotes(
            richest_variant
                .inner
                .notes
                .as_ref()
                .map(|notes| notes.to_string()),
        ));
        dispatcher.apply(SampleActions::SetContainer(
            Some(richest_variant.container).map(Rc::from),
        ));
        dispatcher.apply(SampleActions::SetProject(
            Some(richest_variant.project).map(Rc::from),
        ));
        dispatcher.apply(SampleActions::SetSampledBy(
            Some(richest_variant.sampled_by).map(Rc::from),
        ));
        dispatcher.apply(SampleActions::SetState(
            Some(richest_variant.state).map(Rc::from),
        ));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
            && self.container.is_some()
            && self.project.is_some()
            && self.sampled_by.is_some()
            && self.state.is_some()
    }
}

impl From<SampleBuilder> for NewSample {
    fn from(builder: SampleBuilder) -> Self {
        Self {
            id: builder.id.unwrap_or_else(uuid::Uuid::new_v4),
            container_id: builder.container.as_deref().cloned().unwrap().inner.id,
            notes: builder.notes.as_deref().cloned(),
            project_id: builder.project.as_deref().cloned().unwrap().inner.id,
            sampled_by: builder.sampled_by.as_deref().cloned().unwrap().inner.id,
            state_id: builder.state.as_deref().cloned().unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewSample {
    type Builder = SampleBuilder;
    fn title() -> &'static str {
        "Sample"
    }
    fn task_target() -> &'static str {
        "Sample"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateSampleFormProp {
    #[prop_or_default]
    pub container_id: Option<i32>,
    #[prop_or_default]
    pub project_id: Option<i32>,
    #[prop_or_default]
    pub sampled_by: Option<i32>,
    #[prop_or(1)]
    pub state_id: i32,
}

#[function_component(CreateSampleForm)]
pub fn create_sample_form(props: &CreateSampleFormProp) -> Html {
    let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = yewdux::use_store::<SampleBuilder>();
    let user_state = yewdux::use_store_value::<crate::stores::user_state::UserState>();
    if let Some(container_id) = props.container_id {
        named_requests.push(ComponentMessage::get_named::<&str, SampleContainer>(
            "container",
            container_id.into(),
        ));
    }
    if let Some(project_id) = props.project_id {
        named_requests.push(ComponentMessage::get_named::<&str, Project>(
            "project",
            project_id.into(),
        ));
    }
    if let Some(sampled_by) = props.sampled_by {
        named_requests.push(ComponentMessage::get_named::<&str, User>(
            "sampled_by",
            sampled_by.into(),
        ));
    } else if let Some(user) = user_state.as_ref().user() {
        builder_dispatch.apply(SampleActions::SetSampledBy(Some(user)));
    }
    named_requests.push(ComponentMessage::get_named::<&str, SampleState>(
        "state",
        props.state_id.into(),
    ));
    let set_notes =
        builder_dispatch.apply_callback(|notes: Option<String>| SampleActions::SetNotes(notes));
    let set_container = builder_dispatch.apply_callback(
        |container: Option<Rc<web_common::database::nested_variants::NestedSampleContainer>>| {
            SampleActions::SetContainer(container)
        },
    );
    let set_project = builder_dispatch.apply_callback(
        |project: Option<Rc<web_common::database::nested_variants::NestedProject>>| {
            SampleActions::SetProject(project)
        },
    );
    let set_sampled_by = builder_dispatch.apply_callback(
        |sampled_by: Option<Rc<web_common::database::nested_variants::NestedUser>>| {
            SampleActions::SetSampledBy(sampled_by)
        },
    );
    let set_state = builder_dispatch.apply_callback(
        |state: Option<Rc<web_common::database::nested_variants::NestedSampleState>>| {
            SampleActions::SetState(state)
        },
    );
    html! {
        <BasicForm<NewSample>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Notes" optional={true} errors={builder_store.errors_notes.clone()} builder={set_notes} value={builder_store.notes.clone()} />
            <Datalist<web_common::database::nested_variants::NestedSampleContainer, false> builder={set_container} optional={false} errors={builder_store.errors_container.clone()} value={builder_store.container.clone()} label="Container" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedProject, true> builder={set_project} optional={false} errors={builder_store.errors_project.clone()} value={builder_store.project.clone()} label="Project" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedUser, false> builder={set_sampled_by} optional={false} errors={builder_store.errors_sampled_by.clone()} value={builder_store.sampled_by.clone()} label="Sampled by" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedSampleState, false> builder={set_state} optional={false} errors={builder_store.errors_state.clone()} value={builder_store.state.clone()} label="State" scanner={false} />
        </BasicForm<NewSample>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateSampleFormProp {
    pub id: uuid::Uuid,
}

#[function_component(UpdateSampleForm)]
pub fn update_sample_form(props: &UpdateSampleFormProp) -> Html {
    let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = yewdux::use_store::<SampleBuilder>();
    // We push the ID of the row to the named requests.
    let props = props.clone();
    named_requests.push(ComponentMessage::get::<NewSample>(props.id.into()));
    let set_notes =
        builder_dispatch.apply_callback(|notes: Option<String>| SampleActions::SetNotes(notes));
    let set_container = builder_dispatch.apply_callback(
        |container: Option<Rc<web_common::database::nested_variants::NestedSampleContainer>>| {
            SampleActions::SetContainer(container)
        },
    );
    let set_project = builder_dispatch.apply_callback(
        |project: Option<Rc<web_common::database::nested_variants::NestedProject>>| {
            SampleActions::SetProject(project)
        },
    );
    let set_sampled_by = builder_dispatch.apply_callback(
        |sampled_by: Option<Rc<web_common::database::nested_variants::NestedUser>>| {
            SampleActions::SetSampledBy(sampled_by)
        },
    );
    let set_state = builder_dispatch.apply_callback(
        |state: Option<Rc<web_common::database::nested_variants::NestedSampleState>>| {
            SampleActions::SetState(state)
        },
    );
    html! {
        <BasicForm<NewSample>
            method={FormMethod::PUT}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Notes" optional={true} errors={builder_store.errors_notes.clone()} builder={set_notes} value={builder_store.notes.clone()} />
            <Datalist<web_common::database::nested_variants::NestedSampleContainer, false> builder={set_container} optional={false} errors={builder_store.errors_container.clone()} value={builder_store.container.clone()} label="Container" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedProject, true> builder={set_project} optional={false} errors={builder_store.errors_project.clone()} value={builder_store.project.clone()} label="Project" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedUser, false> builder={set_sampled_by} optional={false} errors={builder_store.errors_sampled_by.clone()} value={builder_store.sampled_by.clone()} label="Sampled by" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedSampleState, false> builder={set_state} optional={false} errors={builder_store.errors_state.clone()} value={builder_store.state.clone()} label="State" scanner={false} />
        </BasicForm<NewSample>>
    }
}
