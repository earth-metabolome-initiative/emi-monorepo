//! This module contains the forms for the frontend.
//!
//! This module is automatically generated. Do not write anything here.

use serde::{Deserialize, Serialize};
use web_common::database::*;
use yew::prelude::*;
use yewdux::{use_store, Reducer, Store};
use crate::components::forms::*;
use web_common::api::form_traits::FormMethod;
use std::rc::Rc;
use uuid::Uuid;
use std::ops::Deref;
use chrono::NaiveDateTime;
use web_common::api::ApiError;

#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewContainerHorizontalRuleBuilder {
    pub item_type: Option<NestedItemCategory>,
    pub other_item_type: Option<NestedItemCategory>,
    pub name: Option<String>,
    pub minimum_temperature: Option<i32>,
    pub maximum_temperature: Option<i32>,
    pub minimum_humidity: Option<i32>,
    pub maximum_humidity: Option<i32>,
    pub minimum_pressure: Option<i32>,
    pub maximum_pressure: Option<i32>,
    pub errors_item_type: Vec<ApiError>,
    pub errors_other_item_type: Vec<ApiError>,
    pub errors_name: Vec<ApiError>,
    pub errors_minimum_temperature: Vec<ApiError>,
    pub errors_maximum_temperature: Vec<ApiError>,
    pub errors_minimum_humidity: Vec<ApiError>,
    pub errors_maximum_humidity: Vec<ApiError>,
    pub errors_minimum_pressure: Vec<ApiError>,
    pub errors_maximum_pressure: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewContainerHorizontalRuleBuilderActions {
    SetItemType(Option<NestedItemCategory>),
    SetOtherItemType(Option<NestedItemCategory>),
    SetName(Option<String>),
    SetMinimumTemperature(Option<i32>),
    SetMaximumTemperature(Option<i32>),
    SetMinimumHumidity(Option<i32>),
    SetMaximumHumidity(Option<i32>),
    SetMinimumPressure(Option<i32>),
    SetMaximumPressure(Option<i32>),
}

impl FormBuilder for NestedNewContainerHorizontalRuleBuilder {
    type Data = NewContainerHorizontalRule;
    type Actions = NestedNewContainerHorizontalRuleBuilderActions;

