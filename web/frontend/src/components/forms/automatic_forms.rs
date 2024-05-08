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

#[derive(Store, Eq, PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
#[store(storage = "local", storage_tab_sync)]
pub struct ContainerHorizontalRuleBuilder {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub minimum_temperature: Option<i32>,
    pub maximum_temperature: Option<i32>,
    pub minimum_humidity: Option<i32>,
    pub maximum_humidity: Option<i32>,
    pub minimum_pressure: Option<i32>,
    pub maximum_pressure: Option<i32>,
    pub item_type: Option<NestedItemCategory>,
    pub other_item_type: Option<NestedItemCategory>,
    pub errors_name: Vec<ApiError>,
    pub errors_minimum_temperature: Vec<ApiError>,
    pub errors_maximum_temperature: Vec<ApiError>,
    pub errors_minimum_humidity: Vec<ApiError>,
    pub errors_maximum_humidity: Vec<ApiError>,
    pub errors_minimum_pressure: Vec<ApiError>,
    pub errors_maximum_pressure: Vec<ApiError>,
    pub errors_item_type: Vec<ApiError>,
    pub errors_other_item_type: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum ContainerHorizontalRuleActions {
    SetName(Option<String>),
    SetMinimumTemperature(Option<String>),
    SetMaximumTemperature(Option<String>),
    SetMinimumHumidity(Option<String>),
    SetMaximumHumidity(Option<String>),
    SetMinimumPressure(Option<String>),
    SetMaximumPressure(Option<String>),
    SetItemType(Option<NestedItemCategory>),
    SetOtherItemType(Option<NestedItemCategory>),
}

impl Reducer<ContainerHorizontalRuleBuilder> for ContainerHorizontalRuleActions {
    fn apply(self, mut state: std::rc::Rc<ContainerHorizontalRuleBuilder>) -> std::rc::Rc<ContainerHorizontalRuleBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        state_mut.form_updated_at = chrono::Utc::now().naive_utc();
        match self {
            ContainerHorizontalRuleActions::SetName(name) => {
                state_mut.errors_name.clear();
        if name.is_none() {
            state_mut.errors_name.push(ApiError::BadRequest(vec![
                "The Name field is required.".to_string()
             ]));
        }
                state_mut.name = name;
            }
            ContainerHorizontalRuleActions::SetMinimumTemperature(minimum_temperature) => {
                state_mut.errors_minimum_temperature.clear();
                match minimum_temperature {
                    Some(value) => match value.parse::<i128>() {
                        Ok(value) => {
                            if value < i32::MIN as i128 || value > i32::MAX as i128 {
                                state_mut.errors_minimum_temperature.push(ApiError::BadRequest(vec![
                                    format!(                                            "The minimum_temperature field must be between {} and {}.",
                                            i32::MIN,
                                            i32::MAX
                                    )
                                ]));
                            } else {
                                state_mut.minimum_temperature = Some(value as i32);
                            }
                        }
                        Err(_) => {
                            state_mut.errors_minimum_temperature.push(ApiError::BadRequest(vec![
                                "The minimum_temperature field must be a valid i32.".to_string()
                            ]));
                        }
                    },
                    None => state_mut.minimum_temperature = None,
                }
            }
            ContainerHorizontalRuleActions::SetMaximumTemperature(maximum_temperature) => {
                state_mut.errors_maximum_temperature.clear();
                match maximum_temperature {
                    Some(value) => match value.parse::<i128>() {
                        Ok(value) => {
                            if value < i32::MIN as i128 || value > i32::MAX as i128 {
                                state_mut.errors_maximum_temperature.push(ApiError::BadRequest(vec![
                                    format!(                                            "The maximum_temperature field must be between {} and {}.",
                                            i32::MIN,
                                            i32::MAX
                                    )
                                ]));
                            } else {
                                state_mut.maximum_temperature = Some(value as i32);
                            }
                        }
                        Err(_) => {
                            state_mut.errors_maximum_temperature.push(ApiError::BadRequest(vec![
                                "The maximum_temperature field must be a valid i32.".to_string()
                            ]));
                        }
                    },
                    None => state_mut.maximum_temperature = None,
                }
            }
            ContainerHorizontalRuleActions::SetMinimumHumidity(minimum_humidity) => {
                state_mut.errors_minimum_humidity.clear();
                match minimum_humidity {
                    Some(value) => match value.parse::<i128>() {
                        Ok(value) => {
                            if value < i32::MIN as i128 || value > i32::MAX as i128 {
                                state_mut.errors_minimum_humidity.push(ApiError::BadRequest(vec![
                                    format!(                                            "The minimum_humidity field must be between {} and {}.",
                                            i32::MIN,
                                            i32::MAX
                                    )
                                ]));
                            } else {
                                state_mut.minimum_humidity = Some(value as i32);
                            }
                        }
                        Err(_) => {
                            state_mut.errors_minimum_humidity.push(ApiError::BadRequest(vec![
                                "The minimum_humidity field must be a valid i32.".to_string()
                            ]));
                        }
                    },
                    None => state_mut.minimum_humidity = None,
                }
            }
            ContainerHorizontalRuleActions::SetMaximumHumidity(maximum_humidity) => {
                state_mut.errors_maximum_humidity.clear();
                match maximum_humidity {
                    Some(value) => match value.parse::<i128>() {
                        Ok(value) => {
                            if value < i32::MIN as i128 || value > i32::MAX as i128 {
                                state_mut.errors_maximum_humidity.push(ApiError::BadRequest(vec![
                                    format!(                                            "The maximum_humidity field must be between {} and {}.",
                                            i32::MIN,
                                            i32::MAX
                                    )
                                ]));
                            } else {
                                state_mut.maximum_humidity = Some(value as i32);
                            }
                        }
                        Err(_) => {
                            state_mut.errors_maximum_humidity.push(ApiError::BadRequest(vec![
                                "The maximum_humidity field must be a valid i32.".to_string()
                            ]));
                        }
                    },
                    None => state_mut.maximum_humidity = None,
                }
            }
            ContainerHorizontalRuleActions::SetMinimumPressure(minimum_pressure) => {
                state_mut.errors_minimum_pressure.clear();
                match minimum_pressure {
                    Some(value) => match value.parse::<i128>() {
                        Ok(value) => {
                            if value < i32::MIN as i128 || value > i32::MAX as i128 {
                                state_mut.errors_minimum_pressure.push(ApiError::BadRequest(vec![
                                    format!(                                            "The minimum_pressure field must be between {} and {}.",
                                            i32::MIN,
                                            i32::MAX
                                    )
                                ]));
                            } else {
                                state_mut.minimum_pressure = Some(value as i32);
                            }
                        }
                        Err(_) => {
                            state_mut.errors_minimum_pressure.push(ApiError::BadRequest(vec![
                                "The minimum_pressure field must be a valid i32.".to_string()
                            ]));
                        }
                    },
                    None => state_mut.minimum_pressure = None,
                }
            }
            ContainerHorizontalRuleActions::SetMaximumPressure(maximum_pressure) => {
                state_mut.errors_maximum_pressure.clear();
                match maximum_pressure {
                    Some(value) => match value.parse::<i128>() {
                        Ok(value) => {
                            if value < i32::MIN as i128 || value > i32::MAX as i128 {
                                state_mut.errors_maximum_pressure.push(ApiError::BadRequest(vec![
                                    format!(                                            "The maximum_pressure field must be between {} and {}.",
                                            i32::MIN,
                                            i32::MAX
                                    )
                                ]));
                            } else {
                                state_mut.maximum_pressure = Some(value as i32);
                            }
                        }
                        Err(_) => {
                            state_mut.errors_maximum_pressure.push(ApiError::BadRequest(vec![
                                "The maximum_pressure field must be a valid i32.".to_string()
                            ]));
                        }
                    },
                    None => state_mut.maximum_pressure = None,
                }
            }
            ContainerHorizontalRuleActions::SetItemType(item_type) => {
                state_mut.errors_item_type.clear();
        if item_type.is_none() {
            state_mut.errors_item_type.push(ApiError::BadRequest(vec![
                "The ItemType field is required.".to_string()
             ]));
        }
                state_mut.item_type = item_type;
            }
            ContainerHorizontalRuleActions::SetOtherItemType(other_item_type) => {
                state_mut.errors_other_item_type.clear();
        if other_item_type.is_none() {
            state_mut.errors_other_item_type.push(ApiError::BadRequest(vec![
                "The OtherItemType field is required.".to_string()
             ]));
        }
                state_mut.other_item_type = other_item_type;
            }
        }
        state
    }
}
impl FormBuilder for ContainerHorizontalRuleBuilder {
    type Actions = ContainerHorizontalRuleActions;

    fn has_errors(&self) -> bool {
!self.errors_name.is_empty() || !self.errors_minimum_temperature.is_empty() || !self.errors_maximum_temperature.is_empty() || !self.errors_minimum_humidity.is_empty() || !self.errors_maximum_humidity.is_empty() || !self.errors_minimum_pressure.is_empty() || !self.errors_maximum_pressure.is_empty() || !self.errors_item_type.is_empty() || !self.errors_other_item_type.is_empty()
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.name.is_some()
        && self.item_type.is_some()
        && self.other_item_type.is_some()
    }

}

