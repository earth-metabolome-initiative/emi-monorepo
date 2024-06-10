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
#[derive(Store, PartialEq, PartialOrd, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ObservationBuilder {
    pub id: Option<uuid::Uuid>,
    pub notes: Option<Rc<String>>,
    pub picture: Option<Rc<JPEG>>,
    pub parent_observation: Option<Rc<NestedObservation>>,
    pub project: Option<Rc<NestedProject>>,
    pub organism: Option<Rc<NestedOrganism>>,
    pub sample: Option<Rc<NestedSample>>,
    pub subject: Option<Rc<NestedObservationSubject>>,
    pub errors_notes: Vec<ApiError>,
    pub errors_picture: Vec<ApiError>,
    pub errors_parent_observation: Vec<ApiError>,
    pub errors_project: Vec<ApiError>,
    pub errors_organism: Vec<ApiError>,
    pub errors_sample: Vec<ApiError>,
    pub errors_subject: Vec<ApiError>,
    pub form_updated_at: chrono::NaiveDateTime,
}

unsafe impl Send for ObservationBuilder {}
unsafe impl Sync for ObservationBuilder {}
impl Default for ObservationBuilder {
    fn default() -> Self {
        Self {
            id: None,
            notes: None,
            picture: None,
            parent_observation: Default::default(),
            project: Default::default(),
            organism: Default::default(),
            sample: Default::default(),
            subject: Default::default(),
            errors_notes: Default::default(),
            errors_picture: Default::default(),
            errors_parent_observation: Default::default(),
            errors_project: Default::default(),
            errors_organism: Default::default(),
            errors_sample: Default::default(),
            errors_subject: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) enum ObservationActions {
    SetNotes(Option<String>),
    SetPicture(Option<Rc<JPEG>>),
    SetParentObservation(Option<Rc<NestedObservation>>),
    SetProject(Option<Rc<NestedProject>>),
    SetOrganism(Option<Rc<NestedOrganism>>),
    SetSample(Option<Rc<NestedSample>>),
    SetSubject(Option<Rc<NestedObservationSubject>>),
}

impl FromOperation for ObservationActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "parent_observation" => {
                ObservationActions::SetParentObservation(Some(bincode::deserialize(&row).unwrap()))
            }
            "project" => ObservationActions::SetProject(Some(bincode::deserialize(&row).unwrap())),
            "organism" => {
                ObservationActions::SetOrganism(Some(bincode::deserialize(&row).unwrap()))
            }
            "sample" => ObservationActions::SetSample(Some(bincode::deserialize(&row).unwrap())),
            "subject" => ObservationActions::SetSubject(Some(bincode::deserialize(&row).unwrap())),
            operation_name => {
                unreachable!("The operation name '{}' is not supported.", operation_name)
            }
        }
    }
}

impl Reducer<ObservationBuilder> for ObservationActions {
    fn apply(self, mut state: std::rc::Rc<ObservationBuilder>) -> std::rc::Rc<ObservationBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            ObservationActions::SetNotes(notes) => 'notes: {
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
            ObservationActions::SetPicture(picture) => 'picture: {
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
            ObservationActions::SetParentObservation(parent_observation) => 'parent_observation: {
                state_mut.errors_parent_observation.clear();
                match parent_observation.as_ref() {
                    Some(parent_observation) => {
                        if state_mut
                            .id
                            .map_or(false, |id| id == parent_observation.inner.id)
                        {
                            state_mut.errors_parent_observation.push(ApiError::BadRequest(vec![
                                "The Parent observation field must be distinct from the current value.".to_string()
                             ]));
                            break 'parent_observation;
                        }
                    }
                    None => (),
                }
                state_mut.parent_observation = parent_observation.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'parent_observation;
            }
            ObservationActions::SetProject(project) => 'project: {
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
            ObservationActions::SetOrganism(organism) => 'organism: {
                state_mut.errors_organism.clear();
                state_mut.organism = organism.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'organism;
            }
            ObservationActions::SetSample(sample) => 'sample: {
                state_mut.errors_sample.clear();
                state_mut.sample = sample.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'sample;
            }
            ObservationActions::SetSubject(subject) => 'subject: {
                state_mut.errors_subject.clear();
                if subject.is_none() {
                    state_mut.errors_subject.push(ApiError::BadRequest(vec![
                        "The Subject field is required.".to_string(),
                    ]));
                    state_mut.subject = None;
                    break 'subject;
                }
                state_mut.subject = subject.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'subject;
            }
        }
        state
    }
}
impl FormBuilder for ObservationBuilder {
    type Actions = ObservationActions;

    type RichVariant = NestedObservation;

    fn has_errors(&self) -> bool {
        !self.errors_notes.is_empty()
            || !self.errors_picture.is_empty()
            || !self.errors_parent_observation.is_empty()
            || !self.errors_project.is_empty()
            || !self.errors_organism.is_empty()
            || !self.errors_sample.is_empty()
            || !self.errors_subject.is_empty()
    }