    fn has_errors(&self) -> bool {
!self.errors_item_type.is_empty() || !self.errors_other_item_type.is_empty() || !self.errors_name.is_empty() || !self.errors_minimum_temperature.is_empty() || !self.errors_maximum_temperature.is_empty() || !self.errors_minimum_humidity.is_empty() || !self.errors_maximum_humidity.is_empty() || !self.errors_minimum_pressure.is_empty() || !self.errors_maximum_pressure.is_empty()
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.item_type.is_some()
        && self.other_item_type.is_some()
        && self.name.is_some()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }

}
impl From<NestedNewContainerHorizontalRuleBuilder> for NewContainerHorizontalRule {
    fn from(builder: NestedNewContainerHorizontalRuleBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            item_type_id: builder.item_type.unwrap().inner.id,
            other_item_type_id: builder.other_item_type.unwrap().inner.id,
            minimum_temperature: builder.minimum_temperature,
            maximum_temperature: builder.maximum_temperature,
            minimum_humidity: builder.minimum_humidity,
            maximum_humidity: builder.maximum_humidity,
            minimum_pressure: builder.minimum_pressure,
            maximum_pressure: builder.maximum_pressure,
        }
    }
}
impl FormBuildable for NewContainerHorizontalRule {
    type Builder = NestedNewContainerHorizontalRuleBuilder;
    const TABLE: Table = Table::ContainerHorizontalRules;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewContainerHorizontalRule"
    }
    fn task_target() -> &'static str {
        "NewContainerHorizontalRule"
    }
    fn description() -> &'static str {
        concat!("Create a new NewContainerHorizontalRule.",)
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}
impl Reducer<NestedNewContainerHorizontalRuleBuilder> for NestedNewContainerHorizontalRuleBuilderActions {
    fn apply(self, mut state: std::rc::Rc<NestedNewContainerHorizontalRuleBuilder>) -> std::rc::Rc<NestedNewContainerHorizontalRuleBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewContainerHorizontalRuleBuilderActions::SetItemType(item_type) => {
        if item_type.is_none() {
            state_mut.errors_item_type.push(ApiError::BadRequest(vec![
                "The ItemType field is required.".to_string()
             ]));
        }
                state_mut.item_type = item_type;
            }
            NestedNewContainerHorizontalRuleBuilderActions::SetOtherItemType(other_item_type) => {
        if other_item_type.is_none() {
            state_mut.errors_other_item_type.push(ApiError::BadRequest(vec![
                "The OtherItemType field is required.".to_string()
             ]));
        }
                state_mut.other_item_type = other_item_type;
            }
            NestedNewContainerHorizontalRuleBuilderActions::SetName(name) => {
        if name.is_none() {
            state_mut.errors_name.push(ApiError::BadRequest(vec![
                "The Name field is required.".to_string()
             ]));
        }
                state_mut.name = name;
            }
            NestedNewContainerHorizontalRuleBuilderActions::SetMinimumTemperature(minimum_temperature) => {
                state_mut.minimum_temperature = minimum_temperature;
            }
            NestedNewContainerHorizontalRuleBuilderActions::SetMaximumTemperature(maximum_temperature) => {
                state_mut.maximum_temperature = maximum_temperature;
            }
            NestedNewContainerHorizontalRuleBuilderActions::SetMinimumHumidity(minimum_humidity) => {
                state_mut.minimum_humidity = minimum_humidity;
            }
            NestedNewContainerHorizontalRuleBuilderActions::SetMaximumHumidity(maximum_humidity) => {
                state_mut.maximum_humidity = maximum_humidity;
            }
            NestedNewContainerHorizontalRuleBuilderActions::SetMinimumPressure(minimum_pressure) => {
                state_mut.minimum_pressure = minimum_pressure;
            }
            NestedNewContainerHorizontalRuleBuilderActions::SetMaximumPressure(maximum_pressure) => {
                state_mut.maximum_pressure = maximum_pressure;
            }
        }
        state
    }
}
#[function_component(NewContainerHorizontalRuleForm)]
pub fn new_container_horizontal_rule_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewContainerHorizontalRuleBuilder>();
    let set_item_type = builder_dispatch.apply_callback(|item_type: Option<NestedItemCategory>| NestedNewContainerHorizontalRuleBuilderActions::SetItemType(item_type));
    let set_other_item_type = builder_dispatch.apply_callback(|other_item_type: Option<NestedItemCategory>| NestedNewContainerHorizontalRuleBuilderActions::SetOtherItemType(other_item_type));
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| NestedNewContainerHorizontalRuleBuilderActions::SetName(name));
    let set_minimum_temperature = builder_dispatch.apply_callback(|minimum_temperature: Option<i32>| NestedNewContainerHorizontalRuleBuilderActions::SetMinimumTemperature(minimum_temperature));
    let set_maximum_temperature = builder_dispatch.apply_callback(|maximum_temperature: Option<i32>| NestedNewContainerHorizontalRuleBuilderActions::SetMaximumTemperature(maximum_temperature));
    let set_minimum_humidity = builder_dispatch.apply_callback(|minimum_humidity: Option<i32>| NestedNewContainerHorizontalRuleBuilderActions::SetMinimumHumidity(minimum_humidity));
    let set_maximum_humidity = builder_dispatch.apply_callback(|maximum_humidity: Option<i32>| NestedNewContainerHorizontalRuleBuilderActions::SetMaximumHumidity(maximum_humidity));
    let set_minimum_pressure = builder_dispatch.apply_callback(|minimum_pressure: Option<i32>| NestedNewContainerHorizontalRuleBuilderActions::SetMinimumPressure(minimum_pressure));
    let set_maximum_pressure = builder_dispatch.apply_callback(|maximum_pressure: Option<i32>| NestedNewContainerHorizontalRuleBuilderActions::SetMaximumPressure(maximum_pressure));
    html! {
        <BasicForm<NewContainerHorizontalRule> builder={builder_store.deref().clone()}>
            <Datalist<NestedItemCategory> builder={set_item_type} errors={builder_store.errors_item_type.clone()} value={builder_store.item_type.clone()} label="ItemType" />
            <Datalist<NestedItemCategory> builder={set_other_item_type} errors={builder_store.errors_other_item_type.clone()} value={builder_store.other_item_type.clone()} label="OtherItemType" />
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
        </BasicForm<NewContainerHorizontalRule>>
    }
}
#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewContainerVerticalRuleBuilder {
    pub container_item_type: Option<NestedItemCategory>,
    pub contained_item_type: Option<NestedItemCategory>,
    pub name: Option<String>,
    pub minimum_temperature: Option<i32>,
    pub maximum_temperature: Option<i32>,
    pub minimum_humidity: Option<i32>,
    pub maximum_humidity: Option<i32>,
    pub minimum_pressure: Option<i32>,
    pub maximum_pressure: Option<i32>,
    pub errors_container_item_type: Vec<ApiError>,
    pub errors_contained_item_type: Vec<ApiError>,
    pub errors_name: Vec<ApiError>,
    pub errors_minimum_temperature: Vec<ApiError>,
    pub errors_maximum_temperature: Vec<ApiError>,
    pub errors_minimum_humidity: Vec<ApiError>,
    pub errors_maximum_humidity: Vec<ApiError>,
    pub errors_minimum_pressure: Vec<ApiError>,
    pub errors_maximum_pressure: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewContainerVerticalRuleBuilderActions {
    SetContainerItemType(Option<NestedItemCategory>),
    SetContainedItemType(Option<NestedItemCategory>),
    SetName(Option<String>),
    SetMinimumTemperature(Option<i32>),
    SetMaximumTemperature(Option<i32>),
    SetMinimumHumidity(Option<i32>),
    SetMaximumHumidity(Option<i32>),
    SetMinimumPressure(Option<i32>),
    SetMaximumPressure(Option<i32>),
}

impl FormBuilder for NestedNewContainerVerticalRuleBuilder {
    type Data = NewContainerVerticalRule;
    type Actions = NestedNewContainerVerticalRuleBuilderActions;

