//! This module contains the forms for the frontend.
//!
//! This module is automatically generated. Do not write anything here.

use serde::{Deserialize, Serialize};
use web_common::database::*;
use yew::prelude::*;
use yewdux::{use_store, use_store_value, Reducer, Store};
use crate::components::forms::*;
use web_common::api::form_traits::FormMethod;
use std::rc::Rc;
use uuid::Uuid;
use std::ops::Deref;
use yewdux::Dispatch;
use chrono::NaiveDateTime;
use web_common::api::ApiError;
use crate::stores::user_state::UserState;
use crate::workers::ws_worker::ComponentMessage;
use web_common::custom_validators::Image;
use web_common::file_formats::GenericFileFormat;

#[derive(Store, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct DerivedSampleBuilder {
    pub parent_sample: Option<NestedSample>,
    pub child_sample: Option<NestedSample>,
    pub errors_parent_sample: Vec<ApiError>,
    pub errors_child_sample: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for DerivedSampleBuilder {
    fn default() -> Self {
        Self {
            parent_sample: Default::default(),
            child_sample: Default::default(),
            errors_parent_sample: Default::default(),
            errors_child_sample: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum DerivedSampleActions {
    SetParentSample(Option<NestedSample>),
    SetChildSample(Option<NestedSample>),
}

impl FromOperation for DerivedSampleActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "parent_sample" => DerivedSampleActions::SetParentSample(Some(bincode::deserialize(&row).unwrap())),
            "child_sample" => DerivedSampleActions::SetChildSample(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<DerivedSampleBuilder> for DerivedSampleActions {
    fn apply(self, mut state: std::rc::Rc<DerivedSampleBuilder>) -> std::rc::Rc<DerivedSampleBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            DerivedSampleActions::SetParentSample(parent_sample) => 'parent_sample: {
                state_mut.errors_parent_sample.clear();
        if parent_sample.is_none() {
            state_mut.errors_parent_sample.push(ApiError::BadRequest(vec![
                "The Parent sample field is required.".to_string()
             ]));
            state_mut.parent_sample = None;
             break 'parent_sample;
        }
                state_mut.parent_sample = parent_sample;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'parent_sample;
            }
            DerivedSampleActions::SetChildSample(child_sample) => 'child_sample: {
                state_mut.errors_child_sample.clear();
        if child_sample.is_none() {
            state_mut.errors_child_sample.push(ApiError::BadRequest(vec![
                "The Child sample field is required.".to_string()
             ]));
            state_mut.child_sample = None;
             break 'child_sample;
        }
                state_mut.child_sample = child_sample;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'child_sample;
            }
        }
        state
    }
}
impl FormBuilder for DerivedSampleBuilder {
    type Actions = DerivedSampleActions;

    type RichVariant = NestedDerivedSample;

    fn has_errors(&self) -> bool {
!self.errors_parent_sample.is_empty() || !self.errors_child_sample.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.apply(DerivedSampleActions::SetParentSample(Some(richest_variant.parent_sample)));
        dispatcher.apply(DerivedSampleActions::SetChildSample(Some(richest_variant.child_sample)));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.parent_sample.is_some()
        && self.child_sample.is_some()
    }

}

impl From<DerivedSampleBuilder> for NewDerivedSample {
    fn from(builder: DerivedSampleBuilder) -> Self {
        Self {
            parent_sample_id: builder.parent_sample.as_ref().map(|parent_sample| parent_sample.inner.id).unwrap(),
            child_sample_id: builder.child_sample.as_ref().map(|child_sample| child_sample.inner.id).unwrap(),
        }
    }
}
impl From<DerivedSampleBuilder> for UpdateDerivedSample {
    fn from(builder: DerivedSampleBuilder) -> Self {
        Self {
            parent_sample_id: builder.parent_sample.as_ref().map(|parent_sample| parent_sample.inner.id).unwrap(),
            child_sample_id: builder.child_sample.as_ref().map(|child_sample| child_sample.inner.id).unwrap(),
        }
    }
}
impl FormBuildable for NewDerivedSample {
    type Builder = DerivedSampleBuilder;
    fn title() -> &'static str {
        "Derived sample"
    }
    fn task_target() -> &'static str {
        "Derived sample"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

