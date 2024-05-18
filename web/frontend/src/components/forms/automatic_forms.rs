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
use crate::workers::ws_worker::Tabular;
use yewdux::Dispatch;
use chrono::NaiveDateTime;
use web_common::api::ApiError;
use crate::workers::ws_worker::ComponentMessage;
use web_common::custom_validators::Image;
use web_common::file_formats::GenericFileFormat;

#[derive(Store, PartialEq, Debug, Clone, Serialize, Deserialize)]
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

impl Default for ProjectBuilder {
    fn default() -> Self {
        Self {
            id: None,
            name: None,
            description: None,
            public: Some(true),
            budget: None,
            expenses: None,
            expected_end_date: None,
            end_date: None,
            state: None,
            parent_project: None,
            errors_name: Vec::new(),
            errors_description: Vec::new(),
            errors_public: Vec::new(),
            errors_budget: Vec::new(),
            errors_expenses: Vec::new(),
            errors_expected_end_date: Vec::new(),
            errors_end_date: Vec::new(),
            errors_state: Vec::new(),
            errors_parent_project: Vec::new(),
            form_updated_at: <NaiveDateTime>::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum ProjectActions {
    SetName(Option<String>),
    SetDescription(Option<String>),
    SetPublic(Option<bool>),
    SetBudget(Option<String>),
    SetExpenses(Option<String>),
    SetExpectedEndDate(Option<String>),
    SetEndDate(Option<String>),
    SetState(Option<NestedProjectState>),
    SetParentProject(Option<NestedProject>),
}

impl FromOperation for ProjectActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "parent_project" => ProjectActions::SetParentProject(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<ProjectBuilder> for ProjectActions {
    fn apply(self, mut state: std::rc::Rc<ProjectBuilder>) -> std::rc::Rc<ProjectBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            ProjectActions::SetName(name) => 'name: {
                state_mut.errors_name.clear();
        if name.is_none() {
            state_mut.errors_name.push(ApiError::BadRequest(vec![
                "The Name field is required.".to_string()
             ]));
            state_mut.name = None;
             break 'name;
        }
                if let Some(value) = name.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_name.push(ApiError::BadRequest(vec![
                            "The Name field cannot be left empty.".to_string()
                        ]));
                         state_mut.name = None;
                          break 'name;
                    }
                }
                state_mut.name = name;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'name;
            }
            ProjectActions::SetDescription(description) => 'description: {
                state_mut.errors_description.clear();
        if description.is_none() {
            state_mut.errors_description.push(ApiError::BadRequest(vec![
                "The Description field is required.".to_string()
             ]));
            state_mut.description = None;
             break 'description;
        }
                if let Some(value) = description.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_description.push(ApiError::BadRequest(vec![
                            "The Description field cannot be left empty.".to_string()
                        ]));
                         state_mut.description = None;
                          break 'description;
                    }
                }
                state_mut.description = description;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'description;
            }
            ProjectActions::SetPublic(public) => 'public: {
                state_mut.errors_public.clear();
        if public.is_none() {
            state_mut.errors_public.push(ApiError::BadRequest(vec![
                "The Public field is required.".to_string()
             ]));
            state_mut.public = None;
             break 'public;
        }
                state_mut.public = public;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'public;
            }
            ProjectActions::SetBudget(budget) => 'budget: {
                state_mut.errors_budget.clear();
                state_mut.form_updated_at = chrono::Utc::now().naive_utc();
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
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'budget;
            }
            ProjectActions::SetExpenses(expenses) => 'expenses: {
                state_mut.errors_expenses.clear();
                state_mut.form_updated_at = chrono::Utc::now().naive_utc();
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
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'expenses;
            }
            ProjectActions::SetExpectedEndDate(expected_end_date) => 'expected_end_date: {
                state_mut.errors_expected_end_date.clear();
                match expected_end_date {
                    Some(value) => match NaiveDateTime::parse_from_str(&value, "%Y-%m-%dT%H:%M") {
                        Ok(expected_end_date) => state_mut.expected_end_date = Some(expected_end_date),
                        Err(_) => state_mut.errors_expected_end_date.push(ApiError::BadRequest(vec![
                            "The expected_end_date field must be a valid date and time.".to_string()
                        ])),
                    },
                    None => state_mut.expected_end_date = None,
                }
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'expected_end_date;
            }
            ProjectActions::SetEndDate(end_date) => 'end_date: {
                state_mut.errors_end_date.clear();
                match end_date {
                    Some(value) => match NaiveDateTime::parse_from_str(&value, "%Y-%m-%dT%H:%M") {
                        Ok(end_date) => state_mut.end_date = Some(end_date),
                        Err(_) => state_mut.errors_end_date.push(ApiError::BadRequest(vec![
                            "The end_date field must be a valid date and time.".to_string()
                        ])),
                    },
                    None => state_mut.end_date = None,
                }
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'end_date;
            }
            ProjectActions::SetState(state) => 'state: {
                state_mut.errors_state.clear();
        if state.is_none() {
            state_mut.errors_state.push(ApiError::BadRequest(vec![
                "The State field is required.".to_string()
             ]));
            state_mut.state = None;
             break 'state;
        }
                state_mut.state = state;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'state;
            }
            ProjectActions::SetParentProject(parent_project) => 'parent_project: {
                state_mut.errors_parent_project.clear();
                match parent_project.as_ref() {
                    Some(parent_project) => {
                            if state_mut.id.map_or(false, |id| id == parent_project.inner.id)
                        {
                            state_mut.errors_parent_project.push(ApiError::BadRequest(vec![
                                "The Parent project field must be distinct from the current value.".to_string()
                             ]));
                            break 'parent_project;
                        }
                    }
                    None => (),
                }
                state_mut.parent_project = parent_project;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'parent_project;
            }
        }
        state
    }
}
impl FormBuilder for ProjectBuilder {
    type Actions = ProjectActions;

