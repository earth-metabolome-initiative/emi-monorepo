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
pub struct NameplateBuilder {
    pub id: Option<i32>,
    pub barcode: Option<Rc<String>>,
    pub geolocation: Option<web_common::types::Point>,
    pub project: Option<Rc<web_common::database::nested_variants::NestedProject>>,
    pub category: Option<Rc<web_common::database::nested_variants::NestedNameplateCategory>>,
    pub errors_barcode: Vec<ApiError>,
    pub errors_geolocation: Vec<ApiError>,
    pub errors_project: Vec<ApiError>,
    pub errors_category: Vec<ApiError>,
    pub form_updated_at: chrono::NaiveDateTime,
}

unsafe impl Send for NameplateBuilder {}
unsafe impl Sync for NameplateBuilder {}
impl Default for NameplateBuilder {
    fn default() -> Self {
        Self {
            id: None,
            barcode: None,
            geolocation: None,
            project: Default::default(),
            category: Default::default(),
            errors_barcode: Default::default(),
            errors_geolocation: Default::default(),
            errors_project: Default::default(),
            errors_category: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) enum NameplateActions {
    SetBarcode(Option<String>),
    SetGeolocation(Option<web_common::types::Point>),
    SetProject(Option<Rc<web_common::database::nested_variants::NestedProject>>),
    SetCategory(Option<Rc<web_common::database::nested_variants::NestedNameplateCategory>>),
}

impl FromOperation for NameplateActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "project" => NameplateActions::SetProject(Some(bincode::deserialize(&row).unwrap())),
            "category" => NameplateActions::SetCategory(Some(bincode::deserialize(&row).unwrap())),
            operation_name => {
                unreachable!("The operation name '{}' is not supported.", operation_name)
            }
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
                        "The Barcode field is required.".to_string(),
                    ]));
                    state_mut.barcode = None;
                    break 'barcode;
                }
                if let Some(value) = barcode.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_barcode.push(ApiError::BadRequest(vec![
                            "The Barcode field cannot be left empty.".to_string(),
                        ]));
                        state_mut.barcode = None;
                        break 'barcode;
                    }
                }
                state_mut.barcode = barcode.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'barcode;
            }
            NameplateActions::SetGeolocation(geolocation) => 'geolocation: {
                state_mut.errors_geolocation.clear();
                if geolocation.is_none() {
                    state_mut.errors_geolocation.push(ApiError::BadRequest(vec![
                        "The Geolocation field is required.".to_string(),
                    ]));
                    state_mut.geolocation = None;
                    break 'geolocation;
                }
                state_mut.geolocation = geolocation;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'geolocation;
            }
            NameplateActions::SetProject(project) => 'project: {
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
            NameplateActions::SetCategory(category) => 'category: {
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
impl FormBuilder for NameplateBuilder {
    type Actions = NameplateActions;

    type RichVariant = NestedNameplate;

    fn has_errors(&self) -> bool {
        !self.errors_barcode.is_empty()
            || !self.errors_geolocation.is_empty()
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
        dispatcher.apply(NameplateActions::SetBarcode(Some(
            richest_variant.inner.barcode.to_string(),
        )));
        dispatcher.apply(NameplateActions::SetGeolocation(Some(
            richest_variant.inner.geolocation,
        )));
        dispatcher.apply(NameplateActions::SetProject(
            Some(richest_variant.project).map(Rc::from),
        ));
        dispatcher.apply(NameplateActions::SetCategory(
            Some(richest_variant.category).map(Rc::from),
        ));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
            && self.barcode.is_some()
            && self.geolocation.is_some()
            && self.project.is_some()
            && self.category.is_some()
    }
}