    fn update(
        dispatcher: &Dispatch<Self>,
        richest_variant: Self::RichVariant,
    ) -> Vec<ComponentMessage> {
        dispatcher.reduce_mut(|state| {
            state.id = Some(richest_variant.inner.id);
        });
        dispatcher.apply(ObservationActions::SetNotes(
            richest_variant
                .inner
                .notes
                .as_ref()
                .map(|notes| notes.to_string()),
        ));
        dispatcher.apply(ObservationActions::SetPicture(
            Some(richest_variant.inner.as_ref().clone().picture).map(Rc::from),
        ));
        dispatcher.apply(ObservationActions::SetProject(
            Some(richest_variant.project).map(Rc::from),
        ));
        dispatcher.apply(ObservationActions::SetOrganism(
            richest_variant.organism.map(Rc::from),
        ));
        dispatcher.apply(ObservationActions::SetSample(
            richest_variant.sample.map(Rc::from),
        ));
        dispatcher.apply(ObservationActions::SetSubject(
            Some(richest_variant.subject).map(Rc::from),
        ));
        let mut named_requests = Vec::new();
        if let Some(parent_observation_id) = richest_variant.inner.parent_observation_id {
            named_requests.push(ComponentMessage::get_named::<&str, Observation>(
                "parent_observation",
                parent_observation_id.into(),
            ));
        } else {
            dispatcher.apply(ObservationActions::SetParentObservation(None));
        }
        named_requests
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
            && self.picture.is_some()
            && self.project.is_some()
            && self.subject.is_some()
    }
}

impl From<ObservationBuilder> for NewObservation {
    fn from(builder: ObservationBuilder) -> Self {
        Self {
            id: builder.id.unwrap_or_else(uuid::Uuid::new_v4),
            parent_observation_id: builder
                .parent_observation
                .as_deref()
                .cloned()
                .map(|parent_observation| parent_observation.inner.id),
            project_id: builder.project.as_deref().cloned().unwrap().inner.id,
            organism_id: builder
                .organism
                .as_deref()
                .cloned()
                .map(|organism| organism.inner.id),
            sample_id: builder
                .sample
                .as_deref()
                .cloned()
                .map(|sample| sample.inner.id),
            subject_id: builder.subject.as_deref().cloned().unwrap().inner.id,
            notes: builder.notes.as_deref().cloned(),
            picture: builder.picture.as_deref().cloned().unwrap(),
        }
    }
}
impl FormBuildable for NewObservation {
    type Builder = ObservationBuilder;
    fn title() -> &'static str {
        "Observation"
    }
    fn task_target() -> &'static str {
        "Observation"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateObservationFormProp {
    #[prop_or_default]
    pub parent_observation_id: Option<uuid::Uuid>,
    #[prop_or_default]
    pub project_id: Option<i32>,
    #[prop_or_default]
    pub organism_id: Option<uuid::Uuid>,
    #[prop_or_default]
    pub sample_id: Option<uuid::Uuid>,
    #[prop_or_default]
    pub subject_id: Option<i32>,
}