impl FormBuildable for UpdateDerivedSample {
    type Builder = DerivedSampleBuilder;
    fn title() -> &'static str {
        "Derived sample"
    }
    fn task_target() -> &'static str {
        "Derived sample"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateDerivedSampleFormProp {
     #[prop_or_default]
    pub parent_sample_id: Option<Uuid>,
     #[prop_or_default]
    pub child_sample_id: Option<Uuid>,
}

#[function_component(CreateDerivedSampleForm)]
pub fn create_derived_sample_form(props: &CreateDerivedSampleFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<DerivedSampleBuilder>();
   if let Some(parent_sample_id) = props.parent_sample_id {
         named_requests.push(ComponentMessage::get_named::<&str, Sample>("parent_sample", parent_sample_id.into()));
    }
   if let Some(child_sample_id) = props.child_sample_id {
         named_requests.push(ComponentMessage::get_named::<&str, Sample>("child_sample", child_sample_id.into()));
    }
    let set_parent_sample = builder_dispatch.apply_callback(|parent_sample: Option<NestedSample>| DerivedSampleActions::SetParentSample(parent_sample));
    let set_child_sample = builder_dispatch.apply_callback(|child_sample: Option<NestedSample>| DerivedSampleActions::SetChildSample(child_sample));
    html! {
        <BasicForm<NewDerivedSample>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Datalist<NestedSample, false> builder={set_parent_sample} optional={false} errors={builder_store.errors_parent_sample.clone()} value={builder_store.parent_sample.clone()} label="Parent sample" scanner={false} />
            <Datalist<NestedSample, false> builder={set_child_sample} optional={false} errors={builder_store.errors_child_sample.clone()} value={builder_store.child_sample.clone()} label="Child sample" scanner={false} />
        </BasicForm<NewDerivedSample>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateDerivedSampleFormProp {
    pub parent_sample_id: Uuid,
    pub child_sample_id: Uuid,
}

#[function_component(UpdateDerivedSampleForm)]
pub fn update_derived_sample_form(props: &UpdateDerivedSampleFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<DerivedSampleBuilder>();
    // We push the ID of the row to the named requests.
    let props = props.clone();
   named_requests.push(ComponentMessage::get::<UpdateDerivedSample>(( props.parent_sample_id, props.child_sample_id ).into()));
    let set_parent_sample = builder_dispatch.apply_callback(|parent_sample: Option<NestedSample>| DerivedSampleActions::SetParentSample(parent_sample));
    let set_child_sample = builder_dispatch.apply_callback(|child_sample: Option<NestedSample>| DerivedSampleActions::SetChildSample(child_sample));
    html! {
        <BasicForm<UpdateDerivedSample>
            method={FormMethod::PUT}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Datalist<NestedSample, false> builder={set_parent_sample} optional={false} errors={builder_store.errors_parent_sample.clone()} value={builder_store.parent_sample.clone()} label="Parent sample" scanner={false} />
            <Datalist<NestedSample, false> builder={set_child_sample} optional={false} errors={builder_store.errors_child_sample.clone()} value={builder_store.child_sample.clone()} label="Child sample" scanner={false} />
        </BasicForm<UpdateDerivedSample>>
    }
}
#[derive(Store, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct NameplateBuilder {
    pub id: Option<i32>,
    pub barcode: Option<String>,
    pub project: Option<NestedProject>,
    pub category: Option<NestedNameplateCategory>,
    pub errors_barcode: Vec<ApiError>,
    pub errors_project: Vec<ApiError>,
    pub errors_category: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for NameplateBuilder {
    fn default() -> Self {
        Self {
            id: None,
            barcode: None,
            project: Default::default(),
            category: Default::default(),
            errors_barcode: Default::default(),
            errors_project: Default::default(),
            errors_category: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum NameplateActions {
    SetBarcode(Option<String>),
    SetProject(Option<NestedProject>),
    SetCategory(Option<NestedNameplateCategory>),
}

impl FromOperation for NameplateActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "project" => NameplateActions::SetProject(Some(bincode::deserialize(&row).unwrap())),
            "category" => NameplateActions::SetCategory(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<NameplateBuilder> for NameplateActions {
    fn apply(self, mut state: std::rc::Rc<NameplateBuilder>) -> std::rc::Rc<NameplateBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NameplateActions::SetBarcode(barcode) => 'barcode: {
                state_mut.errors_barcode.clear();
        if barcode.is_none() {
            state_mut.errors_barcode.push(ApiError::BadRequest(vec![
                "The Barcode field is required.".to_string()
             ]));
            state_mut.barcode = None;
             break 'barcode;
        }
                if let Some(value) = barcode.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_barcode.push(ApiError::BadRequest(vec![
                            "The Barcode field cannot be left empty.".to_string()
                        ]));
                         state_mut.barcode = None;
                          break 'barcode;
                    }
                }
                state_mut.barcode = barcode;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'barcode;
            }
            NameplateActions::SetProject(project) => 'project: {
                state_mut.errors_project.clear();
        if project.is_none() {
            state_mut.errors_project.push(ApiError::BadRequest(vec![
                "The Project field is required.".to_string()
             ]));
            state_mut.project = None;
             break 'project;
        }
                state_mut.project = project;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'project;
            }
            NameplateActions::SetCategory(category) => 'category: {
                state_mut.errors_category.clear();
        if category.is_none() {
            state_mut.errors_category.push(ApiError::BadRequest(vec![
                "The Category field is required.".to_string()
             ]));
            state_mut.category = None;
             break 'category;
        }
                state_mut.category = category;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'category;
            }
        }
        state
    }
}
impl FormBuilder for NameplateBuilder {
    type Actions = NameplateActions;