    fn has_errors(&self) -> bool {
!self.errors_container_item_type.is_empty() || !self.errors_contained_item_type.is_empty() || !self.errors_name.is_empty() || !self.errors_minimum_temperature.is_empty() || !self.errors_maximum_temperature.is_empty() || !self.errors_minimum_humidity.is_empty() || !self.errors_maximum_humidity.is_empty() || !self.errors_minimum_pressure.is_empty() || !self.errors_maximum_pressure.is_empty()
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.container_item_type.is_some()
        && self.contained_item_type.is_some()
        && self.name.is_some()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }

}
impl From<NestedNewContainerVerticalRuleBuilder> for NewContainerVerticalRule {
    fn from(builder: NestedNewContainerVerticalRuleBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            container_item_type_id: builder.container_item_type.unwrap().inner.id,
            contained_item_type_id: builder.contained_item_type.unwrap().inner.id,
            minimum_temperature: builder.minimum_temperature,
            maximum_temperature: builder.maximum_temperature,
            minimum_humidity: builder.minimum_humidity,
            maximum_humidity: builder.maximum_humidity,
            minimum_pressure: builder.minimum_pressure,
            maximum_pressure: builder.maximum_pressure,
        }
    }
}
impl FormBuildable for NewContainerVerticalRule {
    type Builder = NestedNewContainerVerticalRuleBuilder;
    const TABLE: Table = Table::ContainerVerticalRules;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewContainerVerticalRule"
    }
    fn task_target() -> &'static str {
        "NewContainerVerticalRule"
    }
    fn description() -> &'static str {
        concat!("Create a new NewContainerVerticalRule.",)
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}
impl Reducer<NestedNewContainerVerticalRuleBuilder> for NestedNewContainerVerticalRuleBuilderActions {
    fn apply(self, mut state: std::rc::Rc<NestedNewContainerVerticalRuleBuilder>) -> std::rc::Rc<NestedNewContainerVerticalRuleBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewContainerVerticalRuleBuilderActions::SetContainerItemType(container_item_type) => {
        if container_item_type.is_none() {
            state_mut.errors_container_item_type.push(ApiError::BadRequest(vec![
                "The ContainerItemType field is required.".to_string()
             ]));
        }
                state_mut.container_item_type = container_item_type;
            }
            NestedNewContainerVerticalRuleBuilderActions::SetContainedItemType(contained_item_type) => {
        if contained_item_type.is_none() {
            state_mut.errors_contained_item_type.push(ApiError::BadRequest(vec![
                "The ContainedItemType field is required.".to_string()
             ]));
        }
                state_mut.contained_item_type = contained_item_type;
            }
            NestedNewContainerVerticalRuleBuilderActions::SetName(name) => {
        if name.is_none() {
            state_mut.errors_name.push(ApiError::BadRequest(vec![
                "The Name field is required.".to_string()
             ]));
        }
                state_mut.name = name;
            }
            NestedNewContainerVerticalRuleBuilderActions::SetMinimumTemperature(minimum_temperature) => {
                state_mut.minimum_temperature = minimum_temperature;
            }
            NestedNewContainerVerticalRuleBuilderActions::SetMaximumTemperature(maximum_temperature) => {
                state_mut.maximum_temperature = maximum_temperature;
            }
            NestedNewContainerVerticalRuleBuilderActions::SetMinimumHumidity(minimum_humidity) => {
                state_mut.minimum_humidity = minimum_humidity;
            }
            NestedNewContainerVerticalRuleBuilderActions::SetMaximumHumidity(maximum_humidity) => {
                state_mut.maximum_humidity = maximum_humidity;
            }
            NestedNewContainerVerticalRuleBuilderActions::SetMinimumPressure(minimum_pressure) => {
                state_mut.minimum_pressure = minimum_pressure;
            }
            NestedNewContainerVerticalRuleBuilderActions::SetMaximumPressure(maximum_pressure) => {
                state_mut.maximum_pressure = maximum_pressure;
            }
        }
        state
    }
}
#[function_component(NewContainerVerticalRuleForm)]
pub fn new_container_vertical_rule_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewContainerVerticalRuleBuilder>();
    let set_container_item_type = builder_dispatch.apply_callback(|container_item_type: Option<NestedItemCategory>| NestedNewContainerVerticalRuleBuilderActions::SetContainerItemType(container_item_type));
    let set_contained_item_type = builder_dispatch.apply_callback(|contained_item_type: Option<NestedItemCategory>| NestedNewContainerVerticalRuleBuilderActions::SetContainedItemType(contained_item_type));
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| NestedNewContainerVerticalRuleBuilderActions::SetName(name));
    let set_minimum_temperature = builder_dispatch.apply_callback(|minimum_temperature: Option<i32>| NestedNewContainerVerticalRuleBuilderActions::SetMinimumTemperature(minimum_temperature));
    let set_maximum_temperature = builder_dispatch.apply_callback(|maximum_temperature: Option<i32>| NestedNewContainerVerticalRuleBuilderActions::SetMaximumTemperature(maximum_temperature));
    let set_minimum_humidity = builder_dispatch.apply_callback(|minimum_humidity: Option<i32>| NestedNewContainerVerticalRuleBuilderActions::SetMinimumHumidity(minimum_humidity));
    let set_maximum_humidity = builder_dispatch.apply_callback(|maximum_humidity: Option<i32>| NestedNewContainerVerticalRuleBuilderActions::SetMaximumHumidity(maximum_humidity));
    let set_minimum_pressure = builder_dispatch.apply_callback(|minimum_pressure: Option<i32>| NestedNewContainerVerticalRuleBuilderActions::SetMinimumPressure(minimum_pressure));
    let set_maximum_pressure = builder_dispatch.apply_callback(|maximum_pressure: Option<i32>| NestedNewContainerVerticalRuleBuilderActions::SetMaximumPressure(maximum_pressure));
    html! {
        <BasicForm<NewContainerVerticalRule> builder={builder_store.deref().clone()}>
            <Datalist<NestedItemCategory> builder={set_container_item_type} errors={builder_store.errors_container_item_type.clone()} value={builder_store.container_item_type.clone()} label="ContainerItemType" />
            <Datalist<NestedItemCategory> builder={set_contained_item_type} errors={builder_store.errors_contained_item_type.clone()} value={builder_store.contained_item_type.clone()} label="ContainedItemType" />
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
        </BasicForm<NewContainerVerticalRule>>
    }
}
#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewItemCategoryBuilder {
    pub name: Option<String>,
    pub description: Option<String>,
    pub errors_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewItemCategoryBuilderActions {
    SetName(Option<String>),
    SetDescription(Option<String>),
}

