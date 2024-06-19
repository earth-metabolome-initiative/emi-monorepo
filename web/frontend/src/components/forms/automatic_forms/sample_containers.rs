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
pub struct SampleContainerBuilder {
    pub id: Option<i32>,
    pub barcode: Option<Rc<String>>,
    pub project: Option<Rc<web_common::database::nested_variants::NestedProject>>,
    pub category: Option<Rc<web_common::database::nested_variants::NestedSampleContainerCategory>>,
    pub errors_barcode: Vec<ApiError>,
    pub errors_project: Vec<ApiError>,
    pub errors_category: Vec<ApiError>,
    pub form_updated_at: chrono::NaiveDateTime,
}

unsafe impl Send for SampleContainerBuilder {}
unsafe impl Sync for SampleContainerBuilder {}
impl Default for SampleContainerBuilder {
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

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) enum SampleContainerActions {
    SetBarcode(Option<String>),
    SetProject(Option<Rc<web_common::database::nested_variants::NestedProject>>),
    SetCategory(Option<Rc<web_common::database::nested_variants::NestedSampleContainerCategory>>),
}

impl FromOperation for SampleContainerActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "project" => {
                SampleContainerActions::SetProject(Some(bincode::deserialize(&row).unwrap()))
            }
            "category" => {
                SampleContainerActions::SetCategory(Some(bincode::deserialize(&row).unwrap()))
            }
            operation_name => {
                unreachable!("The operation name '{}' is not supported.", operation_name)
            }
        }
    }
}

impl Reducer<SampleContainerBuilder> for SampleContainerActions {
    fn apply(
        self,
        mut state: std::rc::Rc<SampleContainerBuilder>,
    ) -> std::rc::Rc<SampleContainerBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            SampleContainerActions::SetBarcode(barcode) => 'barcode: {
                state_mut.errors_barcode.clear();
                if barcode.is_none() {
                    state_mut.errors_barcode.push(ApiError::BadRequest(vec![
                        "The Barcode field is required.".to_string(),
                    ]));
                    state_mut.barcode = None;
                    break 'barcode;
                }
                if let Some(value) = barcode.as_ref() {
                    if value.is_empty() {
                        state_mut
                            .errors_barcode
                            .push(ApiError::Empty("Barcode".to_string()));
                        state_mut.barcode = None;
                        break 'barcode;
                    }
                }
                state_mut.barcode = barcode.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'barcode;
            }
            SampleContainerActions::SetProject(project) => 'project: {
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
            SampleContainerActions::SetCategory(category) => 'category: {
                state_mut.errors_category.clear();
                if category.is_none() {
                    state_mut.errors_category.push(ApiError::BadRequest(vec![
                        "The Category field is required.".to_string(),
                    ]));
                    state_mut.category = None;
                    break 'category;
                }
                state_mut.category = category.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'category;
            }
        }
        state
    }
}
impl FormBuilder for SampleContainerBuilder {
    type Actions = SampleContainerActions;

    type RichVariant = NestedSampleContainer;

    fn has_errors(&self) -> bool {
        !self.errors_barcode.is_empty()
            || !self.errors_project.is_empty()
            || !self.errors_category.is_empty()
    }

    fn update(
        dispatcher: &Dispatch<Self>,
        richest_variant: Self::RichVariant,
    ) -> Vec<ComponentMessage> {
        dispatcher.reduce_mut(|state| {
            state.id = Some(richest_variant.inner.id);
        });
        dispatcher.apply(SampleContainerActions::SetBarcode(Some(
            richest_variant.inner.barcode.to_string(),
        )));
        dispatcher.apply(SampleContainerActions::SetProject(
            Some(richest_variant.project).map(Rc::from),
        ));
        dispatcher.apply(SampleContainerActions::SetCategory(
            Some(richest_variant.category).map(Rc::from),
        ));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
            && self.barcode.is_some()
            && self.project.is_some()
            && self.category.is_some()
    }
}

