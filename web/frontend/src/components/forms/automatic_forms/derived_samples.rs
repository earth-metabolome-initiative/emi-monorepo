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
pub struct DerivedSampleBuilder {
    pub quantity: Option<f64>,
    pub parent_sample: Option<Rc<web_common::database::nested_variants::NestedSample>>,
    pub child_sample: Option<Rc<web_common::database::nested_variants::NestedSample>>,
    pub unit: Option<Rc<web_common::database::nested_variants::NestedUnit>>,
    pub errors_quantity: Vec<ApiError>,
    pub errors_parent_sample: Vec<ApiError>,
    pub errors_child_sample: Vec<ApiError>,
    pub errors_unit: Vec<ApiError>,
    pub form_updated_at: chrono::NaiveDateTime,
}

unsafe impl Send for DerivedSampleBuilder {}
unsafe impl Sync for DerivedSampleBuilder {}
impl Default for DerivedSampleBuilder {
    fn default() -> Self {
        Self {
            quantity: None,
            parent_sample: Default::default(),
            child_sample: Default::default(),
            unit: Default::default(),
            errors_quantity: Default::default(),
            errors_parent_sample: Default::default(),
            errors_child_sample: Default::default(),
            errors_unit: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) enum DerivedSampleActions {
    SetQuantity(Option<String>),
    SetParentSample(Option<Rc<web_common::database::nested_variants::NestedSample>>),
    SetChildSample(Option<Rc<web_common::database::nested_variants::NestedSample>>),
    SetUnit(Option<Rc<web_common::database::nested_variants::NestedUnit>>),
}

impl FromOperation for DerivedSampleActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "parent_sample" => {
                DerivedSampleActions::SetParentSample(Some(bincode::deserialize(&row).unwrap()))
            }
            "child_sample" => {
                DerivedSampleActions::SetChildSample(Some(bincode::deserialize(&row).unwrap()))
            }
            "unit" => DerivedSampleActions::SetUnit(Some(bincode::deserialize(&row).unwrap())),
            operation_name => {
                unreachable!("The operation name '{}' is not supported.", operation_name)
            }
        }
    }
}

impl Reducer<DerivedSampleBuilder> for DerivedSampleActions {
    fn apply(
        self,
        mut state: std::rc::Rc<DerivedSampleBuilder>,
    ) -> std::rc::Rc<DerivedSampleBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            DerivedSampleActions::SetQuantity(quantity) => 'quantity: {
                state_mut.errors_quantity.clear();
                if quantity.is_none() {
                    state_mut.errors_quantity.push(ApiError::BadRequest(vec![
                        "The Quantity field is required.".to_string(),
                    ]));
                    state_mut.quantity = None;
                    break 'quantity;
                }
                state_mut.form_updated_at = chrono::Utc::now().naive_utc();
                match quantity {
                    Some(value) => match value.parse::<f64>() {
                        Ok(value) => {
                            if value.is_nan() || value.is_infinite() {
                                state_mut.errors_quantity.push(ApiError::BadRequest(vec![
                                    "The quantity field must be a valid f64.".to_string(),
                                ]));
                            } else if value < f64::MIN as f64 || value > f64::MAX as f64 {
                                state_mut.errors_quantity.push(ApiError::BadRequest(vec![
                                    format!(
                                        "The quantity field must be between {} and {}.",
                                        f64::MIN,
                                        f64::MAX
                                    ),
                                ]));
                            } else {
                                state_mut.quantity = Some(value as f64);
                            }
                        }
                        Err(_) => {
                            state_mut.errors_quantity.push(ApiError::BadRequest(vec![
                                "The quantity field must be a valid f64.".to_string(),
                            ]));
                        }
                    },
                    None => state_mut.quantity = None,
                }
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'quantity;
            }
            DerivedSampleActions::SetParentSample(parent_sample) => 'parent_sample: {
                state_mut.errors_parent_sample.clear();
                if parent_sample.is_none() {
                    state_mut
                        .errors_parent_sample
                        .push(ApiError::BadRequest(vec![
                            "The Parent sample field is required.".to_string(),
                        ]));
                    state_mut.parent_sample = None;
                    break 'parent_sample;
                }
                state_mut.parent_sample = parent_sample.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'parent_sample;
            }
            DerivedSampleActions::SetChildSample(child_sample) => 'child_sample: {
                state_mut.errors_child_sample.clear();
                if child_sample.is_none() {
                    state_mut
                        .errors_child_sample
                        .push(ApiError::BadRequest(vec![
                            "The Child sample field is required.".to_string(),
                        ]));
                    state_mut.child_sample = None;
                    break 'child_sample;
                }
                state_mut.child_sample = child_sample.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'child_sample;
            }
            DerivedSampleActions::SetUnit(unit) => 'unit: {
                state_mut.errors_unit.clear();
                if unit.is_none() {
                    state_mut.errors_unit.push(ApiError::BadRequest(vec![
                        "The Unit field is required.".to_string(),
                    ]));
                    state_mut.unit = None;
                    break 'unit;
                }
                state_mut.unit = unit.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'unit;
            }
        }
        state
    }
}
impl FormBuilder for DerivedSampleBuilder {
    type Actions = DerivedSampleActions;