impl FormBuilder for NestedNewItemCategoryBuilder {
    type Data = NewItemCategory;
    type Actions = NestedNewItemCategoryBuilderActions;

    fn has_errors(&self) -> bool {
!self.errors_name.is_empty() || !self.errors_description.is_empty()
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.name.is_some()
        && self.description.is_some()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }

}
impl From<NestedNewItemCategoryBuilder> for NewItemCategory {
    fn from(builder: NestedNewItemCategoryBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            description: builder.description.unwrap(),
        }
    }
}
impl FormBuildable for NewItemCategory {
    type Builder = NestedNewItemCategoryBuilder;
    const TABLE: Table = Table::ItemCategories;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewItemCategory"
    }
    fn task_target() -> &'static str {
        "NewItemCategory"
    }
    fn description() -> &'static str {
        concat!("Create a new NewItemCategory.",)
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}
impl Reducer<NestedNewItemCategoryBuilder> for NestedNewItemCategoryBuilderActions {
    fn apply(self, mut state: std::rc::Rc<NestedNewItemCategoryBuilder>) -> std::rc::Rc<NestedNewItemCategoryBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewItemCategoryBuilderActions::SetName(name) => {
        if name.is_none() {
            state_mut.errors_name.push(ApiError::BadRequest(vec![
                "The Name field is required.".to_string()
             ]));
        }
                state_mut.name = name;
            }
            NestedNewItemCategoryBuilderActions::SetDescription(description) => {
        if description.is_none() {
            state_mut.errors_description.push(ApiError::BadRequest(vec![
                "The Description field is required.".to_string()
             ]));
        }
                state_mut.description = description;
            }
        }
        state
    }
}
#[function_component(NewItemCategoryForm)]
pub fn new_item_category_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewItemCategoryBuilder>();
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| NestedNewItemCategoryBuilderActions::SetName(name));
    let set_description = builder_dispatch.apply_callback(|description: Option<String>| NestedNewItemCategoryBuilderActions::SetDescription(description));
    html! {
        <BasicForm<NewItemCategory> builder={builder_store.deref().clone()}>
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
            <BasicInput label="Description" errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} input_type={InputType::Text} />
        </BasicForm<NewItemCategory>>
    }
}
#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewProcedureBuilder {
    pub name: Option<String>,
    pub description: Option<String>,
    pub errors_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewProcedureBuilderActions {
    SetName(Option<String>),
    SetDescription(Option<String>),
}

impl FormBuilder for NestedNewProcedureBuilder {
    type Data = NewProcedure;
    type Actions = NestedNewProcedureBuilderActions;

    fn has_errors(&self) -> bool {
!self.errors_name.is_empty() || !self.errors_description.is_empty()
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.name.is_some()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }

}
impl From<NestedNewProcedureBuilder> for NewProcedure {
    fn from(builder: NestedNewProcedureBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            description: builder.description,
        }
    }
}
impl FormBuildable for NewProcedure {
    type Builder = NestedNewProcedureBuilder;
    const TABLE: Table = Table::Procedures;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewProcedure"
    }
    fn task_target() -> &'static str {
        "NewProcedure"
    }
    fn description() -> &'static str {
        concat!("Create a new NewProcedure.",)
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}
impl Reducer<NestedNewProcedureBuilder> for NestedNewProcedureBuilderActions {
    fn apply(self, mut state: std::rc::Rc<NestedNewProcedureBuilder>) -> std::rc::Rc<NestedNewProcedureBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewProcedureBuilderActions::SetName(name) => {
        if name.is_none() {
            state_mut.errors_name.push(ApiError::BadRequest(vec![
                "The Name field is required.".to_string()
             ]));
        }
                state_mut.name = name;
            }
            NestedNewProcedureBuilderActions::SetDescription(description) => {
                state_mut.description = description;
            }
        }
        state
    }
}
#[function_component(NewProcedureForm)]
pub fn new_procedure_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewProcedureBuilder>();
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| NestedNewProcedureBuilderActions::SetName(name));
    let set_description = builder_dispatch.apply_callback(|description: Option<String>| NestedNewProcedureBuilderActions::SetDescription(description));
    html! {
        <BasicForm<NewProcedure> builder={builder_store.deref().clone()}>
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
            <BasicInput label="Description" errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} input_type={InputType::Text} />
        </BasicForm<NewProcedure>>
    }
}
#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewProjectRequirementBuilder {
    pub project: Option<NestedProject>,
    pub item_category: Option<NestedItemCategory>,
    pub unit: Option<Unit>,
    pub quantity: Option<i32>,
    pub errors_project: Vec<ApiError>,
    pub errors_item_category: Vec<ApiError>,
    pub errors_unit: Vec<ApiError>,
    pub errors_quantity: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewProjectRequirementBuilderActions {
    SetProject(Option<NestedProject>),
    SetItemCategory(Option<NestedItemCategory>),
    SetUnit(Option<Unit>),
    SetQuantity(Option<i32>),
}