    type RichVariant = NestedProject;

    fn has_errors(&self) -> bool {
!self.errors_name.is_empty() || !self.errors_description.is_empty() || !self.errors_public.is_empty() || !self.errors_budget.is_empty() || !self.errors_expenses.is_empty() || !self.errors_expected_end_date.is_empty() || !self.errors_end_date.is_empty() || !self.errors_state.is_empty() || !self.errors_parent_project.is_empty()
    }

    fn id(&self) -> Option<PrimaryKey> {
        self.id.map(|id| id.into())
    }

    fn update(dispatcher: &Dispatch<Self>, rich_variant: Self::RichVariant) -> Vec<ComponentMessage> {
    dispatcher.apply(ProjectActions::SetName(Some(rich_variant.inner.name.to_string())));
    dispatcher.apply(ProjectActions::SetDescription(Some(rich_variant.inner.description.to_string())));
        dispatcher.apply(ProjectActions::SetPublic(Some(rich_variant.inner.public)));    dispatcher.apply(ProjectActions::SetBudget(rich_variant.inner.budget.map(|budget| budget.to_string())));
    dispatcher.apply(ProjectActions::SetExpenses(rich_variant.inner.expenses.map(|expenses| expenses.to_string())));
    dispatcher.apply(ProjectActions::SetExpectedEndDate(rich_variant.inner.expected_end_date.map(|expected_end_date| expected_end_date.to_string())));
    dispatcher.apply(ProjectActions::SetEndDate(rich_variant.inner.end_date.map(|end_date| end_date.to_string())));
        dispatcher.apply(ProjectActions::SetState(Some(rich_variant.state)));
        vec![ComponentMessage::get_named::<&str, NewProject>("parent_project", rich_variant.inner.id.into())]
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
impl Tabular for NestedProject {
    const TABLE: Table = Table::Projects;
}

impl Tabular for NewProject {
    const TABLE: Table = Table::Projects;
}

impl FormBuildable for NewProject {
    type Builder = ProjectBuilder;
    fn title() -> &'static str {
        "Project"
    }
    fn task_target() -> &'static str {
        "Project"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

impl Tabular for UpdateProject {
    const TABLE: Table = Table::Projects;
}

impl FormBuildable for UpdateProject {
    type Builder = ProjectBuilder;
    fn title() -> &'static str {
        "Project"
    }
    fn task_target() -> &'static str {
        "Project"
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
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| ProjectActions::SetName(name));
    let set_description = builder_dispatch.apply_callback(|description: Option<String>| ProjectActions::SetDescription(description));
    let set_public = builder_dispatch.apply_callback(|public: bool| ProjectActions::SetPublic(Some(public)));
    let set_budget = builder_dispatch.apply_callback(|budget: Option<String>| ProjectActions::SetBudget(budget));
    let set_expenses = builder_dispatch.apply_callback(|expenses: Option<String>| ProjectActions::SetExpenses(expenses));
    let set_expected_end_date = builder_dispatch.apply_callback(|expected_end_date: Option<String>| ProjectActions::SetExpectedEndDate(expected_end_date));
    let set_end_date = builder_dispatch.apply_callback(|end_date: Option<String>| ProjectActions::SetEndDate(end_date));
    let set_state = builder_dispatch.apply_callback(|state: Option<NestedProjectState>| ProjectActions::SetState(state));
    let set_parent_project = builder_dispatch.apply_callback(|parent_project: Option<NestedProject>| ProjectActions::SetParentProject(parent_project));
    html! {
        <BasicForm<NewProject> method={FormMethod::POST} builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Name" optional={false} errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<String> label="Description" optional={false} errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
            <Checkbox label="Public" errors={builder_store.errors_public.clone()} builder={set_public} value={builder_store.public.unwrap_or(false)} />
            <BasicInput<f64> label="Budget" optional={true} errors={builder_store.errors_budget.clone()} builder={set_budget} value={builder_store.budget.clone()} />
            <BasicInput<f64> label="Expenses" optional={true} errors={builder_store.errors_expenses.clone()} builder={set_expenses} value={builder_store.expenses.clone()} />
            <BasicInput<NaiveDateTime> label="Expected end date" optional={true} errors={builder_store.errors_expected_end_date.clone()} builder={set_expected_end_date} value={builder_store.expected_end_date.clone()} />
            <BasicInput<NaiveDateTime> label="End date" optional={true} errors={builder_store.errors_end_date.clone()} builder={set_end_date} value={builder_store.end_date.clone()} />
            <Datalist<NestedProjectState, false> builder={set_state} optional={false} errors={builder_store.errors_state.clone()} value={builder_store.state.clone()} label="State" />
            <Datalist<NestedProject, true> builder={set_parent_project} optional={true} errors={builder_store.errors_parent_project.clone()} value={builder_store.parent_project.clone()} label="Parent project" />
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
     builder_dispatch.reduce_mut(|builder| {
         builder.id = Some(props.id);
     });
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| ProjectActions::SetName(name));
    let set_description = builder_dispatch.apply_callback(|description: Option<String>| ProjectActions::SetDescription(description));
    let set_public = builder_dispatch.apply_callback(|public: bool| ProjectActions::SetPublic(Some(public)));
    let set_budget = builder_dispatch.apply_callback(|budget: Option<String>| ProjectActions::SetBudget(budget));
    let set_expenses = builder_dispatch.apply_callback(|expenses: Option<String>| ProjectActions::SetExpenses(expenses));
    let set_expected_end_date = builder_dispatch.apply_callback(|expected_end_date: Option<String>| ProjectActions::SetExpectedEndDate(expected_end_date));
    let set_end_date = builder_dispatch.apply_callback(|end_date: Option<String>| ProjectActions::SetEndDate(end_date));
    let set_state = builder_dispatch.apply_callback(|state: Option<NestedProjectState>| ProjectActions::SetState(state));
    let set_parent_project = builder_dispatch.apply_callback(|parent_project: Option<NestedProject>| ProjectActions::SetParentProject(parent_project));
    html! {
        <BasicForm<UpdateProject> method={FormMethod::PUT} builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Name" optional={false} errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<String> label="Description" optional={false} errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
            <Checkbox label="Public" errors={builder_store.errors_public.clone()} builder={set_public} value={builder_store.public.unwrap_or(false)} />
            <BasicInput<f64> label="Budget" optional={true} errors={builder_store.errors_budget.clone()} builder={set_budget} value={builder_store.budget.clone()} />
            <BasicInput<f64> label="Expenses" optional={true} errors={builder_store.errors_expenses.clone()} builder={set_expenses} value={builder_store.expenses.clone()} />
            <BasicInput<NaiveDateTime> label="Expected end date" optional={true} errors={builder_store.errors_expected_end_date.clone()} builder={set_expected_end_date} value={builder_store.expected_end_date.clone()} />
            <BasicInput<NaiveDateTime> label="End date" optional={true} errors={builder_store.errors_end_date.clone()} builder={set_end_date} value={builder_store.end_date.clone()} />
            <Datalist<NestedProjectState, false> builder={set_state} optional={false} errors={builder_store.errors_state.clone()} value={builder_store.state.clone()} label="State" />
            <Datalist<NestedProject, true> builder={set_parent_project} optional={true} errors={builder_store.errors_parent_project.clone()} value={builder_store.parent_project.clone()} label="Parent project" />
        </BasicForm<UpdateProject>>
    }
}
#[derive(Store, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct SampledIndividualBuilder {
    pub id: Option<Uuid>,
    pub tagged: Option<bool>,
    pub errors_tagged: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for SampledIndividualBuilder {
    fn default() -> Self {
        Self {
            id: None,
            tagged: Some(false),
            errors_tagged: Vec::new(),
            form_updated_at: <NaiveDateTime>::default(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum SampledIndividualActions {
    SetTagged(Option<bool>),
}

impl FromOperation for SampledIndividualActions {
    fn from_operation<S: AsRef<str>>(_operation: S, _row: Vec<u8>) -> Self {
        unreachable!("No operations are expected to be needed for the builder SampledIndividualBuilder.")
    }
}

impl Reducer<SampledIndividualBuilder> for SampledIndividualActions {
    fn apply(self, mut state: std::rc::Rc<SampledIndividualBuilder>) -> std::rc::Rc<SampledIndividualBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            SampledIndividualActions::SetTagged(tagged) => 'tagged: {
                state_mut.errors_tagged.clear();
        if tagged.is_none() {
            state_mut.errors_tagged.push(ApiError::BadRequest(vec![
                "The Tagged field is required.".to_string()
             ]));
            state_mut.tagged = None;
             break 'tagged;
        }
                state_mut.tagged = tagged;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'tagged;
            }
        }
        state
    }
}
impl FormBuilder for SampledIndividualBuilder {
    type Actions = SampledIndividualActions;

    type RichVariant = NestedSampledIndividual;

    fn has_errors(&self) -> bool {
!self.errors_tagged.is_empty()
    }

    fn id(&self) -> Option<PrimaryKey> {
        self.id.map(|id| id.into())
    }

    fn update(dispatcher: &Dispatch<Self>, rich_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.apply(SampledIndividualActions::SetTagged(Some(rich_variant.inner.tagged)));        Vec::new()
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
            tagged: builder.tagged.unwrap(),
        }
    }
}
impl Tabular for NestedSampledIndividual {
    const TABLE: Table = Table::SampledIndividuals;
}