#[function_component(CreateObservationForm)]
pub fn create_observation_form(props: &CreateObservationFormProp) -> Html {
    let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = yewdux::use_store::<ObservationBuilder>();
    if let Some(parent_observation_id) = props.parent_observation_id {
        named_requests.push(ComponentMessage::get_named::<&str, Observation>(
            "parent_observation",
            parent_observation_id.into(),
        ));
    }
    if let Some(project_id) = props.project_id {
        named_requests.push(ComponentMessage::get_named::<&str, Project>(
            "project",
            project_id.into(),
        ));
    }
    if let Some(organism_id) = props.organism_id {
        named_requests.push(ComponentMessage::get_named::<&str, Organism>(
            "organism",
            organism_id.into(),
        ));
    }
    if let Some(sample_id) = props.sample_id {
        named_requests.push(ComponentMessage::get_named::<&str, Sample>(
            "sample",
            sample_id.into(),
        ));
    }
    if let Some(subject_id) = props.subject_id {
        named_requests.push(ComponentMessage::get_named::<&str, ObservationSubject>(
            "subject",
            subject_id.into(),
        ));
    }
    let set_notes = builder_dispatch
        .apply_callback(|notes: Option<String>| ObservationActions::SetNotes(notes));
    let set_picture =
        builder_dispatch.apply_callback(|picture: Option<Rc<web_common::types::JPEG>>| {
            ObservationActions::SetPicture(picture.clone())
        });
    let set_parent_observation =
        builder_dispatch.apply_callback(|parent_observation: Option<Rc<NestedObservation>>| {
            ObservationActions::SetParentObservation(parent_observation)
        });
    let set_project = builder_dispatch.apply_callback(|project: Option<Rc<NestedProject>>| {
        ObservationActions::SetProject(project)
    });
    let set_organism = builder_dispatch.apply_callback(|organism: Option<Rc<NestedOrganism>>| {
        ObservationActions::SetOrganism(organism)
    });
    let set_sample = builder_dispatch
        .apply_callback(|sample: Option<Rc<NestedSample>>| ObservationActions::SetSample(sample));
    let set_subject =
        builder_dispatch.apply_callback(|subject: Option<Rc<NestedObservationSubject>>| {
            ObservationActions::SetSubject(subject)
        });
    html! {
        <BasicForm<NewObservation>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Notes" optional={true} errors={builder_store.errors_notes.clone()} builder={set_notes} value={builder_store.notes.clone()} />
            <FileInput<web_common::types::JPEG> label="Picture" optional={false} errors={builder_store.errors_picture.clone()} builder={set_picture} file={builder_store.picture.clone()} />
            <Datalist<NestedObservation, false> builder={set_parent_observation} optional={true} errors={builder_store.errors_parent_observation.clone()} value={builder_store.parent_observation.clone()} label="Parent observation" scanner={false} />
            <Datalist<NestedProject, true> builder={set_project} optional={false} errors={builder_store.errors_project.clone()} value={builder_store.project.clone()} label="Project" scanner={false} />
            <Datalist<NestedOrganism, false> builder={set_organism} optional={true} errors={builder_store.errors_organism.clone()} value={builder_store.organism.clone()} label="Organism" scanner={false} />
            <Datalist<NestedSample, false> builder={set_sample} optional={true} errors={builder_store.errors_sample.clone()} value={builder_store.sample.clone()} label="Sample" scanner={false} />
            <Datalist<NestedObservationSubject, false> builder={set_subject} optional={false} errors={builder_store.errors_subject.clone()} value={builder_store.subject.clone()} label="Subject" scanner={false} />
        </BasicForm<NewObservation>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateObservationFormProp {
    pub id: uuid::Uuid,
}

#[function_component(UpdateObservationForm)]
pub fn update_observation_form(props: &UpdateObservationFormProp) -> Html {
    let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = yewdux::use_store::<ObservationBuilder>();
    // We push the ID of the row to the named requests.
    let props = props.clone();
    named_requests.push(ComponentMessage::get::<NewObservation>(props.id.into()));
    let set_notes = builder_dispatch
        .apply_callback(|notes: Option<String>| ObservationActions::SetNotes(notes));
    let set_picture =
        builder_dispatch.apply_callback(|picture: Option<Rc<web_common::types::JPEG>>| {
            ObservationActions::SetPicture(picture.clone())
        });
    let set_parent_observation =
        builder_dispatch.apply_callback(|parent_observation: Option<Rc<NestedObservation>>| {
            ObservationActions::SetParentObservation(parent_observation)
        });
    let set_project = builder_dispatch.apply_callback(|project: Option<Rc<NestedProject>>| {
        ObservationActions::SetProject(project)
    });
    let set_organism = builder_dispatch.apply_callback(|organism: Option<Rc<NestedOrganism>>| {
        ObservationActions::SetOrganism(organism)
    });
    let set_sample = builder_dispatch
        .apply_callback(|sample: Option<Rc<NestedSample>>| ObservationActions::SetSample(sample));
    let set_subject =
        builder_dispatch.apply_callback(|subject: Option<Rc<NestedObservationSubject>>| {
            ObservationActions::SetSubject(subject)
        });
    html! {
        <BasicForm<NewObservation>
            method={FormMethod::PUT}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Notes" optional={true} errors={builder_store.errors_notes.clone()} builder={set_notes} value={builder_store.notes.clone()} />
            <FileInput<web_common::types::JPEG> label="Picture" optional={false} errors={builder_store.errors_picture.clone()} builder={set_picture} file={builder_store.picture.clone()} />
            <Datalist<NestedObservation, false> builder={set_parent_observation} optional={true} errors={builder_store.errors_parent_observation.clone()} value={builder_store.parent_observation.clone()} label="Parent observation" scanner={false} />
            <Datalist<NestedProject, true> builder={set_project} optional={false} errors={builder_store.errors_project.clone()} value={builder_store.project.clone()} label="Project" scanner={false} />
            <Datalist<NestedOrganism, false> builder={set_organism} optional={true} errors={builder_store.errors_organism.clone()} value={builder_store.organism.clone()} label="Organism" scanner={false} />
            <Datalist<NestedSample, false> builder={set_sample} optional={true} errors={builder_store.errors_sample.clone()} value={builder_store.sample.clone()} label="Sample" scanner={false} />
            <Datalist<NestedObservationSubject, false> builder={set_subject} optional={false} errors={builder_store.errors_subject.clone()} value={builder_store.subject.clone()} label="Subject" scanner={false} />
        </BasicForm<NewObservation>>
    }
}