impl FormBuilder for NestedNewProjectRequirementBuilder {
    type Data = NewProjectRequirement;
    type Actions = NestedNewProjectRequirementBuilderActions;

    fn has_errors(&self) -> bool {
!self.errors_project.is_empty() || !self.errors_item_category.is_empty() || !self.errors_unit.is_empty() || !self.errors_quantity.is_empty()
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.project.is_some()
        && self.item_category.is_some()
        && self.quantity.is_some()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }

}
impl From<NestedNewProjectRequirementBuilder> for NewProjectRequirement {
    fn from(builder: NestedNewProjectRequirementBuilder) -> Self {
        Self {
            project_id: builder.project.unwrap().inner.id,
            item_category_id: builder.item_category.unwrap().inner.id,
            quantity: builder.quantity.unwrap(),
            unit_id: builder.unit.map(|unit| unit.id),
        }
    }
}
impl FormBuildable for NewProjectRequirement {
    type Builder = NestedNewProjectRequirementBuilder;
    const TABLE: Table = Table::ProjectRequirements;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewProjectRequirement"
    }
    fn task_target() -> &'static str {
        "NewProjectRequirement"
    }
    fn description() -> &'static str {
        concat!("Create a new NewProjectRequirement.",)
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}
impl Reducer<NestedNewProjectRequirementBuilder> for NestedNewProjectRequirementBuilderActions {
    fn apply(self, mut state: std::rc::Rc<NestedNewProjectRequirementBuilder>) -> std::rc::Rc<NestedNewProjectRequirementBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewProjectRequirementBuilderActions::SetProject(project) => {
        if project.is_none() {
            state_mut.errors_project.push(ApiError::BadRequest(vec![
                "The Project field is required.".to_string()
             ]));
        }
                state_mut.project = project;
            }
            NestedNewProjectRequirementBuilderActions::SetItemCategory(item_category) => {
        if item_category.is_none() {
            state_mut.errors_item_category.push(ApiError::BadRequest(vec![
                "The ItemCategory field is required.".to_string()
             ]));
        }
                state_mut.item_category = item_category;
            }
            NestedNewProjectRequirementBuilderActions::SetUnit(unit) => {
                state_mut.unit = unit;
            }
            NestedNewProjectRequirementBuilderActions::SetQuantity(quantity) => {
        if quantity.is_none() {
            state_mut.errors_quantity.push(ApiError::BadRequest(vec![
                "The Quantity field is required.".to_string()
             ]));
        }
                state_mut.quantity = quantity;
            }
        }
        state
    }
}
#[function_component(NewProjectRequirementForm)]
pub fn new_project_requirement_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewProjectRequirementBuilder>();
    let set_project = builder_dispatch.apply_callback(|project: Option<NestedProject>| NestedNewProjectRequirementBuilderActions::SetProject(project));
    let set_item_category = builder_dispatch.apply_callback(|item_category: Option<NestedItemCategory>| NestedNewProjectRequirementBuilderActions::SetItemCategory(item_category));
    let set_unit = builder_dispatch.apply_callback(|unit: Option<Unit>| NestedNewProjectRequirementBuilderActions::SetUnit(unit));
    let set_quantity = builder_dispatch.apply_callback(|quantity: Option<i32>| NestedNewProjectRequirementBuilderActions::SetQuantity(quantity));
    html! {
        <BasicForm<NewProjectRequirement> builder={builder_store.deref().clone()}>
            <Datalist<NestedProject> builder={set_project} errors={builder_store.errors_project.clone()} value={builder_store.project.clone()} label="Project" />
            <Datalist<NestedItemCategory> builder={set_item_category} errors={builder_store.errors_item_category.clone()} value={builder_store.item_category.clone()} label="ItemCategory" />
            <Datalist<Unit> builder={set_unit} errors={builder_store.errors_unit.clone()} value={builder_store.unit.clone()} label="Unit" />
        </BasicForm<NewProjectRequirement>>
    }
}
#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewProjectBuilder {
    pub state: Option<NestedProjectState>,
    pub parent_project: Option<NestedProject>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub public: Option<bool>,
    pub budget: Option<i64>,
    pub expenses: Option<i64>,
    pub expected_end_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub errors_state: Vec<ApiError>,
    pub errors_parent_project: Vec<ApiError>,
    pub errors_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
    pub errors_public: Vec<ApiError>,
    pub errors_budget: Vec<ApiError>,
    pub errors_expenses: Vec<ApiError>,
    pub errors_expected_end_date: Vec<ApiError>,
    pub errors_end_date: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewProjectBuilderActions {
    SetState(Option<NestedProjectState>),
    SetParentProject(Option<NestedProject>),
    SetName(Option<String>),
    SetDescription(Option<String>),
    SetPublic(Option<bool>),
    SetBudget(Option<i64>),
    SetExpenses(Option<i64>),
    SetExpectedEndDate(Option<NaiveDateTime>),
    SetEndDate(Option<NaiveDateTime>),
}

