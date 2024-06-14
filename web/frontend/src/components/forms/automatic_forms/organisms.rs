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
pub struct OrganismBuilder {
    pub id: Option<uuid::Uuid>,
    pub notes: Option<Rc<String>>,
    pub wild: Option<bool>,
    pub host_organism: Option<Rc<web_common::database::nested_variants::NestedOrganism>>,
    pub sample: Option<Rc<web_common::database::nested_variants::NestedSample>>,
    pub nameplate: Option<Rc<web_common::database::nested_variants::NestedNameplate>>,
    pub project: Option<Rc<web_common::database::nested_variants::NestedProject>>,
    pub errors_notes: Vec<ApiError>,
    pub errors_wild: Vec<ApiError>,
    pub errors_host_organism: Vec<ApiError>,
    pub errors_sample: Vec<ApiError>,
    pub errors_nameplate: Vec<ApiError>,
    pub errors_project: Vec<ApiError>,
    pub form_updated_at: chrono::NaiveDateTime,
}

unsafe impl Send for OrganismBuilder {}
unsafe impl Sync for OrganismBuilder {}
impl Default for OrganismBuilder {
    fn default() -> Self {
        Self {
            id: None,
            notes: None,
            wild: Some(true),
            host_organism: Default::default(),
            sample: Default::default(),
            nameplate: Default::default(),
            project: Default::default(),
            errors_notes: Default::default(),
            errors_wild: Default::default(),
            errors_host_organism: Default::default(),
            errors_sample: Default::default(),
            errors_nameplate: Default::default(),
            errors_project: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) enum OrganismActions {
    SetNotes(Option<String>),
    SetWild(Option<bool>),
    SetHostOrganism(Option<Rc<web_common::database::nested_variants::NestedOrganism>>),
    SetSample(Option<Rc<web_common::database::nested_variants::NestedSample>>),
    SetNameplate(Option<Rc<web_common::database::nested_variants::NestedNameplate>>),
    SetProject(Option<Rc<web_common::database::nested_variants::NestedProject>>),
}

impl FromOperation for OrganismActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "host_organism" => {
                OrganismActions::SetHostOrganism(Some(bincode::deserialize(&row).unwrap()))
            }
            "sample" => OrganismActions::SetSample(Some(bincode::deserialize(&row).unwrap())),
            "nameplate" => OrganismActions::SetNameplate(Some(bincode::deserialize(&row).unwrap())),
            "project" => OrganismActions::SetProject(Some(bincode::deserialize(&row).unwrap())),
            operation_name => {
                unreachable!("The operation name '{}' is not supported.", operation_name)
            }
        }
    }
}

impl Reducer<OrganismBuilder> for OrganismActions {
    fn apply(self, mut state: std::rc::Rc<OrganismBuilder>) -> std::rc::Rc<OrganismBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            OrganismActions::SetNotes(notes) => 'notes: {
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
            OrganismActions::SetWild(wild) => 'wild: {
                state_mut.errors_wild.clear();
                if wild.is_none() {
                    state_mut.errors_wild.push(ApiError::BadRequest(vec![
                        "The Wild field is required.".to_string(),
                    ]));
                    state_mut.wild = None;
                    break 'wild;
                }
                state_mut.wild = wild;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'wild;
            }
            OrganismActions::SetHostOrganism(host_organism) => 'host_organism: {
                state_mut.errors_host_organism.clear();
                match host_organism.as_ref() {
                    Some(host_organism) => {
                        if state_mut
                            .id
                            .map_or(false, |id| id == host_organism.inner.id)
                        {
                            state_mut
                                .errors_host_organism
                                .push(ApiError::BadRequest(vec![
                                "The Host organism field must be distinct from the current value."
                                    .to_string(),
                            ]));
                            break 'host_organism;
                        }
                    }
                    None => (),
                }
                state_mut.host_organism = host_organism.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'host_organism;
            }
            OrganismActions::SetSample(sample) => 'sample: {
                state_mut.errors_sample.clear();
                state_mut.sample = sample.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'sample;
            }
            OrganismActions::SetNameplate(nameplate) => 'nameplate: {
                state_mut.errors_nameplate.clear();
                if nameplate.is_none() {
                    state_mut.errors_nameplate.push(ApiError::BadRequest(vec![
                        "The Nameplate field is required.".to_string(),
                    ]));
                    state_mut.nameplate = None;
                    break 'nameplate;
                }
                state_mut.nameplate = nameplate.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'nameplate;
            }
            OrganismActions::SetProject(project) => 'project: {
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
        }
        state
    }
}
impl FormBuilder for OrganismBuilder {
    type Actions = OrganismActions;