    type RichVariant = NestedDerivedSample;

    fn has_errors(&self) -> bool {
        !self.errors_quantity.is_empty()
            || !self.errors_parent_sample.is_empty()
            || !self.errors_child_sample.is_empty()
            || !self.errors_unit.is_empty()
    }

    fn update(
        dispatcher: &Dispatch<Self>,
        richest_variant: Self::RichVariant,
    ) -> Vec<ComponentMessage> {
        dispatcher.apply(DerivedSampleActions::SetQuantity(Some(
            richest_variant.inner.quantity.to_string(),
        )));
        dispatcher.apply(DerivedSampleActions::SetParentSample(
            Some(richest_variant.parent_sample).map(Rc::from),
        ));
        dispatcher.apply(DerivedSampleActions::SetChildSample(
            Some(richest_variant.child_sample).map(Rc::from),
        ));
        dispatcher.apply(DerivedSampleActions::SetUnit(
            Some(richest_variant.unit).map(Rc::from),
        ));
        vec![]
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
            && self.quantity.is_some()
            && self.parent_sample.is_some()
            && self.child_sample.is_some()
            && self.unit.is_some()
    }
}

impl From<DerivedSampleBuilder> for NewDerivedSample {
    fn from(builder: DerivedSampleBuilder) -> Self {
        Self {
            parent_sample_id: builder
                .parent_sample
                .as_ref()
                .map(|parent_sample| parent_sample.inner.id)
                .unwrap(),
            child_sample_id: builder
                .child_sample
                .as_ref()
                .map(|child_sample| child_sample.inner.id)
                .unwrap(),
            quantity: builder.quantity.unwrap(),
            unit_id: builder.unit.as_deref().cloned().unwrap().inner.id,
        }
    }
}
impl From<DerivedSampleBuilder> for UpdateDerivedSample {
    fn from(builder: DerivedSampleBuilder) -> Self {
        Self {
            parent_sample_id: builder
                .parent_sample
                .as_ref()
                .map(|parent_sample| parent_sample.inner.id)
                .unwrap(),
            child_sample_id: builder
                .child_sample
                .as_ref()
                .map(|child_sample| child_sample.inner.id)
                .unwrap(),
            quantity: builder.quantity.unwrap(),
            unit_id: builder.unit.as_deref().cloned().unwrap().inner.id,
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
    pub parent_sample_id: Option<uuid::Uuid>,
    #[prop_or_default]
    pub child_sample_id: Option<uuid::Uuid>,
    #[prop_or_default]
    pub unit_id: Option<i32>,
}

#[function_component(CreateDerivedSampleForm)]
pub fn create_derived_sample_form(props: &CreateDerivedSampleFormProp) -> Html {
    let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = yewdux::use_store::<DerivedSampleBuilder>();
    if let Some(parent_sample_id) = props.parent_sample_id {
        named_requests.push(ComponentMessage::get_named::<&str, Sample>(
            "parent_sample",
            parent_sample_id.into(),
        ));
    }
    if let Some(child_sample_id) = props.child_sample_id {
        named_requests.push(ComponentMessage::get_named::<&str, Sample>(
            "child_sample",
            child_sample_id.into(),
        ));
    }
    if let Some(unit_id) = props.unit_id {
        named_requests.push(ComponentMessage::get_named::<&str, Unit>(
            "unit",
            unit_id.into(),
        ));
    }
    let set_quantity = builder_dispatch
        .apply_callback(|quantity: Option<String>| DerivedSampleActions::SetQuantity(quantity));
    let set_parent_sample = builder_dispatch.apply_callback(
        |parent_sample: Option<Rc<web_common::database::nested_variants::NestedSample>>| {
            DerivedSampleActions::SetParentSample(parent_sample)
        },
    );
    let set_child_sample = builder_dispatch.apply_callback(
        |child_sample: Option<Rc<web_common::database::nested_variants::NestedSample>>| {
            DerivedSampleActions::SetChildSample(child_sample)
        },
    );
    let set_unit = builder_dispatch.apply_callback(
        |unit: Option<Rc<web_common::database::nested_variants::NestedUnit>>| {
            DerivedSampleActions::SetUnit(unit)
        },
    );
    html! {
        <BasicForm<NewDerivedSample>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<f64> label="Quantity" optional={false} errors={builder_store.errors_quantity.clone()} builder={set_quantity} value={builder_store.quantity.clone().map(Rc::from)} />
            <Datalist<web_common::database::nested_variants::NestedSample, false> builder={set_parent_sample} optional={false} errors={builder_store.errors_parent_sample.clone()} value={builder_store.parent_sample.clone()} label="Parent sample" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedSample, false> builder={set_child_sample} optional={false} errors={builder_store.errors_child_sample.clone()} value={builder_store.child_sample.clone()} label="Child sample" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedUnit, false> builder={set_unit} optional={false} errors={builder_store.errors_unit.clone()} value={builder_store.unit.clone()} label="Unit" scanner={false} />
        </BasicForm<NewDerivedSample>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateDerivedSampleFormProp {
    pub parent_sample_id: uuid::Uuid,
    pub child_sample_id: uuid::Uuid,
}

#[function_component(UpdateDerivedSampleForm)]
pub fn update_derived_sample_form(props: &UpdateDerivedSampleFormProp) -> Html {
    let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = yewdux::use_store::<DerivedSampleBuilder>();
    // We push the ID of the row to the named requests.
    let props = props.clone();
    named_requests.push(ComponentMessage::get::<UpdateDerivedSample>(
        (props.parent_sample_id, props.child_sample_id).into(),
    ));
    let set_quantity = builder_dispatch
        .apply_callback(|quantity: Option<String>| DerivedSampleActions::SetQuantity(quantity));
    let set_parent_sample = builder_dispatch.apply_callback(
        |parent_sample: Option<Rc<web_common::database::nested_variants::NestedSample>>| {
            DerivedSampleActions::SetParentSample(parent_sample)
        },
    );
    let set_child_sample = builder_dispatch.apply_callback(
        |child_sample: Option<Rc<web_common::database::nested_variants::NestedSample>>| {
            DerivedSampleActions::SetChildSample(child_sample)
        },
    );
    let set_unit = builder_dispatch.apply_callback(
        |unit: Option<Rc<web_common::database::nested_variants::NestedUnit>>| {
            DerivedSampleActions::SetUnit(unit)
        },
    );
    html! {
        <BasicForm<UpdateDerivedSample>
            method={FormMethod::PUT}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<f64> label="Quantity" optional={false} errors={builder_store.errors_quantity.clone()} builder={set_quantity} value={builder_store.quantity.clone().map(Rc::from)} />
            <Datalist<web_common::database::nested_variants::NestedSample, false> builder={set_parent_sample} optional={false} errors={builder_store.errors_parent_sample.clone()} value={builder_store.parent_sample.clone()} label="Parent sample" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedSample, false> builder={set_child_sample} optional={false} errors={builder_store.errors_child_sample.clone()} value={builder_store.child_sample.clone()} label="Child sample" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedUnit, false> builder={set_unit} optional={false} errors={builder_store.errors_unit.clone()} value={builder_store.unit.clone()} label="Unit" scanner={false} />
        </BasicForm<UpdateDerivedSample>>
    }
}