impl FormBuilder for NestedNewProjectBuilder {
    type Data = NewProject;
    type Actions = NestedNewProjectBuilderActions;

    fn has_errors(&self) -> bool {
!self.errors_state.is_empty() || !self.errors_parent_project.is_empty() || !self.errors_name.is_empty() || !self.errors_description.is_empty() || !self.errors_public.is_empty() || !self.errors_budget.is_empty() || !self.errors_expenses.is_empty() || !self.errors_expected_end_date.is_empty() || !self.errors_end_date.is_empty()
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.state.is_some()
        && self.name.is_some()
        && self.description.is_some()
        && self.public.is_some()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }

}
impl From<NestedNewProjectBuilder> for NewProject {
    fn from(builder: NestedNewProjectBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            description: builder.description.unwrap(),
            public: builder.public.unwrap(),
            state_id: builder.state.unwrap().inner.id,
            parent_project_id: builder.parent_project.map(|parent_project| parent_project.inner.id),
            budget: builder.budget,
            expenses: builder.expenses,
            expected_end_date: builder.expected_end_date,
            end_date: builder.end_date,
        }
    }
}
impl FormBuildable for NewProject {
    type Builder = NestedNewProjectBuilder;
    const TABLE: Table = Table::Projects;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewProject"
    }
    fn task_target() -> &'static str {
        "NewProject"
    }
    fn description() -> &'static str {
        concat!("Create a new NewProject.",)
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}
impl Reducer<NestedNewProjectBuilder> for NestedNewProjectBuilderActions {
    fn apply(self, mut state: std::rc::Rc<NestedNewProjectBuilder>) -> std::rc::Rc<NestedNewProjectBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewProjectBuilderActions::SetState(state) => {
        if state.is_none() {
            state_mut.errors_state.push(ApiError::BadRequest(vec![
                "The State field is required.".to_string()
             ]));
        }
                state_mut.state = state;
            }
            NestedNewProjectBuilderActions::SetParentProject(parent_project) => {
                state_mut.parent_project = parent_project;
            }
            NestedNewProjectBuilderActions::SetName(name) => {
        if name.is_none() {
            state_mut.errors_name.push(ApiError::BadRequest(vec![
                "The Name field is required.".to_string()
             ]));
        }
                state_mut.name = name;
            }
            NestedNewProjectBuilderActions::SetDescription(description) => {
        if description.is_none() {
            state_mut.errors_description.push(ApiError::BadRequest(vec![
                "The Description field is required.".to_string()
             ]));
        }
                state_mut.description = description;
            }
            NestedNewProjectBuilderActions::SetPublic(public) => {
        if public.is_none() {
            state_mut.errors_public.push(ApiError::BadRequest(vec![
                "The Public field is required.".to_string()
             ]));
        }
                state_mut.public = public;
            }
            NestedNewProjectBuilderActions::SetBudget(budget) => {
                state_mut.budget = budget;
            }
            NestedNewProjectBuilderActions::SetExpenses(expenses) => {
                state_mut.expenses = expenses;
            }
            NestedNewProjectBuilderActions::SetExpectedEndDate(expected_end_date) => {
                state_mut.expected_end_date = expected_end_date;
            }
            NestedNewProjectBuilderActions::SetEndDate(end_date) => {
                state_mut.end_date = end_date;
            }
        }
        state
    }
}
#[function_component(NewProjectForm)]
pub fn new_project_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewProjectBuilder>();
    let set_state = builder_dispatch.apply_callback(|state: Option<NestedProjectState>| NestedNewProjectBuilderActions::SetState(state));
    let set_parent_project = builder_dispatch.apply_callback(|parent_project: Option<NestedProject>| NestedNewProjectBuilderActions::SetParentProject(parent_project));
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| NestedNewProjectBuilderActions::SetName(name));
    let set_description = builder_dispatch.apply_callback(|description: Option<String>| NestedNewProjectBuilderActions::SetDescription(description));
    let set_public = builder_dispatch.apply_callback(|public: bool| NestedNewProjectBuilderActions::SetPublic(Some(public)));
    let set_budget = builder_dispatch.apply_callback(|budget: Option<i64>| NestedNewProjectBuilderActions::SetBudget(budget));
    let set_expenses = builder_dispatch.apply_callback(|expenses: Option<i64>| NestedNewProjectBuilderActions::SetExpenses(expenses));
    let set_expected_end_date = builder_dispatch.apply_callback(|expected_end_date: Option<NaiveDateTime>| NestedNewProjectBuilderActions::SetExpectedEndDate(expected_end_date));
    let set_end_date = builder_dispatch.apply_callback(|end_date: Option<NaiveDateTime>| NestedNewProjectBuilderActions::SetEndDate(end_date));
    html! {
        <BasicForm<NewProject> builder={builder_store.deref().clone()}>
            <Datalist<NestedProjectState> builder={set_state} errors={builder_store.errors_state.clone()} value={builder_store.state.clone()} label="State" />
            <Datalist<NestedProject> builder={set_parent_project} errors={builder_store.errors_parent_project.clone()} value={builder_store.parent_project.clone()} label="ParentProject" />
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
            <BasicInput label="Description" errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} input_type={InputType::Text} />
            <Checkbox label="Public" errors={builder_store.errors_public.clone()} builder={set_public} value={builder_store.public.unwrap_or(false)} />
        </BasicForm<NewProject>>
    }
}
#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewSampleBuilder {
    pub sampled_by: Option<User>,
    pub procedure: Option<NestedSamplingProcedure>,
    pub state: Option<NestedSampleState>,
    pub errors_sampled_by: Vec<ApiError>,
    pub errors_procedure: Vec<ApiError>,
    pub errors_state: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewSampleBuilderActions {
    SetSampledBy(Option<User>),
    SetProcedure(Option<NestedSamplingProcedure>),
    SetState(Option<NestedSampleState>),
}