impl Tabular for NewSampledIndividual {
    const TABLE: Table = Table::SampledIndividuals;
}

impl FormBuildable for NewSampledIndividual {
    type Builder = SampledIndividualBuilder;
    fn title() -> &'static str {
        "Sampled individual"
    }
    fn task_target() -> &'static str {
        "Sampled individual"
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
    let set_tagged = builder_dispatch.apply_callback(|tagged: bool| SampledIndividualActions::SetTagged(Some(tagged)));
    html! {
        <BasicForm<NewSampledIndividual> method={FormMethod::POST} builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
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
     builder_dispatch.reduce_mut(|builder| {
         builder.id = Some(props.id);
     });
    let set_tagged = builder_dispatch.apply_callback(|tagged: bool| SampledIndividualActions::SetTagged(Some(tagged)));
    html! {
        <BasicForm<NewSampledIndividual> method={FormMethod::PUT} builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Checkbox label="Tagged" errors={builder_store.errors_tagged.clone()} builder={set_tagged} value={builder_store.tagged.unwrap_or(false)} />
        </BasicForm<NewSampledIndividual>>
    }
}
#[derive(Store, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct SampleBuilder {
    pub id: Option<Uuid>,
    pub sampled_by: Option<User>,
    pub state: Option<NestedSampleState>,
    pub errors_sampled_by: Vec<ApiError>,
    pub errors_state: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for SampleBuilder {
    fn default() -> Self {
        Self {
            id: None,
            sampled_by: None,
            state: None,
            errors_sampled_by: Vec::new(),
            errors_state: Vec::new(),
            form_updated_at: <NaiveDateTime>::default(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum SampleActions {
    SetSampledBy(Option<User>),
    SetState(Option<NestedSampleState>),
}

impl FromOperation for SampleActions {
    fn from_operation<S: AsRef<str>>(_operation: S, _row: Vec<u8>) -> Self {
        unreachable!("No operations are expected to be needed for the builder SampleBuilder.")
    }
}

impl Reducer<SampleBuilder> for SampleActions {
    fn apply(self, mut state: std::rc::Rc<SampleBuilder>) -> std::rc::Rc<SampleBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            SampleActions::SetSampledBy(sampled_by) => 'sampled_by: {
                state_mut.errors_sampled_by.clear();
        if sampled_by.is_none() {
            state_mut.errors_sampled_by.push(ApiError::BadRequest(vec![
                "The Sampled by field is required.".to_string()
             ]));
            state_mut.sampled_by = None;
             break 'sampled_by;
        }
                state_mut.sampled_by = sampled_by;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'sampled_by;
            }
            SampleActions::SetState(state) => 'state: {
                state_mut.errors_state.clear();
        if state.is_none() {
            state_mut.errors_state.push(ApiError::BadRequest(vec![
                "The State field is required.".to_string()
             ]));
            state_mut.state = None;
             break 'state;
        }
                state_mut.state = state;
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
!self.errors_sampled_by.is_empty() || !self.errors_state.is_empty()
    }

    fn id(&self) -> Option<PrimaryKey> {
        self.id.map(|id| id.into())
    }

    fn update(dispatcher: &Dispatch<Self>, rich_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.apply(SampleActions::SetSampledBy(Some(rich_variant.sampled_by)));
        dispatcher.apply(SampleActions::SetState(Some(rich_variant.state)));
        Vec::new()
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.sampled_by.is_some()
        && self.state.is_some()
    }

}

impl From<SampleBuilder> for NewSample {
    fn from(builder: SampleBuilder) -> Self {
        Self {
            id: builder.id.unwrap_or_else(Uuid::new_v4),
            sampled_by: builder.sampled_by.unwrap().id,
            state: builder.state.unwrap().inner.id,
        }
    }
}
impl Tabular for NestedSample {
    const TABLE: Table = Table::Samples;
}

impl Tabular for NewSample {
    const TABLE: Table = Table::Samples;
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

#[function_component(CreateSampleForm)]
pub fn create_sample_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<SampleBuilder>();
    let set_sampled_by = builder_dispatch.apply_callback(|sampled_by: Option<User>| SampleActions::SetSampledBy(sampled_by));
    let set_state = builder_dispatch.apply_callback(|state: Option<NestedSampleState>| SampleActions::SetState(state));
    html! {
        <BasicForm<NewSample> method={FormMethod::POST} builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Datalist<User, false> builder={set_sampled_by} optional={false} errors={builder_store.errors_sampled_by.clone()} value={builder_store.sampled_by.clone()} label="Sampled by" />
            <Datalist<NestedSampleState, false> builder={set_state} optional={false} errors={builder_store.errors_state.clone()} value={builder_store.state.clone()} label="State" />
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
     builder_dispatch.reduce_mut(|builder| {
         builder.id = Some(props.id);
     });
    let set_sampled_by = builder_dispatch.apply_callback(|sampled_by: Option<User>| SampleActions::SetSampledBy(sampled_by));
    let set_state = builder_dispatch.apply_callback(|state: Option<NestedSampleState>| SampleActions::SetState(state));
    html! {
        <BasicForm<NewSample> method={FormMethod::PUT} builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <Datalist<User, false> builder={set_sampled_by} optional={false} errors={builder_store.errors_sampled_by.clone()} value={builder_store.sampled_by.clone()} label="Sampled by" />
            <Datalist<NestedSampleState, false> builder={set_state} optional={false} errors={builder_store.errors_state.clone()} value={builder_store.state.clone()} label="State" />
        </BasicForm<NewSample>>
    }
}
#[derive(Store, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
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

impl Default for TeamBuilder {
    fn default() -> Self {
        Self {
            id: None,
            name: None,
            description: None,
            parent_team: None,
            errors_name: Vec::new(),
            errors_description: Vec::new(),
            errors_parent_team: Vec::new(),
            form_updated_at: <NaiveDateTime>::default(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub(super) enum TeamActions {
    SetName(Option<String>),
    SetDescription(Option<String>),
    SetParentTeam(Option<NestedTeam>),
}

impl FromOperation for TeamActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "parent_team" => TeamActions::SetParentTeam(Some(bincode::deserialize(&row).unwrap())),
            operation_name => unreachable!("The operation name '{}' is not supported.", operation_name),
        }
    }
}

impl Reducer<TeamBuilder> for TeamActions {
    fn apply(self, mut state: std::rc::Rc<TeamBuilder>) -> std::rc::Rc<TeamBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            TeamActions::SetName(name) => 'name: {
                state_mut.errors_name.clear();
        if name.is_none() {
            state_mut.errors_name.push(ApiError::BadRequest(vec![
                "The Name field is required.".to_string()
             ]));
            state_mut.name = None;
             break 'name;
        }
                if let Some(value) = name.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_name.push(ApiError::BadRequest(vec![
                            "The Name field cannot be left empty.".to_string()
                        ]));
                         state_mut.name = None;
                          break 'name;
                    }
                }
                state_mut.name = name;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'name;
            }
            TeamActions::SetDescription(description) => 'description: {
                state_mut.errors_description.clear();
        if description.is_none() {
            state_mut.errors_description.push(ApiError::BadRequest(vec![
                "The Description field is required.".to_string()
             ]));
            state_mut.description = None;
             break 'description;
        }
                if let Some(value) = description.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_description.push(ApiError::BadRequest(vec![
                            "The Description field cannot be left empty.".to_string()
                        ]));
                         state_mut.description = None;
                          break 'description;
                    }
                }
                state_mut.description = description;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'description;
            }
            TeamActions::SetParentTeam(parent_team) => 'parent_team: {
                state_mut.errors_parent_team.clear();
                match parent_team.as_ref() {
                    Some(parent_team) => {
                            if state_mut.id.map_or(false, |id| id == parent_team.inner.id)
                        {
                            state_mut.errors_parent_team.push(ApiError::BadRequest(vec![
                                "The Parent team field must be distinct from the current value.".to_string()
                             ]));
                            break 'parent_team;
                        }
                    }
                    None => (),
                }
                state_mut.parent_team = parent_team;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'parent_team;
            }
        }
        state
    }
}
impl FormBuilder for TeamBuilder {
    type Actions = TeamActions;