    type RichVariant = NestedOrganism;

    fn has_errors(&self) -> bool {
        !self.errors_notes.is_empty()
            || !self.errors_wild.is_empty()
            || !self.errors_host_organism.is_empty()
            || !self.errors_sample.is_empty()
            || !self.errors_nameplate.is_empty()
            || !self.errors_project.is_empty()
    }

    fn update(
        dispatcher: &Dispatch<Self>,
        richest_variant: Self::RichVariant,
    ) -> Vec<ComponentMessage> {
        dispatcher.reduce_mut(|state| {
            state.id = Some(richest_variant.inner.id);
        });
        dispatcher.apply(OrganismActions::SetNotes(
            richest_variant
                .inner
                .notes
                .as_ref()
                .map(|notes| notes.to_string()),
        ));
        dispatcher.apply(OrganismActions::SetWild(Some(richest_variant.inner.wild)));
        dispatcher.apply(OrganismActions::SetSample(
            richest_variant.sample.map(Rc::from),
        ));
        dispatcher.apply(OrganismActions::SetNameplate(
            Some(richest_variant.nameplate).map(Rc::from),
        ));
        dispatcher.apply(OrganismActions::SetProject(
            Some(richest_variant.project).map(Rc::from),
        ));
        let mut named_requests = Vec::new();
        if let Some(host_organism_id) = richest_variant.inner.host_organism_id {
            named_requests.push(ComponentMessage::get_named::<&str, Organism>(
                "host_organism",
                host_organism_id.into(),
            ));
        } else {
            dispatcher.apply(OrganismActions::SetHostOrganism(None));
        }
        named_requests
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
            && self.wild.is_some()
            && self.nameplate.is_some()
            && self.project.is_some()
    }
}