impl From<NameplateBuilder> for NewNameplate {
    fn from(builder: NameplateBuilder) -> Self {
        Self {
            barcode: builder.barcode.as_deref().cloned().unwrap(),
            project_id: builder.project.as_deref().cloned().unwrap().inner.id,
            category_id: builder.category.as_deref().cloned().unwrap().inner.id,
            geolocation: builder.geolocation.unwrap(),
        }
    }
}
impl From<NameplateBuilder> for UpdateNameplate {
    fn from(builder: NameplateBuilder) -> Self {
        Self {
            id: builder.id.unwrap(),
            barcode: builder.barcode.as_deref().cloned().unwrap(),
            project_id: builder.project.as_deref().cloned().unwrap().inner.id,
            category_id: builder.category.as_deref().cloned().unwrap().inner.id,
            geolocation: builder.geolocation.unwrap(),
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
    let (builder_store, builder_dispatch) = yewdux::use_store::<NameplateBuilder>();
    if let Some(project_id) = props.project_id {
        named_requests.push(ComponentMessage::get_named::<&str, Project>(
            "project",
            project_id.into(),
        ));
    }
    named_requests.push(ComponentMessage::get_named::<&str, NameplateCategory>(
        "category",
        props.category_id.into(),
    ));
    let set_barcode = builder_dispatch
        .apply_callback(|barcode: Option<String>| NameplateActions::SetBarcode(barcode));
    let set_geolocation =
        builder_dispatch.apply_callback(|geolocation: Option<web_common::types::Point>| {
            NameplateActions::SetGeolocation(geolocation)
        });
    let set_project = builder_dispatch.apply_callback(
        |project: Option<Rc<web_common::database::nested_variants::NestedProject>>| {
            NameplateActions::SetProject(project)
        },
    );
    let set_category = builder_dispatch.apply_callback(
        |category: Option<Rc<web_common::database::nested_variants::NestedNameplateCategory>>| {
            NameplateActions::SetCategory(category)
        },
    );
    html! {
        <BasicForm<NewNameplate>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<BarCode> label="Barcode" optional={false} errors={builder_store.errors_barcode.clone()} builder={set_barcode} value={builder_store.barcode.as_deref().cloned().map(BarCode::from).map(Rc::from)} />
            <GPSInput label="Geolocation" optional={false} errors={builder_store.errors_geolocation.clone()} builder={set_geolocation} coordinates={builder_store.geolocation.clone()} />
            <Datalist<web_common::database::nested_variants::NestedProject, true> builder={set_project} optional={false} errors={builder_store.errors_project.clone()} value={builder_store.project.clone()} label="Project" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedNameplateCategory, false> builder={set_category} optional={false} errors={builder_store.errors_category.clone()} value={builder_store.category.clone()} label="Category" scanner={false} />
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
    let (builder_store, builder_dispatch) = yewdux::use_store::<NameplateBuilder>();
    // We push the ID of the row to the named requests.
    let props = props.clone();
    named_requests.push(ComponentMessage::get::<UpdateNameplate>(props.id.into()));
    let set_barcode = builder_dispatch
        .apply_callback(|barcode: Option<String>| NameplateActions::SetBarcode(barcode));
    let set_geolocation =
        builder_dispatch.apply_callback(|geolocation: Option<web_common::types::Point>| {
            NameplateActions::SetGeolocation(geolocation)
        });
    let set_project = builder_dispatch.apply_callback(
        |project: Option<Rc<web_common::database::nested_variants::NestedProject>>| {
            NameplateActions::SetProject(project)
        },
    );
    let set_category = builder_dispatch.apply_callback(
        |category: Option<Rc<web_common::database::nested_variants::NestedNameplateCategory>>| {
            NameplateActions::SetCategory(category)
        },
    );
    html! {
        <BasicForm<UpdateNameplate>
            method={FormMethod::PUT}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<BarCode> label="Barcode" optional={false} errors={builder_store.errors_barcode.clone()} builder={set_barcode} value={builder_store.barcode.as_deref().cloned().map(BarCode::from).map(Rc::from)} />
            <GPSInput label="Geolocation" optional={false} errors={builder_store.errors_geolocation.clone()} builder={set_geolocation} coordinates={builder_store.geolocation.clone()} />
            <Datalist<web_common::database::nested_variants::NestedProject, true> builder={set_project} optional={false} errors={builder_store.errors_project.clone()} value={builder_store.project.clone()} label="Project" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedNameplateCategory, false> builder={set_category} optional={false} errors={builder_store.errors_category.clone()} value={builder_store.category.clone()} label="Category" scanner={false} />
        </BasicForm<UpdateNameplate>>
    }
}