    type RichVariant = NestedTeam;

    fn has_errors(&self) -> bool {
!self.errors_name.is_empty() || !self.errors_description.is_empty() || !self.errors_parent_team.is_empty()
    }

    fn id(&self) -> Option<PrimaryKey> {
        self.id.map(|id| id.into())
    }

    fn update(dispatcher: &Dispatch<Self>, rich_variant: Self::RichVariant) -> Vec<ComponentMessage> {
    dispatcher.apply(TeamActions::SetName(Some(rich_variant.inner.name.to_string())));
    dispatcher.apply(TeamActions::SetDescription(Some(rich_variant.inner.description.to_string())));
        vec![ComponentMessage::get_named::<&str, NewTeam>("parent_team", rich_variant.inner.id.into())]
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
impl Tabular for NestedTeam {
    const TABLE: Table = Table::Teams;
}

impl Tabular for NewTeam {
    const TABLE: Table = Table::Teams;
}

impl FormBuildable for NewTeam {
    type Builder = TeamBuilder;
    fn title() -> &'static str {
        "Team"
    }
    fn task_target() -> &'static str {
        "Team"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

impl Tabular for UpdateTeam {
    const TABLE: Table = Table::Teams;
}

impl FormBuildable for UpdateTeam {
    type Builder = TeamBuilder;
    fn title() -> &'static str {
        "Team"
    }
    fn task_target() -> &'static str {
        "Team"
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
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| TeamActions::SetName(name));
    let set_description = builder_dispatch.apply_callback(|description: Option<String>| TeamActions::SetDescription(description));
    let set_parent_team = builder_dispatch.apply_callback(|parent_team: Option<NestedTeam>| TeamActions::SetParentTeam(parent_team));
    html! {
        <BasicForm<NewTeam> method={FormMethod::POST} builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Name" optional={false} errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<String> label="Description" optional={false} errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
            <Datalist<NestedTeam, true> builder={set_parent_team} optional={true} errors={builder_store.errors_parent_team.clone()} value={builder_store.parent_team.clone()} label="Parent team" />
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
     builder_dispatch.reduce_mut(|builder| {
         builder.id = Some(props.id);
     });
    let set_name = builder_dispatch.apply_callback(|name: Option<String>| TeamActions::SetName(name));
    let set_description = builder_dispatch.apply_callback(|description: Option<String>| TeamActions::SetDescription(description));
    let set_parent_team = builder_dispatch.apply_callback(|parent_team: Option<NestedTeam>| TeamActions::SetParentTeam(parent_team));
    html! {
        <BasicForm<UpdateTeam> method={FormMethod::PUT} builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Name" optional={false} errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<String> label="Description" optional={false} errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
            <Datalist<NestedTeam, true> builder={set_parent_team} optional={true} errors={builder_store.errors_parent_team.clone()} value={builder_store.parent_team.clone()} label="Parent team" />
        </BasicForm<UpdateTeam>>
    }
}
#[derive(Store, Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct UserBuilder {
    pub id: Option<i32>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub profile_picture: Option<Vec<u8>>,
    pub errors_first_name: Vec<ApiError>,
    pub errors_middle_name: Vec<ApiError>,
    pub errors_last_name: Vec<ApiError>,
    pub errors_profile_picture: Vec<ApiError>,
    pub form_updated_at: NaiveDateTime,
}