impl From<OrganismBuilder> for NewOrganism {
    fn from(builder: OrganismBuilder) -> Self {
        Self {
            id: builder.id.unwrap_or_else(uuid::Uuid::new_v4),
            host_organism_id: builder
                .host_organism
                .as_deref()
                .cloned()
                .map(|host_organism| host_organism.inner.id),
            sample_id: builder
                .sample
                .as_deref()
                .cloned()
                .map(|sample| sample.inner.id),
            notes: builder.notes.as_deref().cloned(),
            wild: builder.wild.unwrap(),
            nameplate_id: builder.nameplate.as_deref().cloned().unwrap().inner.id,
            project_id: builder.project.as_deref().cloned().unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewOrganism {
    type Builder = OrganismBuilder;
    fn title() -> &'static str {
        "Organism"
    }
    fn task_target() -> &'static str {
        "Organism"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateOrganismFormProp {
    #[prop_or_default]
    pub host_organism_id: Option<uuid::Uuid>,
    #[prop_or_default]
    pub sample_id: Option<uuid::Uuid>,
    #[prop_or_default]
    pub nameplate_id: Option<i32>,
    #[prop_or_default]
    pub project_id: Option<i32>,
}

#[function_component(CreateOrganismForm)]
pub fn create_organism_form(props: &CreateOrganismFormProp) -> Html {
    let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = yewdux::use_store::<OrganismBuilder>();
    if let Some(host_organism_id) = props.host_organism_id {
        named_requests.push(ComponentMessage::get_named::<&str, Organism>(
            "host_organism",
            host_organism_id.into(),
        ));
    }
    if let Some(sample_id) = props.sample_id {
        named_requests.push(ComponentMessage::get_named::<&str, Sample>(
            "sample",
            sample_id.into(),
        ));
    }
    if let Some(nameplate_id) = props.nameplate_id {
        named_requests.push(ComponentMessage::get_named::<&str, Nameplate>(
            "nameplate",
            nameplate_id.into(),
        ));
    }
    if let Some(project_id) = props.project_id {
        named_requests.push(ComponentMessage::get_named::<&str, Project>(
            "project",
            project_id.into(),
        ));
    }
    let set_notes =
        builder_dispatch.apply_callback(|notes: Option<String>| OrganismActions::SetNotes(notes));
    let set_wild =
        builder_dispatch.apply_callback(|wild: bool| OrganismActions::SetWild(Some(wild)));
    let set_host_organism = builder_dispatch.apply_callback(
        |host_organism: Option<Rc<web_common::database::nested_variants::NestedOrganism>>| {
            OrganismActions::SetHostOrganism(host_organism)
        },
    );
    let set_sample = builder_dispatch.apply_callback(
        |sample: Option<Rc<web_common::database::nested_variants::NestedSample>>| {
            OrganismActions::SetSample(sample)
        },
    );
    let set_nameplate = builder_dispatch.apply_callback(
        |nameplate: Option<Rc<web_common::database::nested_variants::NestedNameplate>>| {
            OrganismActions::SetNameplate(nameplate)
        },
    );
    let set_project = builder_dispatch.apply_callback(
        |project: Option<Rc<web_common::database::nested_variants::NestedProject>>| {
            OrganismActions::SetProject(project)
        },
    );
    html! {
        <BasicForm<NewOrganism>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Notes" optional={true} errors={builder_store.errors_notes.clone()} builder={set_notes} value={builder_store.notes.clone()} />
            <Checkbox label="Wild" errors={builder_store.errors_wild.clone()} builder={set_wild} value={builder_store.wild.unwrap_or(false)} />
            <Datalist<web_common::database::nested_variants::NestedOrganism, false> builder={set_host_organism} optional={true} errors={builder_store.errors_host_organism.clone()} value={builder_store.host_organism.clone()} label="Host organism" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedSample, false> builder={set_sample} optional={true} errors={builder_store.errors_sample.clone()} value={builder_store.sample.clone()} label="Sample" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedNameplate, false> builder={set_nameplate} optional={false} errors={builder_store.errors_nameplate.clone()} value={builder_store.nameplate.clone()} label="Nameplate" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedProject, true> builder={set_project} optional={false} errors={builder_store.errors_project.clone()} value={builder_store.project.clone()} label="Project" scanner={false} />
        </BasicForm<NewOrganism>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateOrganismFormProp {
    pub id: uuid::Uuid,
}

#[function_component(UpdateOrganismForm)]
pub fn update_organism_form(props: &UpdateOrganismFormProp) -> Html {
    let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = yewdux::use_store::<OrganismBuilder>();
    // We push the ID of the row to the named requests.
    let props = props.clone();
    named_requests.push(ComponentMessage::get::<NewOrganism>(props.id.into()));
    let set_notes =
        builder_dispatch.apply_callback(|notes: Option<String>| OrganismActions::SetNotes(notes));
    let set_wild =
        builder_dispatch.apply_callback(|wild: bool| OrganismActions::SetWild(Some(wild)));
    let set_host_organism = builder_dispatch.apply_callback(
        |host_organism: Option<Rc<web_common::database::nested_variants::NestedOrganism>>| {
            OrganismActions::SetHostOrganism(host_organism)
        },
    );
    let set_sample = builder_dispatch.apply_callback(
        |sample: Option<Rc<web_common::database::nested_variants::NestedSample>>| {
            OrganismActions::SetSample(sample)
        },
    );
    let set_nameplate = builder_dispatch.apply_callback(
        |nameplate: Option<Rc<web_common::database::nested_variants::NestedNameplate>>| {
            OrganismActions::SetNameplate(nameplate)
        },
    );
    let set_project = builder_dispatch.apply_callback(
        |project: Option<Rc<web_common::database::nested_variants::NestedProject>>| {
            OrganismActions::SetProject(project)
        },
    );
    html! {
        <BasicForm<NewOrganism>
            method={FormMethod::PUT}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Notes" optional={true} errors={builder_store.errors_notes.clone()} builder={set_notes} value={builder_store.notes.clone()} />
            <Checkbox label="Wild" errors={builder_store.errors_wild.clone()} builder={set_wild} value={builder_store.wild.unwrap_or(false)} />
            <Datalist<web_common::database::nested_variants::NestedOrganism, false> builder={set_host_organism} optional={true} errors={builder_store.errors_host_organism.clone()} value={builder_store.host_organism.clone()} label="Host organism" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedSample, false> builder={set_sample} optional={true} errors={builder_store.errors_sample.clone()} value={builder_store.sample.clone()} label="Sample" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedNameplate, false> builder={set_nameplate} optional={false} errors={builder_store.errors_nameplate.clone()} value={builder_store.nameplate.clone()} label="Nameplate" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedProject, true> builder={set_project} optional={false} errors={builder_store.errors_project.clone()} value={builder_store.project.clone()} label="Project" scanner={false} />
        </BasicForm<NewOrganism>>
    }
}