impl FormBuilder for NestedNewSampleBuilder {
    type Data = NewSample;
    type Actions = NestedNewSampleBuilderActions;

    fn has_errors(&self) -> bool {
!self.errors_sampled_by.is_empty() || !self.errors_procedure.is_empty() || !self.errors_state.is_empty()
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.sampled_by.is_some()
        && self.procedure.is_some()
        && self.state.is_some()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }

}
impl From<NestedNewSampleBuilder> for NewSample {
    fn from(builder: NestedNewSampleBuilder) -> Self {
        Self {
            id: Uuid::new_v4(),
            sampled_by: builder.sampled_by.unwrap().id,
            procedure_id: builder.procedure.unwrap().inner.id,
            state: builder.state.unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewSample {
    type Builder = NestedNewSampleBuilder;
    const TABLE: Table = Table::Samples;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewSample"
    }
    fn task_target() -> &'static str {
        "NewSample"
    }
    fn description() -> &'static str {
        concat!("Create a new NewSample.",)
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}
impl Reducer<NestedNewSampleBuilder> for NestedNewSampleBuilderActions {
    fn apply(self, mut state: std::rc::Rc<NestedNewSampleBuilder>) -> std::rc::Rc<NestedNewSampleBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewSampleBuilderActions::SetSampledBy(sampled_by) => {
        if sampled_by.is_none() {
            state_mut.errors_sampled_by.push(ApiError::BadRequest(vec![
                "The SampledBy field is required.".to_string()
             ]));
        }
                state_mut.sampled_by = sampled_by;
            }
            NestedNewSampleBuilderActions::SetProcedure(procedure) => {
        if procedure.is_none() {
            state_mut.errors_procedure.push(ApiError::BadRequest(vec![
                "The Procedure field is required.".to_string()
             ]));
        }
                state_mut.procedure = procedure;
            }
            NestedNewSampleBuilderActions::SetState(state) => {
        if state.is_none() {
            state_mut.errors_state.push(ApiError::BadRequest(vec![
                "The State field is required.".to_string()
             ]));
        }
                state_mut.state = state;
            }
        }
        state
    }
}
#[function_component(NewSampleForm)]
pub fn new_sample_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewSampleBuilder>();
    let set_sampled_by = builder_dispatch.apply_callback(|sampled_by: Option<User>| NestedNewSampleBuilderActions::SetSampledBy(sampled_by));
    let set_procedure = builder_dispatch.apply_callback(|procedure: Option<NestedSamplingProcedure>| NestedNewSampleBuilderActions::SetProcedure(procedure));
    let set_state = builder_dispatch.apply_callback(|state: Option<NestedSampleState>| NestedNewSampleBuilderActions::SetState(state));
    html! {
        <BasicForm<NewSample> builder={builder_store.deref().clone()}>
            <Datalist<User> builder={set_sampled_by} errors={builder_store.errors_sampled_by.clone()} value={builder_store.sampled_by.clone()} label="SampledBy" />
            <Datalist<NestedSamplingProcedure> builder={set_procedure} errors={builder_store.errors_procedure.clone()} value={builder_store.procedure.clone()} label="Procedure" />
            <Datalist<NestedSampleState> builder={set_state} errors={builder_store.errors_state.clone()} value={builder_store.state.clone()} label="State" />
        </BasicForm<NewSample>>
    }
}
#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewSamplingProcedureBuilder {
    pub name: Option<String>,
    pub description: Option<String>,
    pub errors_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewSamplingProcedureBuilderActions {
    SetName(Option<String>),
    SetDescription(Option<String>),
}

impl FormBuilder for NestedNewSamplingProcedureBuilder {
    type Data = NewSamplingProcedure;
    type Actions = NestedNewSamplingProcedureBuilderActions;

