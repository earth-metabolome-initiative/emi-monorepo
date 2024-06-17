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
pub struct SpectraCollectionBuilder {
    pub id: Option<i32>,
    pub notes: Option<Rc<String>>,
    pub sample: Option<Rc<web_common::database::nested_variants::NestedSample>>,
    pub errors_notes: Vec<ApiError>,
    pub errors_sample: Vec<ApiError>,
    pub form_updated_at: chrono::NaiveDateTime,
}

unsafe impl Send for SpectraCollectionBuilder {}
unsafe impl Sync for SpectraCollectionBuilder {}
impl Default for SpectraCollectionBuilder {
    fn default() -> Self {
        Self {
            id: None,
            notes: None,
            sample: Default::default(),
            errors_notes: Default::default(),
            errors_sample: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) enum SpectraCollectionActions {
    SetNotes(Option<String>),
    SetSample(Option<Rc<web_common::database::nested_variants::NestedSample>>),
}

impl FromOperation for SpectraCollectionActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "sample" => {
                SpectraCollectionActions::SetSample(Some(bincode::deserialize(&row).unwrap()))
            }
            operation_name => {
                unreachable!("The operation name '{}' is not supported.", operation_name)
            }
        }
    }
}

impl Reducer<SpectraCollectionBuilder> for SpectraCollectionActions {
    fn apply(
        self,
        mut state: std::rc::Rc<SpectraCollectionBuilder>,
    ) -> std::rc::Rc<SpectraCollectionBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            SpectraCollectionActions::SetNotes(notes) => 'notes: {
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
            SpectraCollectionActions::SetSample(sample) => 'sample: {
                state_mut.errors_sample.clear();
                if sample.is_none() {
                    state_mut.errors_sample.push(ApiError::BadRequest(vec![
                        "The Sample field is required.".to_string(),
                    ]));
                    state_mut.sample = None;
                    break 'sample;
                }
                state_mut.sample = sample.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'sample;
            }
        }
        state
    }
}
impl FormBuilder for SpectraCollectionBuilder {
    type Actions = SpectraCollectionActions;

    type RichVariant = NestedSpectraCollection;

    fn has_errors(&self) -> bool {
        !self.errors_notes.is_empty() || !self.errors_sample.is_empty()
    }

    fn update(
        dispatcher: &Dispatch<Self>,
        richest_variant: Self::RichVariant,
    ) -> Vec<ComponentMessage> {
        dispatcher.reduce_mut(|state| {
            state.id = Some(richest_variant.inner.id);
        });
        dispatcher.apply(SpectraCollectionActions::SetNotes(
            richest_variant
                .inner
                .notes
                .as_ref()
                .map(|notes| notes.to_string()),
        ));
        dispatcher.apply(SpectraCollectionActions::SetSample(
            Some(richest_variant.sample).map(Rc::from),
        ));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors() && self.sample.is_some()
    }
}

impl From<SpectraCollectionBuilder> for NewSpectraCollection {
    fn from(builder: SpectraCollectionBuilder) -> Self {
        Self {
            notes: builder.notes.as_deref().cloned(),
            sample_id: builder.sample.as_deref().cloned().unwrap().inner.id,
        }
    }
}
impl From<SpectraCollectionBuilder> for UpdateSpectraCollection {
    fn from(builder: SpectraCollectionBuilder) -> Self {
        Self {
            id: builder.id.unwrap(),
            notes: builder.notes.as_deref().cloned(),
            sample_id: builder.sample.as_deref().cloned().unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewSpectraCollection {
    type Builder = SpectraCollectionBuilder;
    fn title() -> &'static str {
        "Spectra collection"
    }
    fn task_target() -> &'static str {
        "Spectra collection"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

impl FormBuildable for UpdateSpectraCollection {
    type Builder = SpectraCollectionBuilder;
    fn title() -> &'static str {
        "Spectra collection"
    }
    fn task_target() -> &'static str {
        "Spectra collection"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateSpectraCollectionFormProp {
    #[prop_or_default]
    pub sample_id: Option<uuid::Uuid>,
}

#[function_component(CreateSpectraCollectionForm)]
pub fn create_spectra_collection_form(props: &CreateSpectraCollectionFormProp) -> Html {
    let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = yewdux::use_store::<SpectraCollectionBuilder>();
    if let Some(sample_id) = props.sample_id {
        named_requests.push(ComponentMessage::get_named::<&str, Sample>(
            "sample",
            sample_id.into(),
        ));
    }
    let set_notes = builder_dispatch
        .apply_callback(|notes: Option<String>| SpectraCollectionActions::SetNotes(notes));
    let set_sample = builder_dispatch.apply_callback(
        |sample: Option<Rc<web_common::database::nested_variants::NestedSample>>| {
            SpectraCollectionActions::SetSample(sample)
        },
    );
    html! {
        <BasicForm<NewSpectraCollection>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Notes" optional={true} errors={builder_store.errors_notes.clone()} builder={set_notes} value={builder_store.notes.clone()} />
            <Datalist<web_common::database::nested_variants::NestedSample, false> builder={set_sample} optional={false} errors={builder_store.errors_sample.clone()} value={builder_store.sample.clone()} label="Sample" scanner={false} />
        </BasicForm<NewSpectraCollection>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateSpectraCollectionFormProp {
    pub id: i32,
}

#[function_component(UpdateSpectraCollectionForm)]
pub fn update_spectra_collection_form(props: &UpdateSpectraCollectionFormProp) -> Html {
    let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = yewdux::use_store::<SpectraCollectionBuilder>();
    // We push the ID of the row to the named requests.
    let props = props.clone();
    named_requests.push(ComponentMessage::get::<UpdateSpectraCollection>(
        props.id.into(),
    ));
    let set_notes = builder_dispatch
        .apply_callback(|notes: Option<String>| SpectraCollectionActions::SetNotes(notes));
    let set_sample = builder_dispatch.apply_callback(
        |sample: Option<Rc<web_common::database::nested_variants::NestedSample>>| {
            SpectraCollectionActions::SetSample(sample)
        },
    );
    html! {
        <BasicForm<UpdateSpectraCollection>
            method={FormMethod::PUT}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Notes" optional={true} errors={builder_store.errors_notes.clone()} builder={set_notes} value={builder_store.notes.clone()} />
            <Datalist<web_common::database::nested_variants::NestedSample, false> builder={set_sample} optional={false} errors={builder_store.errors_sample.clone()} value={builder_store.sample.clone()} label="Sample" scanner={false} />
        </BasicForm<UpdateSpectraCollection>>
    }
}