    type RichVariant = NestedNameplate;

    fn has_errors(&self) -> bool {
!self.errors_barcode.is_empty() || !self.errors_project.is_empty() || !self.errors_category.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.reduce_mut(|state| {state.id = Some(richest_variant.inner.id);});
    dispatcher.apply(NameplateActions::SetBarcode(Some(richest_variant.inner.barcode.to_string())));
        dispatcher.apply(NameplateActions::SetProject(Some(richest_variant.project)));
        dispatcher.apply(NameplateActions::SetCategory(Some(richest_variant.category)));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.barcode.is_some()
        && self.project.is_some()
        && self.category.is_some()
    }

}

impl From<NameplateBuilder> for NewNameplate {
    fn from(builder: NameplateBuilder) -> Self {
        Self {
            barcode: builder.barcode.unwrap(),
            project_id: builder.project.unwrap().inner.id,
            category_id: builder.category.unwrap().inner.id,
        }
    }
}
impl From<NameplateBuilder> for UpdateNameplate {
    fn from(builder: NameplateBuilder) -> Self {
        Self {
            id: builder.id.unwrap(),
            barcode: builder.barcode.unwrap(),
            project_id: builder.project.unwrap().inner.id,
            category_id: builder.category.unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewNameplate {
    type Builder = NameplateBuilder;
    fn title() -> &'static str {
        "Nameplate"
    }
    fn task_target() -> &'static str {
        "Nameplate"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

impl FormBuildable for UpdateNameplate {
    type Builder = NameplateBuilder;
    fn title() -> &'static str {
        "Nameplate"
    }
    fn task_target() -> &'static str {
        "Nameplate"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateNameplateFormProp {
     #[prop_or_default]
    pub project_id: Option<i32>,
     #[prop_or(1)]
     pub category_id: i32,
}

#[function_component(CreateNameplateForm)]
pub fn create_nameplate_form(props: &CreateNameplateFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<NameplateBuilder>();
   if let Some(project_id) = props.project_id {
         named_requests.push(ComponentMessage::get_named::<&str, Project>("project", project_id.into()));
    }
    named_requests.push(ComponentMessage::get_named::<&str, NameplateCategory>("category", props.category_id.into()));
    let set_barcode = builder_dispatch.apply_callback(|barcode: Option<String>| NameplateActions::SetBarcode(barcode));
    let set_project = builder_dispatch.apply_callback(|project: Option<NestedProject>| NameplateActions::SetProject(project));
    let set_category = builder_dispatch.apply_callback(|category: Option<NestedNameplateCategory>| NameplateActions::SetCategory(category));
    html! {
        <BasicForm<NewNameplate>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<BarCode> label="Barcode" optional={false} errors={builder_store.errors_barcode.clone()} builder={set_barcode} value={builder_store.barcode.clone().map(BarCode::from)} />
            <Datalist<NestedProject, true> builder={set_project} optional={false} errors={builder_store.errors_project.clone()} value={builder_store.project.clone()} label="Project" scanner={false} />
            <Datalist<NestedNameplateCategory, false> builder={set_category} optional={false} errors={builder_store.errors_category.clone()} value={builder_store.category.clone()} label="Category" scanner={false} />
        </BasicForm<NewNameplate>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateNameplateFormProp {
    pub id: i32,
}

#[function_component(UpdateNameplateForm)]
pub fn update_nameplate_form(props: &UpdateNameplateFormProp) -> Html {
     let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = use_store::<NameplateBuilder>();
    // We push the ID of the row to the named requests.
    let props = props.clone();
   named_requests.push(ComponentMessage::get::<UpdateNameplate>(props.id.into()));
    let set_barcode = builder_dispatch.apply_callback(|barcode: Option<String>| NameplateActions::SetBarcode(barcode));
    let set_project = builder_dispatch.apply_callback(|project: Option<NestedProject>| NameplateActions::SetProject(project));
    let set_category = builder_dispatch.apply_callback(|category: Option<NestedNameplateCategory>| NameplateActions::SetCategory(category));
    html! {
        <BasicForm<UpdateNameplate>
            method={FormMethod::PUT}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<BarCode> label="Barcode" optional={false} errors={builder_store.errors_barcode.clone()} builder={set_barcode} value={builder_store.barcode.clone().map(BarCode::from)} />
            <Datalist<NestedProject, true> builder={set_project} optional={false} errors={builder_store.errors_project.clone()} value={builder_store.project.clone()} label="Project" scanner={false} />
            <Datalist<NestedNameplateCategory, false> builder={set_category} optional={false} errors={builder_store.errors_category.clone()} value={builder_store.category.clone()} label="Category" scanner={false} />
        </BasicForm<UpdateNameplate>>
    }
}
#[derive(Store, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct ObservationBuilder {
    pub id: Option<Uuid>,
    pub notes: Option<String>,
    pub picture: Option<Vec<u8>>,
    pub parent_observation: Option<NestedObservation>,
    pub project: Option<NestedProject>,
    pub organism: Option<NestedOrganism>,
    pub sample: Option<NestedSample>,
    pub errors_notes: Vec<ApiError>,
    pub errors_picture: Vec<ApiError>,
    pub errors_parent_observation: Vec<ApiError>,
    pub errors_project: Vec<ApiError>,
    pub errors_organism: Vec<ApiError>,
    pub errors_sample: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

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
            errors_notes: Default::default(),
            errors_picture: Default::default(),
            errors_parent_observation: Default::default(),
            errors_project: Default::default(),
            errors_organism: Default::default(),
            errors_sample: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum ObservationActions {
    SetNotes(Option<String>),
    SetPicture(Option<Vec<u8>>),
    SetParentObservation(Option<NestedObservation>),
    SetProject(Option<NestedProject>),
    SetOrganism(Option<NestedOrganism>),
    SetSample(Option<NestedSample>),
}

impl FromOperation for ObservationActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "parent_observation" => ObservationActions::SetParentObservation(Some(bincode::deserialize(&row).unwrap())),
            "project" => ObservationActions::SetProject(Some(bincode::deserialize(&row).unwrap())),
            "organism" => ObservationActions::SetOrganism(Some(bincode::deserialize(&row).unwrap())),
            "sample" => ObservationActions::SetSample(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
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
                            "The Notes field cannot be left empty.".to_string()
                        ]));
                         state_mut.notes = None;
                          break 'notes;
                    }
                }
                state_mut.notes = notes;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'notes;
            }
            ObservationActions::SetPicture(picture) => 'picture: {
                state_mut.errors_picture.clear();
        if picture.is_none() {
            state_mut.errors_picture.push(ApiError::BadRequest(vec![
                "The Picture field is required.".to_string()
             ]));
            state_mut.picture = None;
             break 'picture;
        }
                state_mut.picture = picture;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'picture;
            }
            ObservationActions::SetParentObservation(parent_observation) => 'parent_observation: {
                state_mut.errors_parent_observation.clear();
                match parent_observation.as_ref() {
                    Some(parent_observation) => {
                            if state_mut.id.map_or(false, |id| id == parent_observation.inner.id)
                        {
                            state_mut.errors_parent_observation.push(ApiError::BadRequest(vec![
                                "The Parent observation field must be distinct from the current value.".to_string()
                             ]));
                            break 'parent_observation;
                        }
                    }
                    None => (),
                }
                state_mut.parent_observation = parent_observation;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'parent_observation;
            }
            ObservationActions::SetProject(project) => 'project: {
                state_mut.errors_project.clear();
        if project.is_none() {
            state_mut.errors_project.push(ApiError::BadRequest(vec![
                "The Project field is required.".to_string()
             ]));
            state_mut.project = None;
             break 'project;
        }
                state_mut.project = project;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'project;
            }
            ObservationActions::SetOrganism(organism) => 'organism: {
                state_mut.errors_organism.clear();
                state_mut.organism = organism;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'organism;
            }
            ObservationActions::SetSample(sample) => 'sample: {
                state_mut.errors_sample.clear();
                state_mut.sample = sample;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'sample;
            }
        }
        state
    }
}
impl FormBuilder for ObservationBuilder {
    type Actions = ObservationActions;