    fn has_errors(&self) -> bool {
!self.errors_name.is_empty() || !self.errors_description.is_empty()
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.name.is_some()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }

}
impl From<NestedNewSamplingProcedureBuilder> for NewSamplingProcedure {
    fn from(builder: NestedNewSamplingProcedureBuilder) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: builder.name.unwrap(),
            description: builder.description,
        }
    }
}
impl FormBuildable for NewSamplingProcedure {
    type Builder = NestedNewSamplingProcedureBuilder;
    const TABLE: Table = Table::SamplingProcedures;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewSamplingProcedure"
    }
    fn task_target() -> &'static str {
        "NewSamplingProcedure"
    }
    fn description() -> &'static str {
        concat!("Create a new NewSamplingProcedure.",)
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}
impl Reducer<NestedNewSamplingProcedureBuilder> for NestedNewSamplingProcedureBuilderActions {
    fn apply(self, mut state: std::rc::Rc<NestedNewSamplingProcedureBuilder>) -> std::rc::Rc<NestedNewSamplingProcedureBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewSamplingProcedureBuilderActions::SetName(name) => {
        if name.is_none() {
            state_mut.errors_name.push(ApiError::BadRequest(vec![
                "The Name field is required.".to_string()
             ]));
        }
                state_mut.name = name;
            }
            NestedNewSamplingProcedureBuilderActions::SetDescription(description) => {
                state_mut.description = description;
            }
        }
        state
    }
}
#[function_component(NewSamplingProcedureForm)]
pub fn new_sampling_procedure_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewSamplingProcedureBuilder>();
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| NestedNewSamplingProcedureBuilderActions::SetName(name));
    let set_description = builder_dispatch.apply_callback(|description: Option<String>| NestedNewSamplingProcedureBuilderActions::SetDescription(description));
    html! {
        <BasicForm<NewSamplingProcedure> builder={builder_store.deref().clone()}>
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
            <BasicInput label="Description" errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} input_type={InputType::Text} />
        </BasicForm<NewSamplingProcedure>>
    }
}
#[derive(Store, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[store(storage = "session")]
pub struct NestedNewTeamBuilder {
    pub parent_team: Option<NestedTeam>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub errors_parent_team: Vec<ApiError>,
    pub errors_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum NestedNewTeamBuilderActions {
    SetParentTeam(Option<NestedTeam>),
    SetName(Option<String>),
    SetDescription(Option<String>),
}

impl FormBuilder for NestedNewTeamBuilder {
    type Data = NewTeam;
    type Actions = NestedNewTeamBuilderActions;

    fn has_errors(&self) -> bool {
!self.errors_parent_team.is_empty() || !self.errors_name.is_empty() || !self.errors_description.is_empty()
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.name.is_some()
        && self.description.is_some()
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }

}
impl From<NestedNewTeamBuilder> for NewTeam {
    fn from(builder: NestedNewTeamBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            description: builder.description.unwrap(),
            parent_team_id: builder.parent_team.map(|parent_team| parent_team.inner.id),
        }
    }
}
impl FormBuildable for NewTeam {
    type Builder = NestedNewTeamBuilder;
    const TABLE: Table = Table::Teams;
    const METHOD: FormMethod = FormMethod::POST;
    fn title() -> &'static str {
        "NewTeam"
    }
    fn task_target() -> &'static str {
        "NewTeam"
    }
    fn description() -> &'static str {
        concat!("Create a new NewTeam.",)
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}
impl Reducer<NestedNewTeamBuilder> for NestedNewTeamBuilderActions {
    fn apply(self, mut state: std::rc::Rc<NestedNewTeamBuilder>) -> std::rc::Rc<NestedNewTeamBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NestedNewTeamBuilderActions::SetParentTeam(parent_team) => {
                state_mut.parent_team = parent_team;
            }
            NestedNewTeamBuilderActions::SetName(name) => {
        if name.is_none() {
            state_mut.errors_name.push(ApiError::BadRequest(vec![
                "The Name field is required.".to_string()
             ]));
        }
                state_mut.name = name;
            }
            NestedNewTeamBuilderActions::SetDescription(description) => {
        if description.is_none() {
            state_mut.errors_description.push(ApiError::BadRequest(vec![
                "The Description field is required.".to_string()
             ]));
        }
                state_mut.description = description;
            }
        }
        state
    }
}
#[function_component(NewTeamForm)]
pub fn new_team_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<NestedNewTeamBuilder>();
    let set_parent_team = builder_dispatch.apply_callback(|parent_team: Option<NestedTeam>| NestedNewTeamBuilderActions::SetParentTeam(parent_team));
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| NestedNewTeamBuilderActions::SetName(name));
    let set_description = builder_dispatch.apply_callback(|description: Option<String>| NestedNewTeamBuilderActions::SetDescription(description));
    html! {
        <BasicForm<NewTeam> builder={builder_store.deref().clone()}>
            <Datalist<NestedTeam> builder={set_parent_team} errors={builder_store.errors_parent_team.clone()} value={builder_store.parent_team.clone()} label="ParentTeam" />
            <BasicInput label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} input_type={InputType::Text} />
            <BasicInput label="Description" errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} input_type={InputType::Text} />
        </BasicForm<NewTeam>>
    }
}