impl From<SampleContainerBuilder> for NewSampleContainer {
    fn from(builder: SampleContainerBuilder) -> Self {
        Self {
            barcode: builder.barcode.as_deref().cloned().unwrap(),
            project_id: builder.project.as_deref().cloned().unwrap().inner.id,
            category_id: builder.category.as_deref().cloned().unwrap().inner.id,
        }
    }
}
impl From<SampleContainerBuilder> for UpdateSampleContainer {
    fn from(builder: SampleContainerBuilder) -> Self {
        Self {
            id: builder.id.unwrap(),
            barcode: builder.barcode.as_deref().cloned().unwrap(),
            project_id: builder.project.as_deref().cloned().unwrap().inner.id,
            category_id: builder.category.as_deref().cloned().unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewSampleContainer {
    type Builder = SampleContainerBuilder;
    fn title() -> &'static str {
        "Sample container"
    }
    fn task_target() -> &'static str {
        "Sample container"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

impl FormBuildable for UpdateSampleContainer {
    type Builder = SampleContainerBuilder;
    fn title() -> &'static str {
        "Sample container"
    }
    fn task_target() -> &'static str {
        "Sample container"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CreateSampleContainerFormProp {
    #[prop_or_default]
    pub project_id: Option<i32>,
    #[prop_or(1)]
    pub category_id: i32,
}

#[function_component(CreateSampleContainerForm)]
pub fn create_sample_container_form(props: &CreateSampleContainerFormProp) -> Html {
    let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = yewdux::use_store::<SampleContainerBuilder>();
    if let Some(project_id) = props.project_id {
        named_requests.push(ComponentMessage::get_named::<&str, Project>(
            "project",
            project_id.into(),
        ));
    }
    named_requests.push(
        ComponentMessage::get_named::<&str, SampleContainerCategory>(
            "category",
            props.category_id.into(),
        ),
    );
    let set_barcode = builder_dispatch
        .apply_callback(|barcode: Option<String>| SampleContainerActions::SetBarcode(barcode));
    let set_project = builder_dispatch.apply_callback(
        |project: Option<Rc<web_common::database::nested_variants::NestedProject>>| {
            SampleContainerActions::SetProject(project)
        },
    );
    let set_category = builder_dispatch.apply_callback(
        |category: Option<
            Rc<web_common::database::nested_variants::NestedSampleContainerCategory>,
        >| SampleContainerActions::SetCategory(category),
    );
    html! {
        <BasicForm<NewSampleContainer>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<BarCode> label="Barcode" optional={false} errors={builder_store.errors_barcode.clone()} builder={set_barcode} value={builder_store.barcode.as_deref().cloned().map(BarCode::from).map(Rc::from)} />
            <Datalist<web_common::database::nested_variants::NestedProject, true> builder={set_project} optional={false} errors={builder_store.errors_project.clone()} value={builder_store.project.clone()} label="Project" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedSampleContainerCategory, false> builder={set_category} optional={false} errors={builder_store.errors_category.clone()} value={builder_store.category.clone()} label="Category" scanner={false} />
        </BasicForm<NewSampleContainer>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateSampleContainerFormProp {
    pub id: i32,
}

#[function_component(UpdateSampleContainerForm)]
pub fn update_sample_container_form(props: &UpdateSampleContainerFormProp) -> Html {
    let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = yewdux::use_store::<SampleContainerBuilder>();
    // We push the ID of the row to the named requests.
    let props = props.clone();
    named_requests.push(ComponentMessage::get::<UpdateSampleContainer>(
        props.id.into(),
    ));
    let set_barcode = builder_dispatch
        .apply_callback(|barcode: Option<String>| SampleContainerActions::SetBarcode(barcode));
    let set_project = builder_dispatch.apply_callback(
        |project: Option<Rc<web_common::database::nested_variants::NestedProject>>| {
            SampleContainerActions::SetProject(project)
        },
    );
    let set_category = builder_dispatch.apply_callback(
        |category: Option<
            Rc<web_common::database::nested_variants::NestedSampleContainerCategory>,
        >| SampleContainerActions::SetCategory(category),
    );
    html! {
        <BasicForm<UpdateSampleContainer>
            method={FormMethod::PUT}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<BarCode> label="Barcode" optional={false} errors={builder_store.errors_barcode.clone()} builder={set_barcode} value={builder_store.barcode.as_deref().cloned().map(BarCode::from).map(Rc::from)} />
            <Datalist<web_common::database::nested_variants::NestedProject, true> builder={set_project} optional={false} errors={builder_store.errors_project.clone()} value={builder_store.project.clone()} label="Project" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedSampleContainerCategory, false> builder={set_category} optional={false} errors={builder_store.errors_category.clone()} value={builder_store.category.clone()} label="Category" scanner={false} />
        </BasicForm<UpdateSampleContainer>>
    }
}