impl Default for UserBuilder {
    fn default() -> Self {
        Self {
            id: None,
            first_name: None,
            middle_name: None,
            last_name: None,
            profile_picture: None,
            errors_first_name: Vec::new(),
            errors_middle_name: Vec::new(),
            errors_last_name: Vec::new(),
            errors_profile_picture: Vec::new(),
            form_updated_at: <NaiveDateTime>::default(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub(super) enum UserActions {
    SetFirstName(Option<String>),
    SetMiddleName(Option<String>),
    SetLastName(Option<String>),
    SetProfilePicture(Option<Vec<u8>>),
}

impl FromOperation for UserActions {
    fn from_operation<S: AsRef<str>>(_operation: S, _row: Vec<u8>) -> Self {
        unreachable!("No operations are expected to be needed for the builder UserBuilder.")
    }
}

impl Reducer<UserBuilder> for UserActions {
    fn apply(self, mut state: std::rc::Rc<UserBuilder>) -> std::rc::Rc<UserBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            UserActions::SetFirstName(first_name) => 'first_name: {
                state_mut.errors_first_name.clear();
        if first_name.is_none() {
            state_mut.errors_first_name.push(ApiError::BadRequest(vec![
                "The First name field is required.".to_string()
             ]));
            state_mut.first_name = None;
             break 'first_name;
        }
                if let Some(value) = first_name.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_first_name.push(ApiError::BadRequest(vec![
                            "The First name field cannot be left empty.".to_string()
                        ]));
                         state_mut.first_name = None;
                          break 'first_name;
                    }
                }
                state_mut.first_name = first_name;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'first_name;
            }
            UserActions::SetMiddleName(middle_name) => 'middle_name: {
                state_mut.errors_middle_name.clear();
                if let Some(value) = middle_name.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_middle_name.push(ApiError::BadRequest(vec![
                            "The Middle name field cannot be left empty.".to_string()
                        ]));
                         state_mut.middle_name = None;
                          break 'middle_name;
                    }
                }
                state_mut.middle_name = middle_name;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'middle_name;
            }
            UserActions::SetLastName(last_name) => 'last_name: {
                state_mut.errors_last_name.clear();
        if last_name.is_none() {
            state_mut.errors_last_name.push(ApiError::BadRequest(vec![
                "The Last name field is required.".to_string()
             ]));
            state_mut.last_name = None;
             break 'last_name;
        }
                if let Some(value) = last_name.as_ref() {
                    if value.is_empty() {
                        state_mut.errors_last_name.push(ApiError::BadRequest(vec![
                            "The Last name field cannot be left empty.".to_string()
                        ]));
                         state_mut.last_name = None;
                          break 'last_name;
                    }
                }
                state_mut.last_name = last_name;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'last_name;
            }
            UserActions::SetProfilePicture(profile_picture) => 'profile_picture: {
                state_mut.errors_profile_picture.clear();
        if profile_picture.is_none() {
            state_mut.errors_profile_picture.push(ApiError::BadRequest(vec![
                "The Profile picture field is required.".to_string()
             ]));
            state_mut.profile_picture = None;
             break 'profile_picture;
        }
                state_mut.profile_picture = profile_picture;
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'profile_picture;
            }
        }
        state
    }
}
impl FormBuilder for UserBuilder {
    type Actions = UserActions;