impl From<ContainerHorizontalRuleBuilder> for NewContainerHorizontalRule {
    fn from(builder: ContainerHorizontalRuleBuilder) -> Self {
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
impl From<ContainerHorizontalRuleBuilder> for UpdateContainerHorizontalRule {
    fn from(builder: ContainerHorizontalRuleBuilder) -> Self {
        Self {
            id: builder.id.unwrap(),
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
    type Builder = ContainerHorizontalRuleBuilder;
    const TABLE: Table = Table::ContainerHorizontalRules;
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

impl FormBuildable for UpdateContainerHorizontalRule {
    type Builder = ContainerHorizontalRuleBuilder;
    const TABLE: Table = Table::ContainerHorizontalRules;
    fn title() -> &'static str {
        "UpdateContainerHorizontalRule"
    }
    fn task_target() -> &'static str {
        "UpdateContainerHorizontalRule"
    }
    fn description() -> &'static str {
        concat!("Create a new UpdateContainerHorizontalRule.",)
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[function_component(CreateContainerHorizontalRuleForm)]
pub fn create_container_horizontal_rule_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<ContainerHorizontalRuleBuilder>();
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| ContainerHorizontalRuleActions::SetName(name));    let set_minimum_temperature = builder_dispatch.apply_callback(|minimum_temperature: Option<String>| ContainerHorizontalRuleActions::SetMinimumTemperature(minimum_temperature));    let set_maximum_temperature = builder_dispatch.apply_callback(|maximum_temperature: Option<String>| ContainerHorizontalRuleActions::SetMaximumTemperature(maximum_temperature));    let set_minimum_humidity = builder_dispatch.apply_callback(|minimum_humidity: Option<String>| ContainerHorizontalRuleActions::SetMinimumHumidity(minimum_humidity));    let set_maximum_humidity = builder_dispatch.apply_callback(|maximum_humidity: Option<String>| ContainerHorizontalRuleActions::SetMaximumHumidity(maximum_humidity));    let set_minimum_pressure = builder_dispatch.apply_callback(|minimum_pressure: Option<String>| ContainerHorizontalRuleActions::SetMinimumPressure(minimum_pressure));    let set_maximum_pressure = builder_dispatch.apply_callback(|maximum_pressure: Option<String>| ContainerHorizontalRuleActions::SetMaximumPressure(maximum_pressure));    let set_item_type = builder_dispatch.apply_callback(|item_type: Option<NestedItemCategory>| ContainerHorizontalRuleActions::SetItemType(item_type));
    let set_other_item_type = builder_dispatch.apply_callback(|other_item_type: Option<NestedItemCategory>| ContainerHorizontalRuleActions::SetOtherItemType(other_item_type));
    html! {
        <BasicForm<NewContainerHorizontalRule> method={FormMethod::POST} builder={builder_store.deref().clone()}>
            <BasicInput<String> label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<i32> label="MinimumTemperature" errors={builder_store.errors_minimum_temperature.clone()} builder={set_minimum_temperature} value={builder_store.minimum_temperature.clone()} />
            <BasicInput<i32> label="MaximumTemperature" errors={builder_store.errors_maximum_temperature.clone()} builder={set_maximum_temperature} value={builder_store.maximum_temperature.clone()} />
            <BasicInput<i32> label="MinimumHumidity" errors={builder_store.errors_minimum_humidity.clone()} builder={set_minimum_humidity} value={builder_store.minimum_humidity.clone()} />
            <BasicInput<i32> label="MaximumHumidity" errors={builder_store.errors_maximum_humidity.clone()} builder={set_maximum_humidity} value={builder_store.maximum_humidity.clone()} />
            <BasicInput<i32> label="MinimumPressure" errors={builder_store.errors_minimum_pressure.clone()} builder={set_minimum_pressure} value={builder_store.minimum_pressure.clone()} />
            <BasicInput<i32> label="MaximumPressure" errors={builder_store.errors_maximum_pressure.clone()} builder={set_maximum_pressure} value={builder_store.maximum_pressure.clone()} />
            <Datalist<NestedItemCategory> builder={set_item_type} errors={builder_store.errors_item_type.clone()} value={builder_store.item_type.clone()} label="ItemType" />
            <Datalist<NestedItemCategory> builder={set_other_item_type} errors={builder_store.errors_other_item_type.clone()} value={builder_store.other_item_type.clone()} label="OtherItemType" />
        </BasicForm<NewContainerHorizontalRule>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateContainerHorizontalRuleFormProp {
    pub id: i32,
}

#[function_component(UpdateContainerHorizontalRuleForm)]
pub fn update_container_horizontal_rule_form(props: &UpdateContainerHorizontalRuleFormProp) -> Html {
    let (builder_store, builder_dispatch) = use_store::<ContainerHorizontalRuleBuilder>();
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| ContainerHorizontalRuleActions::SetName(name));    let set_minimum_temperature = builder_dispatch.apply_callback(|minimum_temperature: Option<String>| ContainerHorizontalRuleActions::SetMinimumTemperature(minimum_temperature));    let set_maximum_temperature = builder_dispatch.apply_callback(|maximum_temperature: Option<String>| ContainerHorizontalRuleActions::SetMaximumTemperature(maximum_temperature));    let set_minimum_humidity = builder_dispatch.apply_callback(|minimum_humidity: Option<String>| ContainerHorizontalRuleActions::SetMinimumHumidity(minimum_humidity));    let set_maximum_humidity = builder_dispatch.apply_callback(|maximum_humidity: Option<String>| ContainerHorizontalRuleActions::SetMaximumHumidity(maximum_humidity));    let set_minimum_pressure = builder_dispatch.apply_callback(|minimum_pressure: Option<String>| ContainerHorizontalRuleActions::SetMinimumPressure(minimum_pressure));    let set_maximum_pressure = builder_dispatch.apply_callback(|maximum_pressure: Option<String>| ContainerHorizontalRuleActions::SetMaximumPressure(maximum_pressure));    let set_item_type = builder_dispatch.apply_callback(|item_type: Option<NestedItemCategory>| ContainerHorizontalRuleActions::SetItemType(item_type));
    let set_other_item_type = builder_dispatch.apply_callback(|other_item_type: Option<NestedItemCategory>| ContainerHorizontalRuleActions::SetOtherItemType(other_item_type));
    html! {
        <BasicForm<UpdateContainerHorizontalRule> method={FormMethod::PUT} builder={builder_store.deref().clone()}>
            <BasicInput<String> label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<i32> label="MinimumTemperature" errors={builder_store.errors_minimum_temperature.clone()} builder={set_minimum_temperature} value={builder_store.minimum_temperature.clone()} />
            <BasicInput<i32> label="MaximumTemperature" errors={builder_store.errors_maximum_temperature.clone()} builder={set_maximum_temperature} value={builder_store.maximum_temperature.clone()} />
            <BasicInput<i32> label="MinimumHumidity" errors={builder_store.errors_minimum_humidity.clone()} builder={set_minimum_humidity} value={builder_store.minimum_humidity.clone()} />
            <BasicInput<i32> label="MaximumHumidity" errors={builder_store.errors_maximum_humidity.clone()} builder={set_maximum_humidity} value={builder_store.maximum_humidity.clone()} />
            <BasicInput<i32> label="MinimumPressure" errors={builder_store.errors_minimum_pressure.clone()} builder={set_minimum_pressure} value={builder_store.minimum_pressure.clone()} />
            <BasicInput<i32> label="MaximumPressure" errors={builder_store.errors_maximum_pressure.clone()} builder={set_maximum_pressure} value={builder_store.maximum_pressure.clone()} />
            <Datalist<NestedItemCategory> builder={set_item_type} errors={builder_store.errors_item_type.clone()} value={builder_store.item_type.clone()} label="ItemType" />
            <Datalist<NestedItemCategory> builder={set_other_item_type} errors={builder_store.errors_other_item_type.clone()} value={builder_store.other_item_type.clone()} label="OtherItemType" />
        </BasicForm<UpdateContainerHorizontalRule>>
    }
}
#[derive(Store, Eq, PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
#[store(storage = "local", storage_tab_sync)]
pub struct ContainerVerticalRuleBuilder {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub minimum_temperature: Option<i32>,
    pub maximum_temperature: Option<i32>,
    pub minimum_humidity: Option<i32>,
    pub maximum_humidity: Option<i32>,
    pub minimum_pressure: Option<i32>,
    pub maximum_pressure: Option<i32>,
    pub container_item_type: Option<NestedItemCategory>,
    pub contained_item_type: Option<NestedItemCategory>,
    pub errors_name: Vec<ApiError>,
    pub errors_minimum_temperature: Vec<ApiError>,
    pub errors_maximum_temperature: Vec<ApiError>,
    pub errors_minimum_humidity: Vec<ApiError>,
    pub errors_maximum_humidity: Vec<ApiError>,
    pub errors_minimum_pressure: Vec<ApiError>,
    pub errors_maximum_pressure: Vec<ApiError>,
    pub errors_container_item_type: Vec<ApiError>,
    pub errors_contained_item_type: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum ContainerVerticalRuleActions {
    SetName(Option<String>),
    SetMinimumTemperature(Option<String>),
    SetMaximumTemperature(Option<String>),
    SetMinimumHumidity(Option<String>),
    SetMaximumHumidity(Option<String>),
    SetMinimumPressure(Option<String>),
    SetMaximumPressure(Option<String>),
    SetContainerItemType(Option<NestedItemCategory>),
    SetContainedItemType(Option<NestedItemCategory>),
}

impl Reducer<ContainerVerticalRuleBuilder> for ContainerVerticalRuleActions {
    fn apply(self, mut state: std::rc::Rc<ContainerVerticalRuleBuilder>) -> std::rc::Rc<ContainerVerticalRuleBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        state_mut.form_updated_at = chrono::Utc::now().naive_utc();
        match self {
            ContainerVerticalRuleActions::SetName(name) => {
                state_mut.errors_name.clear();
        if name.is_none() {
            state_mut.errors_name.push(ApiError::BadRequest(vec![
                "The Name field is required.".to_string()
             ]));
        }
                state_mut.name = name;
            }
            ContainerVerticalRuleActions::SetMinimumTemperature(minimum_temperature) => {
                state_mut.errors_minimum_temperature.clear();
                match minimum_temperature {
                    Some(value) => match value.parse::<i128>() {
                        Ok(value) => {
                            if value < i32::MIN as i128 || value > i32::MAX as i128 {
                                state_mut.errors_minimum_temperature.push(ApiError::BadRequest(vec![
                                    format!(                                            "The minimum_temperature field must be between {} and {}.",
                                            i32::MIN,
                                            i32::MAX
                                    )
                                ]));
                            } else {
                                state_mut.minimum_temperature = Some(value as i32);
                            }
                        }
                        Err(_) => {
                            state_mut.errors_minimum_temperature.push(ApiError::BadRequest(vec![
                                "The minimum_temperature field must be a valid i32.".to_string()
                            ]));
                        }
                    },
                    None => state_mut.minimum_temperature = None,
                }
            }
            ContainerVerticalRuleActions::SetMaximumTemperature(maximum_temperature) => {
                state_mut.errors_maximum_temperature.clear();
                match maximum_temperature {
                    Some(value) => match value.parse::<i128>() {
                        Ok(value) => {
                            if value < i32::MIN as i128 || value > i32::MAX as i128 {
                                state_mut.errors_maximum_temperature.push(ApiError::BadRequest(vec![
                                    format!(                                            "The maximum_temperature field must be between {} and {}.",
                                            i32::MIN,
                                            i32::MAX
                                    )
                                ]));
                            } else {
                                state_mut.maximum_temperature = Some(value as i32);
                            }
                        }
                        Err(_) => {
                            state_mut.errors_maximum_temperature.push(ApiError::BadRequest(vec![
                                "The maximum_temperature field must be a valid i32.".to_string()
                            ]));
                        }
                    },
                    None => state_mut.maximum_temperature = None,
                }
            }
            ContainerVerticalRuleActions::SetMinimumHumidity(minimum_humidity) => {
                state_mut.errors_minimum_humidity.clear();
                match minimum_humidity {
                    Some(value) => match value.parse::<i128>() {
                        Ok(value) => {
                            if value < i32::MIN as i128 || value > i32::MAX as i128 {
                                state_mut.errors_minimum_humidity.push(ApiError::BadRequest(vec![
                                    format!(                                            "The minimum_humidity field must be between {} and {}.",
                                            i32::MIN,
                                            i32::MAX
                                    )
                                ]));
                            } else {
                                state_mut.minimum_humidity = Some(value as i32);
                            }
                        }
                        Err(_) => {
                            state_mut.errors_minimum_humidity.push(ApiError::BadRequest(vec![
                                "The minimum_humidity field must be a valid i32.".to_string()
                            ]));
                        }
                    },
                    None => state_mut.minimum_humidity = None,
                }
            }
            ContainerVerticalRuleActions::SetMaximumHumidity(maximum_humidity) => {
                state_mut.errors_maximum_humidity.clear();
                match maximum_humidity {
                    Some(value) => match value.parse::<i128>() {
                        Ok(value) => {
                            if value < i32::MIN as i128 || value > i32::MAX as i128 {
                                state_mut.errors_maximum_humidity.push(ApiError::BadRequest(vec![
                                    format!(                                            "The maximum_humidity field must be between {} and {}.",
                                            i32::MIN,
                                            i32::MAX
                                    )
                                ]));
                            } else {
                                state_mut.maximum_humidity = Some(value as i32);
                            }
                        }
                        Err(_) => {
                            state_mut.errors_maximum_humidity.push(ApiError::BadRequest(vec![
                                "The maximum_humidity field must be a valid i32.".to_string()
                            ]));
                        }
                    },
                    None => state_mut.maximum_humidity = None,
                }
            }
            ContainerVerticalRuleActions::SetMinimumPressure(minimum_pressure) => {
                state_mut.errors_minimum_pressure.clear();
                match minimum_pressure {
                    Some(value) => match value.parse::<i128>() {
                        Ok(value) => {
                            if value < i32::MIN as i128 || value > i32::MAX as i128 {
                                state_mut.errors_minimum_pressure.push(ApiError::BadRequest(vec![
                                    format!(                                            "The minimum_pressure field must be between {} and {}.",
                                            i32::MIN,
                                            i32::MAX
                                    )
                                ]));
                            } else {
                                state_mut.minimum_pressure = Some(value as i32);
                            }
                        }
                        Err(_) => {
                            state_mut.errors_minimum_pressure.push(ApiError::BadRequest(vec![
                                "The minimum_pressure field must be a valid i32.".to_string()
                            ]));
                        }
                    },
                    None => state_mut.minimum_pressure = None,
                }
            }
            ContainerVerticalRuleActions::SetMaximumPressure(maximum_pressure) => {
                state_mut.errors_maximum_pressure.clear();
                match maximum_pressure {
                    Some(value) => match value.parse::<i128>() {
                        Ok(value) => {
                            if value < i32::MIN as i128 || value > i32::MAX as i128 {
                                state_mut.errors_maximum_pressure.push(ApiError::BadRequest(vec![
                                    format!(                                            "The maximum_pressure field must be between {} and {}.",
                                            i32::MIN,
                                            i32::MAX
                                    )
                                ]));
                            } else {
                                state_mut.maximum_pressure = Some(value as i32);
                            }
                        }
                        Err(_) => {
                            state_mut.errors_maximum_pressure.push(ApiError::BadRequest(vec![
                                "The maximum_pressure field must be a valid i32.".to_string()
                            ]));
                        }
                    },
                    None => state_mut.maximum_pressure = None,
                }
            }
            ContainerVerticalRuleActions::SetContainerItemType(container_item_type) => {
                state_mut.errors_container_item_type.clear();
        if container_item_type.is_none() {
            state_mut.errors_container_item_type.push(ApiError::BadRequest(vec![
                "The ContainerItemType field is required.".to_string()
             ]));
        }
                state_mut.container_item_type = container_item_type;
            }
            ContainerVerticalRuleActions::SetContainedItemType(contained_item_type) => {
                state_mut.errors_contained_item_type.clear();
        if contained_item_type.is_none() {
            state_mut.errors_contained_item_type.push(ApiError::BadRequest(vec![
                "The ContainedItemType field is required.".to_string()
             ]));
        }
                state_mut.contained_item_type = contained_item_type;
            }
        }
        state
    }
}
impl FormBuilder for ContainerVerticalRuleBuilder {
    type Actions = ContainerVerticalRuleActions;

    fn has_errors(&self) -> bool {
!self.errors_name.is_empty() || !self.errors_minimum_temperature.is_empty() || !self.errors_maximum_temperature.is_empty() || !self.errors_minimum_humidity.is_empty() || !self.errors_maximum_humidity.is_empty() || !self.errors_minimum_pressure.is_empty() || !self.errors_maximum_pressure.is_empty() || !self.errors_container_item_type.is_empty() || !self.errors_contained_item_type.is_empty()
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.name.is_some()
        && self.container_item_type.is_some()
        && self.contained_item_type.is_some()
    }

}

impl From<ContainerVerticalRuleBuilder> for NewContainerVerticalRule {
    fn from(builder: ContainerVerticalRuleBuilder) -> Self {
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
impl From<ContainerVerticalRuleBuilder> for UpdateContainerVerticalRule {
    fn from(builder: ContainerVerticalRuleBuilder) -> Self {
        Self {
            id: builder.id.unwrap(),
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
    type Builder = ContainerVerticalRuleBuilder;
    const TABLE: Table = Table::ContainerVerticalRules;
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

impl FormBuildable for UpdateContainerVerticalRule {
    type Builder = ContainerVerticalRuleBuilder;
    const TABLE: Table = Table::ContainerVerticalRules;
    fn title() -> &'static str {
        "UpdateContainerVerticalRule"
    }
    fn task_target() -> &'static str {
        "UpdateContainerVerticalRule"
    }
    fn description() -> &'static str {
        concat!("Create a new UpdateContainerVerticalRule.",)
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[function_component(CreateContainerVerticalRuleForm)]
pub fn create_container_vertical_rule_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<ContainerVerticalRuleBuilder>();
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| ContainerVerticalRuleActions::SetName(name));    let set_minimum_temperature = builder_dispatch.apply_callback(|minimum_temperature: Option<String>| ContainerVerticalRuleActions::SetMinimumTemperature(minimum_temperature));    let set_maximum_temperature = builder_dispatch.apply_callback(|maximum_temperature: Option<String>| ContainerVerticalRuleActions::SetMaximumTemperature(maximum_temperature));    let set_minimum_humidity = builder_dispatch.apply_callback(|minimum_humidity: Option<String>| ContainerVerticalRuleActions::SetMinimumHumidity(minimum_humidity));    let set_maximum_humidity = builder_dispatch.apply_callback(|maximum_humidity: Option<String>| ContainerVerticalRuleActions::SetMaximumHumidity(maximum_humidity));    let set_minimum_pressure = builder_dispatch.apply_callback(|minimum_pressure: Option<String>| ContainerVerticalRuleActions::SetMinimumPressure(minimum_pressure));    let set_maximum_pressure = builder_dispatch.apply_callback(|maximum_pressure: Option<String>| ContainerVerticalRuleActions::SetMaximumPressure(maximum_pressure));    let set_container_item_type = builder_dispatch.apply_callback(|container_item_type: Option<NestedItemCategory>| ContainerVerticalRuleActions::SetContainerItemType(container_item_type));
    let set_contained_item_type = builder_dispatch.apply_callback(|contained_item_type: Option<NestedItemCategory>| ContainerVerticalRuleActions::SetContainedItemType(contained_item_type));
    html! {
        <BasicForm<NewContainerVerticalRule> method={FormMethod::POST} builder={builder_store.deref().clone()}>
            <BasicInput<String> label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<i32> label="MinimumTemperature" errors={builder_store.errors_minimum_temperature.clone()} builder={set_minimum_temperature} value={builder_store.minimum_temperature.clone()} />
            <BasicInput<i32> label="MaximumTemperature" errors={builder_store.errors_maximum_temperature.clone()} builder={set_maximum_temperature} value={builder_store.maximum_temperature.clone()} />
            <BasicInput<i32> label="MinimumHumidity" errors={builder_store.errors_minimum_humidity.clone()} builder={set_minimum_humidity} value={builder_store.minimum_humidity.clone()} />
            <BasicInput<i32> label="MaximumHumidity" errors={builder_store.errors_maximum_humidity.clone()} builder={set_maximum_humidity} value={builder_store.maximum_humidity.clone()} />
            <BasicInput<i32> label="MinimumPressure" errors={builder_store.errors_minimum_pressure.clone()} builder={set_minimum_pressure} value={builder_store.minimum_pressure.clone()} />
            <BasicInput<i32> label="MaximumPressure" errors={builder_store.errors_maximum_pressure.clone()} builder={set_maximum_pressure} value={builder_store.maximum_pressure.clone()} />
            <Datalist<NestedItemCategory> builder={set_container_item_type} errors={builder_store.errors_container_item_type.clone()} value={builder_store.container_item_type.clone()} label="ContainerItemType" />
            <Datalist<NestedItemCategory> builder={set_contained_item_type} errors={builder_store.errors_contained_item_type.clone()} value={builder_store.contained_item_type.clone()} label="ContainedItemType" />
        </BasicForm<NewContainerVerticalRule>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateContainerVerticalRuleFormProp {
    pub id: i32,
}

#[function_component(UpdateContainerVerticalRuleForm)]
pub fn update_container_vertical_rule_form(props: &UpdateContainerVerticalRuleFormProp) -> Html {
    let (builder_store, builder_dispatch) = use_store::<ContainerVerticalRuleBuilder>();
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| ContainerVerticalRuleActions::SetName(name));    let set_minimum_temperature = builder_dispatch.apply_callback(|minimum_temperature: Option<String>| ContainerVerticalRuleActions::SetMinimumTemperature(minimum_temperature));    let set_maximum_temperature = builder_dispatch.apply_callback(|maximum_temperature: Option<String>| ContainerVerticalRuleActions::SetMaximumTemperature(maximum_temperature));    let set_minimum_humidity = builder_dispatch.apply_callback(|minimum_humidity: Option<String>| ContainerVerticalRuleActions::SetMinimumHumidity(minimum_humidity));    let set_maximum_humidity = builder_dispatch.apply_callback(|maximum_humidity: Option<String>| ContainerVerticalRuleActions::SetMaximumHumidity(maximum_humidity));    let set_minimum_pressure = builder_dispatch.apply_callback(|minimum_pressure: Option<String>| ContainerVerticalRuleActions::SetMinimumPressure(minimum_pressure));    let set_maximum_pressure = builder_dispatch.apply_callback(|maximum_pressure: Option<String>| ContainerVerticalRuleActions::SetMaximumPressure(maximum_pressure));    let set_container_item_type = builder_dispatch.apply_callback(|container_item_type: Option<NestedItemCategory>| ContainerVerticalRuleActions::SetContainerItemType(container_item_type));
    let set_contained_item_type = builder_dispatch.apply_callback(|contained_item_type: Option<NestedItemCategory>| ContainerVerticalRuleActions::SetContainedItemType(contained_item_type));
    html! {
        <BasicForm<UpdateContainerVerticalRule> method={FormMethod::PUT} builder={builder_store.deref().clone()}>
            <BasicInput<String> label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<i32> label="MinimumTemperature" errors={builder_store.errors_minimum_temperature.clone()} builder={set_minimum_temperature} value={builder_store.minimum_temperature.clone()} />
            <BasicInput<i32> label="MaximumTemperature" errors={builder_store.errors_maximum_temperature.clone()} builder={set_maximum_temperature} value={builder_store.maximum_temperature.clone()} />
            <BasicInput<i32> label="MinimumHumidity" errors={builder_store.errors_minimum_humidity.clone()} builder={set_minimum_humidity} value={builder_store.minimum_humidity.clone()} />
            <BasicInput<i32> label="MaximumHumidity" errors={builder_store.errors_maximum_humidity.clone()} builder={set_maximum_humidity} value={builder_store.maximum_humidity.clone()} />
            <BasicInput<i32> label="MinimumPressure" errors={builder_store.errors_minimum_pressure.clone()} builder={set_minimum_pressure} value={builder_store.minimum_pressure.clone()} />
            <BasicInput<i32> label="MaximumPressure" errors={builder_store.errors_maximum_pressure.clone()} builder={set_maximum_pressure} value={builder_store.maximum_pressure.clone()} />
            <Datalist<NestedItemCategory> builder={set_container_item_type} errors={builder_store.errors_container_item_type.clone()} value={builder_store.container_item_type.clone()} label="ContainerItemType" />
            <Datalist<NestedItemCategory> builder={set_contained_item_type} errors={builder_store.errors_contained_item_type.clone()} value={builder_store.contained_item_type.clone()} label="ContainedItemType" />
        </BasicForm<UpdateContainerVerticalRule>>
    }
}
#[derive(Store, Eq, PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
#[store(storage = "local", storage_tab_sync)]
pub struct ItemCategoryBuilder {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub errors_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum ItemCategoryActions {
    SetName(Option<String>),
    SetDescription(Option<String>),
}

impl Reducer<ItemCategoryBuilder> for ItemCategoryActions {
    fn apply(self, mut state: std::rc::Rc<ItemCategoryBuilder>) -> std::rc::Rc<ItemCategoryBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        state_mut.form_updated_at = chrono::Utc::now().naive_utc();
        match self {
            ItemCategoryActions::SetName(name) => {
                state_mut.errors_name.clear();
        if name.is_none() {
            state_mut.errors_name.push(ApiError::BadRequest(vec![
                "The Name field is required.".to_string()
             ]));
        }
                state_mut.name = name;
            }
            ItemCategoryActions::SetDescription(description) => {
                state_mut.errors_description.clear();
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
impl FormBuilder for ItemCategoryBuilder {
    type Actions = ItemCategoryActions;

    fn has_errors(&self) -> bool {
!self.errors_name.is_empty() || !self.errors_description.is_empty()
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.name.is_some()
        && self.description.is_some()
    }

}

impl From<ItemCategoryBuilder> for NewItemCategory {
    fn from(builder: ItemCategoryBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            description: builder.description.unwrap(),
        }
    }
}
impl From<ItemCategoryBuilder> for UpdateItemCategory {
    fn from(builder: ItemCategoryBuilder) -> Self {
        Self {
            id: builder.id.unwrap(),
            name: builder.name.unwrap(),
            description: builder.description.unwrap(),
        }
    }
}
impl FormBuildable for NewItemCategory {
    type Builder = ItemCategoryBuilder;
    const TABLE: Table = Table::ItemCategories;
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

impl FormBuildable for UpdateItemCategory {
    type Builder = ItemCategoryBuilder;
    const TABLE: Table = Table::ItemCategories;
    fn title() -> &'static str {
        "UpdateItemCategory"
    }
    fn task_target() -> &'static str {
        "UpdateItemCategory"
    }
    fn description() -> &'static str {
        concat!("Create a new UpdateItemCategory.",)
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[function_component(CreateItemCategoryForm)]
pub fn create_item_category_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<ItemCategoryBuilder>();
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| ItemCategoryActions::SetName(name));    let set_description = builder_dispatch.apply_callback(|description: Option<String>| ItemCategoryActions::SetDescription(description));    html! {
        <BasicForm<NewItemCategory> method={FormMethod::POST} builder={builder_store.deref().clone()}>
            <BasicInput<String> label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<String> label="Description" errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
        </BasicForm<NewItemCategory>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateItemCategoryFormProp {
    pub id: i32,
}

#[function_component(UpdateItemCategoryForm)]
pub fn update_item_category_form(props: &UpdateItemCategoryFormProp) -> Html {
    let (builder_store, builder_dispatch) = use_store::<ItemCategoryBuilder>();
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| ItemCategoryActions::SetName(name));    let set_description = builder_dispatch.apply_callback(|description: Option<String>| ItemCategoryActions::SetDescription(description));    html! {
        <BasicForm<UpdateItemCategory> method={FormMethod::PUT} builder={builder_store.deref().clone()}>
            <BasicInput<String> label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<String> label="Description" errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
        </BasicForm<UpdateItemCategory>>
    }
}
#[derive(Store, Eq, PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
#[store(storage = "local", storage_tab_sync)]
pub struct ProcedureBuilder {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub errors_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum ProcedureActions {
    SetName(Option<String>),
    SetDescription(Option<String>),
}

impl Reducer<ProcedureBuilder> for ProcedureActions {
    fn apply(self, mut state: std::rc::Rc<ProcedureBuilder>) -> std::rc::Rc<ProcedureBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        state_mut.form_updated_at = chrono::Utc::now().naive_utc();
        match self {
            ProcedureActions::SetName(name) => {
                state_mut.errors_name.clear();
        if name.is_none() {
            state_mut.errors_name.push(ApiError::BadRequest(vec![
                "The Name field is required.".to_string()
             ]));
        }
                state_mut.name = name;
            }
            ProcedureActions::SetDescription(description) => {
                state_mut.errors_description.clear();
                state_mut.description = description;
            }
        }
        state
    }
}
impl FormBuilder for ProcedureBuilder {
    type Actions = ProcedureActions;

    fn has_errors(&self) -> bool {
!self.errors_name.is_empty() || !self.errors_description.is_empty()
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.name.is_some()
    }

}

impl From<ProcedureBuilder> for NewProcedure {
    fn from(builder: ProcedureBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            description: builder.description,
        }
    }
}
impl From<ProcedureBuilder> for UpdateProcedure {
    fn from(builder: ProcedureBuilder) -> Self {
        Self {
            id: builder.id.unwrap(),
            name: builder.name.unwrap(),
            description: builder.description,
        }
    }
}
impl FormBuildable for NewProcedure {
    type Builder = ProcedureBuilder;
    const TABLE: Table = Table::Procedures;
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

impl FormBuildable for UpdateProcedure {
    type Builder = ProcedureBuilder;
    const TABLE: Table = Table::Procedures;
    fn title() -> &'static str {
        "UpdateProcedure"
    }
    fn task_target() -> &'static str {
        "UpdateProcedure"
    }
    fn description() -> &'static str {
        concat!("Create a new UpdateProcedure.",)
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[function_component(CreateProcedureForm)]
pub fn create_procedure_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<ProcedureBuilder>();
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| ProcedureActions::SetName(name));    let set_description = builder_dispatch.apply_callback(|description: Option<String>| ProcedureActions::SetDescription(description));    html! {
        <BasicForm<NewProcedure> method={FormMethod::POST} builder={builder_store.deref().clone()}>
            <BasicInput<String> label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<String> label="Description" errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
        </BasicForm<NewProcedure>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateProcedureFormProp {
    pub id: i32,
}

#[function_component(UpdateProcedureForm)]
pub fn update_procedure_form(props: &UpdateProcedureFormProp) -> Html {
    let (builder_store, builder_dispatch) = use_store::<ProcedureBuilder>();
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| ProcedureActions::SetName(name));    let set_description = builder_dispatch.apply_callback(|description: Option<String>| ProcedureActions::SetDescription(description));    html! {
        <BasicForm<UpdateProcedure> method={FormMethod::PUT} builder={builder_store.deref().clone()}>
            <BasicInput<String> label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<String> label="Description" errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
        </BasicForm<UpdateProcedure>>
    }
}
#[derive(Store, PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
#[store(storage = "local", storage_tab_sync)]
pub struct ProjectRequirementBuilder {
    pub id: Option<i32>,
    pub quantity: Option<i32>,
    pub project: Option<NestedProject>,
    pub item_category: Option<NestedItemCategory>,
    pub unit: Option<Unit>,
    pub errors_quantity: Vec<ApiError>,
    pub errors_project: Vec<ApiError>,
    pub errors_item_category: Vec<ApiError>,
    pub errors_unit: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum ProjectRequirementActions {
    SetQuantity(Option<String>),
    SetProject(Option<NestedProject>),
    SetItemCategory(Option<NestedItemCategory>),
    SetUnit(Option<Unit>),
}

impl Reducer<ProjectRequirementBuilder> for ProjectRequirementActions {
    fn apply(self, mut state: std::rc::Rc<ProjectRequirementBuilder>) -> std::rc::Rc<ProjectRequirementBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        state_mut.form_updated_at = chrono::Utc::now().naive_utc();
        match self {
            ProjectRequirementActions::SetQuantity(quantity) => {
                state_mut.errors_quantity.clear();
        if quantity.is_none() {
            state_mut.errors_quantity.push(ApiError::BadRequest(vec![
                "The Quantity field is required.".to_string()
             ]));
        }
                match quantity {
                    Some(value) => match value.parse::<i128>() {
                        Ok(value) => {
                            if value < i32::MIN as i128 || value > i32::MAX as i128 {
                                state_mut.errors_quantity.push(ApiError::BadRequest(vec![
                                    format!(                                            "The quantity field must be between {} and {}.",
                                            i32::MIN,
                                            i32::MAX
                                    )
                                ]));
                            } else {
                                state_mut.quantity = Some(value as i32);
                            }
                        }
                        Err(_) => {
                            state_mut.errors_quantity.push(ApiError::BadRequest(vec![
                                "The quantity field must be a valid i32.".to_string()
                            ]));
                        }
                    },
                    None => state_mut.quantity = None,
                }
            }
            ProjectRequirementActions::SetProject(project) => {
                state_mut.errors_project.clear();
        if project.is_none() {
            state_mut.errors_project.push(ApiError::BadRequest(vec![
                "The Project field is required.".to_string()
             ]));
        }
                state_mut.project = project;
            }
            ProjectRequirementActions::SetItemCategory(item_category) => {
                state_mut.errors_item_category.clear();
        if item_category.is_none() {
            state_mut.errors_item_category.push(ApiError::BadRequest(vec![
                "The ItemCategory field is required.".to_string()
             ]));
        }
                state_mut.item_category = item_category;
            }
            ProjectRequirementActions::SetUnit(unit) => {
                state_mut.errors_unit.clear();
                state_mut.unit = unit;
            }
        }
        state
    }
}
impl FormBuilder for ProjectRequirementBuilder {
    type Actions = ProjectRequirementActions;

    fn has_errors(&self) -> bool {
!self.errors_quantity.is_empty() || !self.errors_project.is_empty() || !self.errors_item_category.is_empty() || !self.errors_unit.is_empty()
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.quantity.is_some()
        && self.project.is_some()
        && self.item_category.is_some()
    }

}

impl From<ProjectRequirementBuilder> for NewProjectRequirement {
    fn from(builder: ProjectRequirementBuilder) -> Self {
        Self {
            project_id: builder.project.unwrap().inner.id,
            item_category_id: builder.item_category.unwrap().inner.id,
            quantity: builder.quantity.unwrap(),
            unit_id: builder.unit.map(|unit| unit.id),
        }
    }
}
impl From<ProjectRequirementBuilder> for UpdateProjectRequirement {
    fn from(builder: ProjectRequirementBuilder) -> Self {
        Self {
            id: builder.id.unwrap(),
            project_id: builder.project.unwrap().inner.id,
            item_category_id: builder.item_category.unwrap().inner.id,
            quantity: builder.quantity.unwrap(),
            unit_id: builder.unit.map(|unit| unit.id),
        }
    }
}
impl FormBuildable for NewProjectRequirement {
    type Builder = ProjectRequirementBuilder;
    const TABLE: Table = Table::ProjectRequirements;
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

impl FormBuildable for UpdateProjectRequirement {
    type Builder = ProjectRequirementBuilder;
    const TABLE: Table = Table::ProjectRequirements;
    fn title() -> &'static str {
        "UpdateProjectRequirement"
    }
    fn task_target() -> &'static str {
        "UpdateProjectRequirement"
    }
    fn description() -> &'static str {
        concat!("Create a new UpdateProjectRequirement.",)
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[function_component(CreateProjectRequirementForm)]
pub fn create_project_requirement_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<ProjectRequirementBuilder>();
    let set_quantity = builder_dispatch.apply_callback(|quantity: Option<String>| ProjectRequirementActions::SetQuantity(quantity));    let set_project = builder_dispatch.apply_callback(|project: Option<NestedProject>| ProjectRequirementActions::SetProject(project));
    let set_item_category = builder_dispatch.apply_callback(|item_category: Option<NestedItemCategory>| ProjectRequirementActions::SetItemCategory(item_category));
    let set_unit = builder_dispatch.apply_callback(|unit: Option<Unit>| ProjectRequirementActions::SetUnit(unit));
    html! {
        <BasicForm<NewProjectRequirement> method={FormMethod::POST} builder={builder_store.deref().clone()}>
            <BasicInput<i32> label="Quantity" errors={builder_store.errors_quantity.clone()} builder={set_quantity} value={builder_store.quantity.clone()} />
            <Datalist<NestedProject> builder={set_project} errors={builder_store.errors_project.clone()} value={builder_store.project.clone()} label="Project" />
            <Datalist<NestedItemCategory> builder={set_item_category} errors={builder_store.errors_item_category.clone()} value={builder_store.item_category.clone()} label="ItemCategory" />
            <Datalist<Unit> builder={set_unit} errors={builder_store.errors_unit.clone()} value={builder_store.unit.clone()} label="Unit" />
        </BasicForm<NewProjectRequirement>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateProjectRequirementFormProp {
    pub id: i32,
}

#[function_component(UpdateProjectRequirementForm)]
pub fn update_project_requirement_form(props: &UpdateProjectRequirementFormProp) -> Html {
    let (builder_store, builder_dispatch) = use_store::<ProjectRequirementBuilder>();
    let set_quantity = builder_dispatch.apply_callback(|quantity: Option<String>| ProjectRequirementActions::SetQuantity(quantity));    let set_project = builder_dispatch.apply_callback(|project: Option<NestedProject>| ProjectRequirementActions::SetProject(project));
    let set_item_category = builder_dispatch.apply_callback(|item_category: Option<NestedItemCategory>| ProjectRequirementActions::SetItemCategory(item_category));
    let set_unit = builder_dispatch.apply_callback(|unit: Option<Unit>| ProjectRequirementActions::SetUnit(unit));
    html! {
        <BasicForm<UpdateProjectRequirement> method={FormMethod::PUT} builder={builder_store.deref().clone()}>
            <BasicInput<i32> label="Quantity" errors={builder_store.errors_quantity.clone()} builder={set_quantity} value={builder_store.quantity.clone()} />
            <Datalist<NestedProject> builder={set_project} errors={builder_store.errors_project.clone()} value={builder_store.project.clone()} label="Project" />
            <Datalist<NestedItemCategory> builder={set_item_category} errors={builder_store.errors_item_category.clone()} value={builder_store.item_category.clone()} label="ItemCategory" />
            <Datalist<Unit> builder={set_unit} errors={builder_store.errors_unit.clone()} value={builder_store.unit.clone()} label="Unit" />
        </BasicForm<UpdateProjectRequirement>>
    }
}
#[derive(Store, PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
#[store(storage = "local", storage_tab_sync)]
pub struct ProjectBuilder {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub public: Option<bool>,
    pub budget: Option<f64>,
    pub expenses: Option<f64>,
    pub expected_end_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub state: Option<NestedProjectState>,
    pub parent_project: Option<NestedProject>,
    pub errors_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
    pub errors_public: Vec<ApiError>,
    pub errors_budget: Vec<ApiError>,
    pub errors_expenses: Vec<ApiError>,
    pub errors_expected_end_date: Vec<ApiError>,
    pub errors_end_date: Vec<ApiError>,
    pub errors_state: Vec<ApiError>,
    pub errors_parent_project: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum ProjectActions {
    SetName(Option<String>),
    SetDescription(Option<String>),
    SetPublic(Option<bool>),
    SetBudget(Option<String>),
    SetExpenses(Option<String>),
    SetExpectedEndDate(Option<NaiveDateTime>),
    SetEndDate(Option<NaiveDateTime>),
    SetState(Option<NestedProjectState>),
    SetParentProject(Option<NestedProject>),
}

impl Reducer<ProjectBuilder> for ProjectActions {
    fn apply(self, mut state: std::rc::Rc<ProjectBuilder>) -> std::rc::Rc<ProjectBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        state_mut.form_updated_at = chrono::Utc::now().naive_utc();
        match self {
            ProjectActions::SetName(name) => {
                state_mut.errors_name.clear();
        if name.is_none() {
            state_mut.errors_name.push(ApiError::BadRequest(vec![
                "The Name field is required.".to_string()
             ]));
        }
                state_mut.name = name;
            }
            ProjectActions::SetDescription(description) => {
                state_mut.errors_description.clear();
        if description.is_none() {
            state_mut.errors_description.push(ApiError::BadRequest(vec![
                "The Description field is required.".to_string()
             ]));
        }
                state_mut.description = description;
            }
            ProjectActions::SetPublic(public) => {
                state_mut.errors_public.clear();
        if public.is_none() {
            state_mut.errors_public.push(ApiError::BadRequest(vec![
                "The Public field is required.".to_string()
             ]));
        }
                state_mut.public = public;
            }
            ProjectActions::SetBudget(budget) => {
                state_mut.errors_budget.clear();
                match budget {
                    Some(value) => match value.parse::<f64>() {
                        Ok(value) => {
                            if value.is_nan() || value.is_infinite() {
                                state_mut.errors_budget.push(ApiError::BadRequest(vec![
                                    "The budget field must be a valid f64.".to_string()
                                ]));
                            } else                             if value < f64::MIN as f64 || value > f64::MAX as f64 {
                                state_mut.errors_budget.push(ApiError::BadRequest(vec![
                                    format!(                                            "The budget field must be between {} and {}.",
                                            f64::MIN,
                                            f64::MAX
                                    )
                                ]));
                            } else {
                                state_mut.budget = Some(value as f64);
                            }
                        }
                        Err(_) => {
                            state_mut.errors_budget.push(ApiError::BadRequest(vec![
                                "The budget field must be a valid f64.".to_string()
                            ]));
                        }
                    },
                    None => state_mut.budget = None,
                }
            }
            ProjectActions::SetExpenses(expenses) => {
                state_mut.errors_expenses.clear();
                match expenses {
                    Some(value) => match value.parse::<f64>() {
                        Ok(value) => {
                            if value.is_nan() || value.is_infinite() {
                                state_mut.errors_expenses.push(ApiError::BadRequest(vec![
                                    "The expenses field must be a valid f64.".to_string()
                                ]));
                            } else                             if value < f64::MIN as f64 || value > f64::MAX as f64 {
                                state_mut.errors_expenses.push(ApiError::BadRequest(vec![
                                    format!(                                            "The expenses field must be between {} and {}.",
                                            f64::MIN,
                                            f64::MAX
                                    )
                                ]));
                            } else {
                                state_mut.expenses = Some(value as f64);
                            }
                        }
                        Err(_) => {
                            state_mut.errors_expenses.push(ApiError::BadRequest(vec![
                                "The expenses field must be a valid f64.".to_string()
                            ]));
                        }
                    },
                    None => state_mut.expenses = None,
                }
            }
            ProjectActions::SetExpectedEndDate(expected_end_date) => {
                state_mut.errors_expected_end_date.clear();
                state_mut.expected_end_date = expected_end_date;
            }
            ProjectActions::SetEndDate(end_date) => {
                state_mut.errors_end_date.clear();
                state_mut.end_date = end_date;
            }
            ProjectActions::SetState(state) => {
                state_mut.errors_state.clear();
        if state.is_none() {
            state_mut.errors_state.push(ApiError::BadRequest(vec![
                "The State field is required.".to_string()
             ]));
        }
                state_mut.state = state;
            }
            ProjectActions::SetParentProject(parent_project) => {
                state_mut.errors_parent_project.clear();
                state_mut.parent_project = parent_project;
            }
        }
        state
    }
}
impl FormBuilder for ProjectBuilder {
    type Actions = ProjectActions;

    fn has_errors(&self) -> bool {
!self.errors_name.is_empty() || !self.errors_description.is_empty() || !self.errors_public.is_empty() || !self.errors_budget.is_empty() || !self.errors_expenses.is_empty() || !self.errors_expected_end_date.is_empty() || !self.errors_end_date.is_empty() || !self.errors_state.is_empty() || !self.errors_parent_project.is_empty()
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.name.is_some()
        && self.description.is_some()
        && self.public.is_some()
        && self.state.is_some()
    }

}

impl From<ProjectBuilder> for NewProject {
    fn from(builder: ProjectBuilder) -> Self {
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
impl From<ProjectBuilder> for UpdateProject {
    fn from(builder: ProjectBuilder) -> Self {
        Self {
            id: builder.id.unwrap(),
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
    type Builder = ProjectBuilder;
    const TABLE: Table = Table::Projects;
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

impl FormBuildable for UpdateProject {
    type Builder = ProjectBuilder;
    const TABLE: Table = Table::Projects;
    fn title() -> &'static str {
        "UpdateProject"
    }
    fn task_target() -> &'static str {
        "UpdateProject"
    }
    fn description() -> &'static str {
        concat!("Create a new UpdateProject.",)
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[function_component(CreateProjectForm)]
pub fn create_project_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<ProjectBuilder>();
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| ProjectActions::SetName(name));    let set_description = builder_dispatch.apply_callback(|description: Option<String>| ProjectActions::SetDescription(description));    let set_public = builder_dispatch.apply_callback(|public: bool| ProjectActions::SetPublic(Some(public)));
    let set_budget = builder_dispatch.apply_callback(|budget: Option<String>| ProjectActions::SetBudget(budget));    let set_expenses = builder_dispatch.apply_callback(|expenses: Option<String>| ProjectActions::SetExpenses(expenses));    let set_expected_end_date = builder_dispatch.apply_callback(|expected_end_date: Option<NaiveDateTime>| ProjectActions::SetExpectedEndDate(expected_end_date));
    let set_end_date = builder_dispatch.apply_callback(|end_date: Option<NaiveDateTime>| ProjectActions::SetEndDate(end_date));
    let set_state = builder_dispatch.apply_callback(|state: Option<NestedProjectState>| ProjectActions::SetState(state));
    let set_parent_project = builder_dispatch.apply_callback(|parent_project: Option<NestedProject>| ProjectActions::SetParentProject(parent_project));
    html! {
        <BasicForm<NewProject> method={FormMethod::POST} builder={builder_store.deref().clone()}>
            <BasicInput<String> label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<String> label="Description" errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
            <Checkbox label="Public" errors={builder_store.errors_public.clone()} builder={set_public} value={builder_store.public.unwrap_or(false)} />
            <BasicInput<f64> label="Budget" errors={builder_store.errors_budget.clone()} builder={set_budget} value={builder_store.budget.clone()} />
            <BasicInput<f64> label="Expenses" errors={builder_store.errors_expenses.clone()} builder={set_expenses} value={builder_store.expenses.clone()} />
            <Datalist<NestedProjectState> builder={set_state} errors={builder_store.errors_state.clone()} value={builder_store.state.clone()} label="State" />
            <Datalist<NestedProject> builder={set_parent_project} errors={builder_store.errors_parent_project.clone()} value={builder_store.parent_project.clone()} label="ParentProject" />
        </BasicForm<NewProject>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateProjectFormProp {
    pub id: i32,
}

#[function_component(UpdateProjectForm)]
pub fn update_project_form(props: &UpdateProjectFormProp) -> Html {
    let (builder_store, builder_dispatch) = use_store::<ProjectBuilder>();
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| ProjectActions::SetName(name));    let set_description = builder_dispatch.apply_callback(|description: Option<String>| ProjectActions::SetDescription(description));    let set_public = builder_dispatch.apply_callback(|public: bool| ProjectActions::SetPublic(Some(public)));
    let set_budget = builder_dispatch.apply_callback(|budget: Option<String>| ProjectActions::SetBudget(budget));    let set_expenses = builder_dispatch.apply_callback(|expenses: Option<String>| ProjectActions::SetExpenses(expenses));    let set_expected_end_date = builder_dispatch.apply_callback(|expected_end_date: Option<NaiveDateTime>| ProjectActions::SetExpectedEndDate(expected_end_date));
    let set_end_date = builder_dispatch.apply_callback(|end_date: Option<NaiveDateTime>| ProjectActions::SetEndDate(end_date));
    let set_state = builder_dispatch.apply_callback(|state: Option<NestedProjectState>| ProjectActions::SetState(state));
    let set_parent_project = builder_dispatch.apply_callback(|parent_project: Option<NestedProject>| ProjectActions::SetParentProject(parent_project));
    html! {
        <BasicForm<UpdateProject> method={FormMethod::PUT} builder={builder_store.deref().clone()}>
            <BasicInput<String> label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<String> label="Description" errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
            <Checkbox label="Public" errors={builder_store.errors_public.clone()} builder={set_public} value={builder_store.public.unwrap_or(false)} />
            <BasicInput<f64> label="Budget" errors={builder_store.errors_budget.clone()} builder={set_budget} value={builder_store.budget.clone()} />
            <BasicInput<f64> label="Expenses" errors={builder_store.errors_expenses.clone()} builder={set_expenses} value={builder_store.expenses.clone()} />
            <Datalist<NestedProjectState> builder={set_state} errors={builder_store.errors_state.clone()} value={builder_store.state.clone()} label="State" />
            <Datalist<NestedProject> builder={set_parent_project} errors={builder_store.errors_parent_project.clone()} value={builder_store.parent_project.clone()} label="ParentProject" />
        </BasicForm<UpdateProject>>
    }
}
#[derive(Store, Eq, PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
#[store(storage = "local", storage_tab_sync)]
pub struct SampledIndividualBuilder {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub tagged: Option<bool>,
    pub errors_name: Vec<ApiError>,
    pub errors_tagged: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum SampledIndividualActions {
    SetName(Option<String>),
    SetTagged(Option<bool>),
}

impl Reducer<SampledIndividualBuilder> for SampledIndividualActions {
    fn apply(self, mut state: std::rc::Rc<SampledIndividualBuilder>) -> std::rc::Rc<SampledIndividualBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        state_mut.form_updated_at = chrono::Utc::now().naive_utc();
        match self {
            SampledIndividualActions::SetName(name) => {
                state_mut.errors_name.clear();
                state_mut.name = name;
            }
            SampledIndividualActions::SetTagged(tagged) => {
                state_mut.errors_tagged.clear();
        if tagged.is_none() {
            state_mut.errors_tagged.push(ApiError::BadRequest(vec![
                "The Tagged field is required.".to_string()
             ]));
        }
                state_mut.tagged = tagged;
            }
        }
        state
    }
}
impl FormBuilder for SampledIndividualBuilder {
    type Actions = SampledIndividualActions;

    fn has_errors(&self) -> bool {
!self.errors_name.is_empty() || !self.errors_tagged.is_empty()
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.tagged.is_some()
    }

}

impl From<SampledIndividualBuilder> for NewSampledIndividual {
    fn from(builder: SampledIndividualBuilder) -> Self {
        Self {
            id: builder.id.unwrap_or_else(Uuid::new_v4),
            name: builder.name,
            tagged: builder.tagged.unwrap(),
        }
    }
}
impl FormBuildable for NewSampledIndividual {
    type Builder = SampledIndividualBuilder;
    const TABLE: Table = Table::SampledIndividuals;
    fn title() -> &'static str {
        "NewSampledIndividual"
    }
    fn task_target() -> &'static str {
        "NewSampledIndividual"
    }
    fn description() -> &'static str {
        concat!("Create a new NewSampledIndividual.",)
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[function_component(CreateSampledIndividualForm)]
pub fn create_sampled_individual_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<SampledIndividualBuilder>();
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| SampledIndividualActions::SetName(name));    let set_tagged = builder_dispatch.apply_callback(|tagged: bool| SampledIndividualActions::SetTagged(Some(tagged)));
    html! {
        <BasicForm<NewSampledIndividual> method={FormMethod::POST} builder={builder_store.deref().clone()}>
            <BasicInput<String> label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <Checkbox label="Tagged" errors={builder_store.errors_tagged.clone()} builder={set_tagged} value={builder_store.tagged.unwrap_or(false)} />
        </BasicForm<NewSampledIndividual>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateSampledIndividualFormProp {
    pub id: Uuid,
}

#[function_component(UpdateSampledIndividualForm)]
pub fn update_sampled_individual_form(props: &UpdateSampledIndividualFormProp) -> Html {
    let (builder_store, builder_dispatch) = use_store::<SampledIndividualBuilder>();
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| SampledIndividualActions::SetName(name));    let set_tagged = builder_dispatch.apply_callback(|tagged: bool| SampledIndividualActions::SetTagged(Some(tagged)));
    html! {
        <BasicForm<NewSampledIndividual> method={FormMethod::PUT} builder={builder_store.deref().clone()}>
            <BasicInput<String> label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <Checkbox label="Tagged" errors={builder_store.errors_tagged.clone()} builder={set_tagged} value={builder_store.tagged.unwrap_or(false)} />
        </BasicForm<NewSampledIndividual>>
    }
}
#[derive(Store, Eq, PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
#[store(storage = "local", storage_tab_sync)]
pub struct SampleBuilder {
    pub id: Option<Uuid>,
    pub sampled_by: Option<User>,
    pub procedure: Option<NestedSamplingProcedure>,
    pub state: Option<NestedSampleState>,
    pub errors_sampled_by: Vec<ApiError>,
    pub errors_procedure: Vec<ApiError>,
    pub errors_state: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum SampleActions {
    SetSampledBy(Option<User>),
    SetProcedure(Option<NestedSamplingProcedure>),
    SetState(Option<NestedSampleState>),
}

impl Reducer<SampleBuilder> for SampleActions {
    fn apply(self, mut state: std::rc::Rc<SampleBuilder>) -> std::rc::Rc<SampleBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        state_mut.form_updated_at = chrono::Utc::now().naive_utc();
        match self {
            SampleActions::SetSampledBy(sampled_by) => {
                state_mut.errors_sampled_by.clear();
        if sampled_by.is_none() {
            state_mut.errors_sampled_by.push(ApiError::BadRequest(vec![
                "The SampledBy field is required.".to_string()
             ]));
        }
                state_mut.sampled_by = sampled_by;
            }
            SampleActions::SetProcedure(procedure) => {
                state_mut.errors_procedure.clear();
        if procedure.is_none() {
            state_mut.errors_procedure.push(ApiError::BadRequest(vec![
                "The Procedure field is required.".to_string()
             ]));
        }
                state_mut.procedure = procedure;
            }
            SampleActions::SetState(state) => {
                state_mut.errors_state.clear();
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
impl FormBuilder for SampleBuilder {
    type Actions = SampleActions;

    fn has_errors(&self) -> bool {
!self.errors_sampled_by.is_empty() || !self.errors_procedure.is_empty() || !self.errors_state.is_empty()
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.sampled_by.is_some()
        && self.procedure.is_some()
        && self.state.is_some()
    }

}

impl From<SampleBuilder> for NewSample {
    fn from(builder: SampleBuilder) -> Self {
        Self {
            id: builder.id.unwrap_or_else(Uuid::new_v4),
            sampled_by: builder.sampled_by.unwrap().id,
            procedure_id: builder.procedure.unwrap().inner.id,
            state: builder.state.unwrap().inner.id,
        }
    }
}
impl FormBuildable for NewSample {
    type Builder = SampleBuilder;
    const TABLE: Table = Table::Samples;
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

#[function_component(CreateSampleForm)]
pub fn create_sample_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<SampleBuilder>();
    let set_sampled_by = builder_dispatch.apply_callback(|sampled_by: Option<User>| SampleActions::SetSampledBy(sampled_by));
    let set_procedure = builder_dispatch.apply_callback(|procedure: Option<NestedSamplingProcedure>| SampleActions::SetProcedure(procedure));
    let set_state = builder_dispatch.apply_callback(|state: Option<NestedSampleState>| SampleActions::SetState(state));
    html! {
        <BasicForm<NewSample> method={FormMethod::POST} builder={builder_store.deref().clone()}>
            <Datalist<User> builder={set_sampled_by} errors={builder_store.errors_sampled_by.clone()} value={builder_store.sampled_by.clone()} label="SampledBy" />
            <Datalist<NestedSamplingProcedure> builder={set_procedure} errors={builder_store.errors_procedure.clone()} value={builder_store.procedure.clone()} label="Procedure" />
            <Datalist<NestedSampleState> builder={set_state} errors={builder_store.errors_state.clone()} value={builder_store.state.clone()} label="State" />
        </BasicForm<NewSample>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateSampleFormProp {
    pub id: Uuid,
}

#[function_component(UpdateSampleForm)]
pub fn update_sample_form(props: &UpdateSampleFormProp) -> Html {
    let (builder_store, builder_dispatch) = use_store::<SampleBuilder>();
    let set_sampled_by = builder_dispatch.apply_callback(|sampled_by: Option<User>| SampleActions::SetSampledBy(sampled_by));
    let set_procedure = builder_dispatch.apply_callback(|procedure: Option<NestedSamplingProcedure>| SampleActions::SetProcedure(procedure));
    let set_state = builder_dispatch.apply_callback(|state: Option<NestedSampleState>| SampleActions::SetState(state));
    html! {
        <BasicForm<NewSample> method={FormMethod::PUT} builder={builder_store.deref().clone()}>
            <Datalist<User> builder={set_sampled_by} errors={builder_store.errors_sampled_by.clone()} value={builder_store.sampled_by.clone()} label="SampledBy" />
            <Datalist<NestedSamplingProcedure> builder={set_procedure} errors={builder_store.errors_procedure.clone()} value={builder_store.procedure.clone()} label="Procedure" />
            <Datalist<NestedSampleState> builder={set_state} errors={builder_store.errors_state.clone()} value={builder_store.state.clone()} label="State" />
        </BasicForm<NewSample>>
    }
}
#[derive(Store, Eq, PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
#[store(storage = "local", storage_tab_sync)]
pub struct SamplingProcedureBuilder {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub errors_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum SamplingProcedureActions {
    SetName(Option<String>),
    SetDescription(Option<String>),
}

impl Reducer<SamplingProcedureBuilder> for SamplingProcedureActions {
    fn apply(self, mut state: std::rc::Rc<SamplingProcedureBuilder>) -> std::rc::Rc<SamplingProcedureBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        state_mut.form_updated_at = chrono::Utc::now().naive_utc();
        match self {
            SamplingProcedureActions::SetName(name) => {
                state_mut.errors_name.clear();
        if name.is_none() {
            state_mut.errors_name.push(ApiError::BadRequest(vec![
                "The Name field is required.".to_string()
             ]));
        }
                state_mut.name = name;
            }
            SamplingProcedureActions::SetDescription(description) => {
                state_mut.errors_description.clear();
                state_mut.description = description;
            }
        }
        state
    }
}
impl FormBuilder for SamplingProcedureBuilder {
    type Actions = SamplingProcedureActions;

    fn has_errors(&self) -> bool {
!self.errors_name.is_empty() || !self.errors_description.is_empty()
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.name.is_some()
    }

}

impl From<SamplingProcedureBuilder> for NewSamplingProcedure {
    fn from(builder: SamplingProcedureBuilder) -> Self {
        Self {
            id: builder.id.unwrap_or_else(Uuid::new_v4),
            name: builder.name.unwrap(),
            description: builder.description,
        }
    }
}
impl FormBuildable for NewSamplingProcedure {
    type Builder = SamplingProcedureBuilder;
    const TABLE: Table = Table::SamplingProcedures;
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

#[function_component(CreateSamplingProcedureForm)]
pub fn create_sampling_procedure_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<SamplingProcedureBuilder>();
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| SamplingProcedureActions::SetName(name));    let set_description = builder_dispatch.apply_callback(|description: Option<String>| SamplingProcedureActions::SetDescription(description));    html! {
        <BasicForm<NewSamplingProcedure> method={FormMethod::POST} builder={builder_store.deref().clone()}>
            <BasicInput<String> label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<String> label="Description" errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
        </BasicForm<NewSamplingProcedure>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateSamplingProcedureFormProp {
    pub id: Uuid,
}

#[function_component(UpdateSamplingProcedureForm)]
pub fn update_sampling_procedure_form(props: &UpdateSamplingProcedureFormProp) -> Html {
    let (builder_store, builder_dispatch) = use_store::<SamplingProcedureBuilder>();
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| SamplingProcedureActions::SetName(name));    let set_description = builder_dispatch.apply_callback(|description: Option<String>| SamplingProcedureActions::SetDescription(description));    html! {
        <BasicForm<NewSamplingProcedure> method={FormMethod::PUT} builder={builder_store.deref().clone()}>
            <BasicInput<String> label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<String> label="Description" errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
        </BasicForm<NewSamplingProcedure>>
    }
}
#[derive(Store, Eq, PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
#[store(storage = "local", storage_tab_sync)]
pub struct TeamBuilder {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub parent_team: Option<NestedTeam>,
    pub errors_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
    pub errors_parent_team: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum TeamActions {
    SetName(Option<String>),
    SetDescription(Option<String>),
    SetParentTeam(Option<NestedTeam>),
}

impl Reducer<TeamBuilder> for TeamActions {
    fn apply(self, mut state: std::rc::Rc<TeamBuilder>) -> std::rc::Rc<TeamBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        state_mut.form_updated_at = chrono::Utc::now().naive_utc();
        match self {
            TeamActions::SetName(name) => {
                state_mut.errors_name.clear();
        if name.is_none() {
            state_mut.errors_name.push(ApiError::BadRequest(vec![
                "The Name field is required.".to_string()
             ]));
        }
                state_mut.name = name;
            }
            TeamActions::SetDescription(description) => {
                state_mut.errors_description.clear();
        if description.is_none() {
            state_mut.errors_description.push(ApiError::BadRequest(vec![
                "The Description field is required.".to_string()
             ]));
        }
                state_mut.description = description;
            }
            TeamActions::SetParentTeam(parent_team) => {
                state_mut.errors_parent_team.clear();
                state_mut.parent_team = parent_team;
            }
        }
        state
    }
}
impl FormBuilder for TeamBuilder {
    type Actions = TeamActions;

    fn has_errors(&self) -> bool {
!self.errors_name.is_empty() || !self.errors_description.is_empty() || !self.errors_parent_team.is_empty()
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.name.is_some()
        && self.description.is_some()
    }

}

impl From<TeamBuilder> for NewTeam {
    fn from(builder: TeamBuilder) -> Self {
        Self {
            name: builder.name.unwrap(),
            description: builder.description.unwrap(),
            parent_team_id: builder.parent_team.map(|parent_team| parent_team.inner.id),
        }
    }
}
impl From<TeamBuilder> for UpdateTeam {
    fn from(builder: TeamBuilder) -> Self {
        Self {
            id: builder.id.unwrap(),
            name: builder.name.unwrap(),
            description: builder.description.unwrap(),
            parent_team_id: builder.parent_team.map(|parent_team| parent_team.inner.id),
        }
    }
}
impl FormBuildable for NewTeam {
    type Builder = TeamBuilder;
    const TABLE: Table = Table::Teams;
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

impl FormBuildable for UpdateTeam {
    type Builder = TeamBuilder;
    const TABLE: Table = Table::Teams;
    fn title() -> &'static str {
        "UpdateTeam"
    }
    fn task_target() -> &'static str {
        "UpdateTeam"
    }
    fn description() -> &'static str {
        concat!("Create a new UpdateTeam.",)
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[function_component(CreateTeamForm)]
pub fn create_team_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<TeamBuilder>();
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| TeamActions::SetName(name));    let set_description = builder_dispatch.apply_callback(|description: Option<String>| TeamActions::SetDescription(description));    let set_parent_team = builder_dispatch.apply_callback(|parent_team: Option<NestedTeam>| TeamActions::SetParentTeam(parent_team));
    html! {
        <BasicForm<NewTeam> method={FormMethod::POST} builder={builder_store.deref().clone()}>
            <BasicInput<String> label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<String> label="Description" errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
            <Datalist<NestedTeam> builder={set_parent_team} errors={builder_store.errors_parent_team.clone()} value={builder_store.parent_team.clone()} label="ParentTeam" />
        </BasicForm<NewTeam>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateTeamFormProp {
    pub id: i32,
}

#[function_component(UpdateTeamForm)]
pub fn update_team_form(props: &UpdateTeamFormProp) -> Html {
    let (builder_store, builder_dispatch) = use_store::<TeamBuilder>();
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| TeamActions::SetName(name));    let set_description = builder_dispatch.apply_callback(|description: Option<String>| TeamActions::SetDescription(description));    let set_parent_team = builder_dispatch.apply_callback(|parent_team: Option<NestedTeam>| TeamActions::SetParentTeam(parent_team));
    html! {
        <BasicForm<UpdateTeam> method={FormMethod::PUT} builder={builder_store.deref().clone()}>
            <BasicInput<String> label="Name" errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<String> label="Description" errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
            <Datalist<NestedTeam> builder={set_parent_team} errors={builder_store.errors_parent_team.clone()} value={builder_store.parent_team.clone()} label="ParentTeam" />
        </BasicForm<UpdateTeam>>
    }
}