    type RichVariant = NestedObservation;

    fn has_errors(&self) -> bool {
!self.errors_notes.is_empty() || !self.errors_picture.is_empty() || !self.errors_parent_observation.is_empty() || !self.errors_project.is_empty() || !self.errors_organism.is_empty() || !self.errors_sample.is_empty()
    }

    fn update(dispatcher: &Dispatch<Self>, richest_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.reduce_mut(|state| {state.id = Some(richest_variant.inner.id);});
    dispatcher.apply(ObservationActions::SetNotes(richest_variant.inner.notes.map(|notes| notes.to_string())));
        dispatcher.apply(ObservationActions::SetPicture(Some(richest_variant.inner.picture)));        dispatcher.apply(ObservationActions::SetProject(Some(richest_variant.project)));
        dispatcher.apply(ObservationActions::SetOrganism(richest_variant.organism));
        dispatcher.apply(ObservationActions::SetSample(richest_variant.sample));
        let mut named_requests = Vec::new();
        if let Some(parent_observation_id) = richest_variant.inner.parent_observation_id {
    named_requests.push(ComponentMessage::get_named::<&str, Observation>("parent_observation", parent_observation_id.into()));
 } else {
    dispatcher.apply(ObservationActions::SetParentObservation(None));
 }
        named_requests
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.picture.is_some()
        && self.project.is_some()
    }

}

impl From<ObservationBuilder> for NewObservation {
    fn from(builder: ObservationBuilder) -> Self {
        Self {
            id: builder.id.unwrap_or_else(Uuid::new_v4),
            parent_observation_id: builder.parent_observation.map(|parent_observation| parent_observation.inner.id),
            project_id: builder.project.unwrap().inner.id,
            organism_id: builder.organism.map(|organism| organism.inner.id),
            sample_id: builder.sample.map(|sample| sample.inner.id),
            notes: builder.notes,
            picture: builder.picture.unwrap(),
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