    type RichVariant = User;

    fn has_errors(&self) -> bool {
!self.errors_first_name.is_empty() || !self.errors_middle_name.is_empty() || !self.errors_last_name.is_empty() || !self.errors_profile_picture.is_empty()
    }

    fn id(&self) -> Option<PrimaryKey> {
        self.id.map(|id| id.into())
    }

    fn update(dispatcher: &Dispatch<Self>, rich_variant: Self::RichVariant) -> Vec<ComponentMessage> {
        dispatcher.apply(UserActions::SetFirstName(Some(rich_variant.first_name)));
        dispatcher.apply(UserActions::SetMiddleName(rich_variant.middle_name));
        dispatcher.apply(UserActions::SetLastName(Some(rich_variant.last_name)));
        dispatcher.apply(UserActions::SetProfilePicture(Some(rich_variant.profile_picture)));
        Vec::new()
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
        && self.first_name.is_some()
        && self.last_name.is_some()
        && self.profile_picture.is_some()
    }

}

impl From<UserBuilder> for NewUser {
    fn from(builder: UserBuilder) -> Self {
        Self {
            first_name: builder.first_name.unwrap(),
            middle_name: builder.middle_name,
            last_name: builder.last_name.unwrap(),
            profile_picture: builder.profile_picture.unwrap(),
        }
    }
}
impl From<UserBuilder> for UpdateUser {
    fn from(builder: UserBuilder) -> Self {
        Self {
            id: builder.id.unwrap(),
            first_name: builder.first_name.unwrap(),
            middle_name: builder.middle_name,
            last_name: builder.last_name.unwrap(),
            profile_picture: builder.profile_picture.unwrap(),
        }
    }
}
impl Tabular for User {
    const TABLE: Table = Table::Users;
}

impl Tabular for NewUser {
    const TABLE: Table = Table::Users;
}

impl FormBuildable for NewUser {
    type Builder = UserBuilder;
    fn title() -> &'static str {
        "User"
    }
    fn task_target() -> &'static str {
        "User"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        false
    }
}

impl Tabular for UpdateUser {
    const TABLE: Table = Table::Users;
}

impl FormBuildable for UpdateUser {
    type Builder = UserBuilder;
    fn title() -> &'static str {
        "User"
    }
    fn task_target() -> &'static str {
        "User"
    }
    fn requires_authentication() -> bool {
        true
    }
    fn can_operate_offline() -> bool {
        true
    }
}

#[function_component(CreateUserForm)]
pub fn create_user_form() -> Html {
    let (builder_store, builder_dispatch) = use_store::<UserBuilder>();
    let set_first_name = builder_dispatch.apply_callback(|first_name: Option<String>| UserActions::SetFirstName(first_name));
    let set_middle_name = builder_dispatch.apply_callback(|middle_name: Option<String>| UserActions::SetMiddleName(middle_name));
    let set_last_name = builder_dispatch.apply_callback(|last_name: Option<String>| UserActions::SetLastName(last_name));
    let set_profile_picture = builder_dispatch.apply_callback(|profile_picture: Option<Image>| UserActions::SetProfilePicture(profile_picture.map(|profile_picture| profile_picture.into())));
    html! {
        <BasicForm<NewUser> method={FormMethod::POST} builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="First name" optional={false} errors={builder_store.errors_first_name.clone()} builder={set_first_name} value={builder_store.first_name.clone()} />
            <BasicInput<String> label="Middle name" optional={true} errors={builder_store.errors_middle_name.clone()} builder={set_middle_name} value={builder_store.middle_name.clone()} />
            <BasicInput<String> label="Last name" optional={false} errors={builder_store.errors_last_name.clone()} builder={set_last_name} value={builder_store.last_name.clone()} />
            <FileInput<Image> label="Profile picture" optional={false} errors={builder_store.errors_profile_picture.clone()} builder={set_profile_picture} allowed_formats={vec![GenericFileFormat::Image]} value={builder_store.profile_picture.clone().map(|profile_picture| profile_picture.into())} />
        </BasicForm<NewUser>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateUserFormProp {
    pub id: i32,
}

#[function_component(UpdateUserForm)]
pub fn update_user_form(props: &UpdateUserFormProp) -> Html {
    let (builder_store, builder_dispatch) = use_store::<UserBuilder>();
     builder_dispatch.reduce_mut(|builder| {
         builder.id = Some(props.id);
     });
    let set_first_name = builder_dispatch.apply_callback(|first_name: Option<String>| UserActions::SetFirstName(first_name));
    let set_middle_name = builder_dispatch.apply_callback(|middle_name: Option<String>| UserActions::SetMiddleName(middle_name));
    let set_last_name = builder_dispatch.apply_callback(|last_name: Option<String>| UserActions::SetLastName(last_name));
    let set_profile_picture = builder_dispatch.apply_callback(|profile_picture: Option<Image>| UserActions::SetProfilePicture(profile_picture.map(|profile_picture| profile_picture.into())));
    html! {
        <BasicForm<UpdateUser> method={FormMethod::PUT} builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="First name" optional={false} errors={builder_store.errors_first_name.clone()} builder={set_first_name} value={builder_store.first_name.clone()} />
            <BasicInput<String> label="Middle name" optional={true} errors={builder_store.errors_middle_name.clone()} builder={set_middle_name} value={builder_store.middle_name.clone()} />
            <BasicInput<String> label="Last name" optional={false} errors={builder_store.errors_last_name.clone()} builder={set_last_name} value={builder_store.last_name.clone()} />
            <FileInput<Image> label="Profile picture" optional={false} errors={builder_store.errors_profile_picture.clone()} builder={set_profile_picture} allowed_formats={vec![GenericFileFormat::Image]} value={builder_store.profile_picture.clone().map(|profile_picture| profile_picture.into())} />
        </BasicForm<UpdateUser>>
    }
}
