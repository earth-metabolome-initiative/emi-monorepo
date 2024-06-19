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
pub struct ProjectBuilder {
    pub id: Option<i32>,
    pub name: Option<Rc<String>>,
    pub description: Option<Rc<String>>,
    pub public: Option<bool>,
    pub budget: Option<f64>,
    pub expenses: Option<f64>,
    pub expected_end_date: Option<chrono::NaiveDateTime>,
    pub end_date: Option<chrono::NaiveDateTime>,
    pub state: Option<Rc<web_common::database::nested_variants::NestedProjectState>>,
    pub icon: Option<Rc<web_common::database::flat_variants::FontAwesomeIcon>>,
    pub color: Option<Rc<web_common::database::flat_variants::Color>>,
    pub parent_project: Option<Rc<web_common::database::nested_variants::NestedProject>>,
    pub errors_name: Vec<ApiError>,
    pub errors_description: Vec<ApiError>,
    pub errors_public: Vec<ApiError>,
    pub errors_budget: Vec<ApiError>,
    pub errors_expenses: Vec<ApiError>,
    pub errors_expected_end_date: Vec<ApiError>,
    pub errors_end_date: Vec<ApiError>,
    pub errors_state: Vec<ApiError>,
    pub errors_icon: Vec<ApiError>,
    pub errors_color: Vec<ApiError>,
    pub errors_parent_project: Vec<ApiError>,
    pub form_updated_at: chrono::NaiveDateTime,
}

unsafe impl Send for ProjectBuilder {}
unsafe impl Sync for ProjectBuilder {}
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
            state: Default::default(),
            icon: Default::default(),
            color: Default::default(),
            parent_project: Default::default(),
            errors_name: Default::default(),
            errors_description: Default::default(),
            errors_public: Default::default(),
            errors_budget: Default::default(),
            errors_expenses: Default::default(),
            errors_expected_end_date: Default::default(),
            errors_end_date: Default::default(),
            errors_state: Default::default(),
            errors_icon: Default::default(),
            errors_color: Default::default(),
            errors_parent_project: Default::default(),
            form_updated_at: Default::default(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) enum ProjectActions {
    SetName(Option<String>),
    SetDescription(Option<String>),
    SetPublic(Option<bool>),
    SetBudget(Option<String>),
    SetExpenses(Option<String>),
    SetExpectedEndDate(Option<String>),
    SetEndDate(Option<String>),
    SetState(Option<Rc<web_common::database::nested_variants::NestedProjectState>>),
    SetIcon(Option<Rc<web_common::database::flat_variants::FontAwesomeIcon>>),
    SetColor(Option<Rc<web_common::database::flat_variants::Color>>),
    SetParentProject(Option<Rc<web_common::database::nested_variants::NestedProject>>),
}

impl FromOperation for ProjectActions {
    fn from_operation<S: AsRef<str>>(operation: S, row: Vec<u8>) -> Self {
        match operation.as_ref() {
            "state" => ProjectActions::SetState(Some(bincode::deserialize(&row).unwrap())),
            "icon" => ProjectActions::SetIcon(Some(bincode::deserialize(&row).unwrap())),
            "color" => ProjectActions::SetColor(Some(bincode::deserialize(&row).unwrap())),
            "parent_project" => {
                ProjectActions::SetParentProject(Some(bincode::deserialize(&row).unwrap()))
            }
            operation_name => {
                unreachable!("The operation name '{}' is not supported.", operation_name)
            }
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
                        "The Name field is required.".to_string(),
                    ]));
                    state_mut.name = None;
                    break 'name;
                }
                if let Some(value) = name.as_ref() {
                    if value.is_empty() {
                        state_mut
                            .errors_name
                            .push(ApiError::Empty("Name".to_string()));
                        state_mut.name = None;
                        break 'name;
                    }
                }
                state_mut.name = name.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'name;
            }
            ProjectActions::SetDescription(description) => 'description: {
                state_mut.errors_description.clear();
                if description.is_none() {
                    state_mut.errors_description.push(ApiError::BadRequest(vec![
                        "The Description field is required.".to_string(),
                    ]));
                    state_mut.description = None;
                    break 'description;
                }
                if let Some(value) = description.as_ref() {
                    if value.is_empty() {
                        state_mut
                            .errors_description
                            .push(ApiError::Empty("Description".to_string()));
                        state_mut.description = None;
                        break 'description;
                    }
                }
                state_mut.description = description.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'description;
            }
            ProjectActions::SetPublic(public) => 'public: {
                state_mut.errors_public.clear();
                if public.is_none() {
                    state_mut.errors_public.push(ApiError::BadRequest(vec![
                        "The Public field is required.".to_string(),
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
                                    "The Budget field must be a valid f64.".to_string(),
                                ]));
                            } else if value < f64::MIN as f64 || value > f64::MAX as f64 {
                                state_mut
                                    .errors_budget
                                    .push(ApiError::BadRequest(vec![format!(
                                        "The Budget field must be between {} and {}.",
                                        f64::MIN,
                                        f64::MAX
                                    )]));
                            } else {
                                state_mut.budget = Some(value as f64);
                            }
                        }
                        Err(_) => {
                            state_mut.errors_budget.push(ApiError::BadRequest(vec![
                                "The Budget field must be a valid f64.".to_string(),
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
                                    "The Expenses field must be a valid f64.".to_string(),
                                ]));
                            } else if value < f64::MIN as f64 || value > f64::MAX as f64 {
                                state_mut.errors_expenses.push(ApiError::BadRequest(vec![
                                    format!(
                                        "The Expenses field must be between {} and {}.",
                                        f64::MIN,
                                        f64::MAX
                                    ),
                                ]));
                            } else {
                                state_mut.expenses = Some(value as f64);
                            }
                        }
                        Err(_) => {
                            state_mut.errors_expenses.push(ApiError::BadRequest(vec![
                                "The Expenses field must be a valid f64.".to_string(),
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
                    Some(value) => {
                        match chrono::NaiveDateTime::parse_from_str(&value, "%Y-%m-%dT%H:%M") {
                            Ok(expected_end_date) => {
                                state_mut.expected_end_date = Some(expected_end_date)
                            }
                            Err(_) => {
                                match chrono::NaiveDateTime::parse_from_str(
                                    &value,
                                    "%Y-%m-%d %H:%M:%S",
                                ) {
                                    Ok(expected_end_date) => {
                                        state_mut.expected_end_date = Some(expected_end_date)
                                    }
                                    Err(_) => state_mut.errors_expected_end_date.push(
                                        ApiError::BadRequest(vec![
                                            "The Expected end date field must be a valid date."
                                                .to_string(),
                                        ]),
                                    ),
                                }
                            }
                        }
                    }
                    None => state_mut.expected_end_date = None,
                }
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'expected_end_date;
            }
            ProjectActions::SetEndDate(end_date) => 'end_date: {
                state_mut.errors_end_date.clear();
                match end_date {
                    Some(value) => {
                        match chrono::NaiveDateTime::parse_from_str(&value, "%Y-%m-%dT%H:%M") {
                            Ok(end_date) => state_mut.end_date = Some(end_date),
                            Err(_) => match chrono::NaiveDateTime::parse_from_str(
                                &value,
                                "%Y-%m-%d %H:%M:%S",
                            ) {
                                Ok(end_date) => state_mut.end_date = Some(end_date),
                                Err(_) => {
                                    state_mut.errors_end_date.push(ApiError::BadRequest(vec![
                                        "The End date field must be a valid date.".to_string(),
                                    ]))
                                }
                            },
                        }
                    }
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
                        "The State field is required.".to_string(),
                    ]));
                    state_mut.state = None;
                    break 'state;
                }
                state_mut.state = state.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'state;
            }
            ProjectActions::SetIcon(icon) => 'icon: {
                state_mut.errors_icon.clear();
                if icon.is_none() {
                    state_mut.errors_icon.push(ApiError::BadRequest(vec![
                        "The Icon field is required.".to_string(),
                    ]));
                    state_mut.icon = None;
                    break 'icon;
                }
                state_mut.icon = icon.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'icon;
            }
            ProjectActions::SetColor(color) => 'color: {
                state_mut.errors_color.clear();
                if color.is_none() {
                    state_mut.errors_color.push(ApiError::BadRequest(vec![
                        "The Color field is required.".to_string(),
                    ]));
                    state_mut.color = None;
                    break 'color;
                }
                state_mut.color = color.map(Rc::from);
                // To avoid having a codesmell relative to the cases where we are not
                // yet handling more corner cases, we always use the break here.
                break 'color;
            }
            ProjectActions::SetParentProject(parent_project) => 'parent_project: {
                state_mut.errors_parent_project.clear();
                match parent_project.as_ref() {
                    Some(parent_project) => {
                        if state_mut
                            .id
                            .map_or(false, |id| id == parent_project.inner.id)
                        {
                            state_mut
                                .errors_parent_project
                                .push(ApiError::BadRequest(vec![
                                "The Parent project field must be distinct from the current value."
                                    .to_string(),
                            ]));
                            break 'parent_project;
                        }
                    }
                    None => (),
                }
                state_mut.parent_project = parent_project.map(Rc::from);
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
        !self.errors_name.is_empty()
            || !self.errors_description.is_empty()
            || !self.errors_public.is_empty()
            || !self.errors_budget.is_empty()
            || !self.errors_expenses.is_empty()
            || !self.errors_expected_end_date.is_empty()
            || !self.errors_end_date.is_empty()
            || !self.errors_state.is_empty()
            || !self.errors_icon.is_empty()
            || !self.errors_color.is_empty()
            || !self.errors_parent_project.is_empty()
    }

    fn update(
        dispatcher: &Dispatch<Self>,
        richest_variant: Self::RichVariant,
    ) -> Vec<ComponentMessage> {
        dispatcher.reduce_mut(|state| {
            state.id = Some(richest_variant.inner.id);
        });
        dispatcher.apply(ProjectActions::SetName(Some(
            richest_variant.inner.name.to_string(),
        )));
        dispatcher.apply(ProjectActions::SetDescription(Some(
            richest_variant.inner.description.to_string(),
        )));
        dispatcher.apply(ProjectActions::SetPublic(Some(
            richest_variant.inner.public,
        )));
        dispatcher.apply(ProjectActions::SetBudget(
            richest_variant
                .inner
                .budget
                .as_ref()
                .map(|budget| budget.to_string()),
        ));
        dispatcher.apply(ProjectActions::SetExpenses(
            richest_variant
                .inner
                .expenses
                .as_ref()
                .map(|expenses| expenses.to_string()),
        ));
        dispatcher.apply(ProjectActions::SetExpectedEndDate(
            richest_variant
                .inner
                .expected_end_date
                .as_ref()
                .map(|expected_end_date| expected_end_date.to_string()),
        ));
        dispatcher.apply(ProjectActions::SetEndDate(
            richest_variant
                .inner
                .end_date
                .as_ref()
                .map(|end_date| end_date.to_string()),
        ));
        dispatcher.apply(ProjectActions::SetState(
            Some(richest_variant.state).map(Rc::from),
        ));
        dispatcher.apply(ProjectActions::SetIcon(
            Some(richest_variant.icon).map(Rc::from),
        ));
        dispatcher.apply(ProjectActions::SetColor(
            Some(richest_variant.color).map(Rc::from),
        ));
        let mut named_requests = Vec::new();
        if let Some(parent_project_id) = richest_variant.inner.parent_project_id {
            named_requests.push(ComponentMessage::get_named::<&str, Project>(
                "parent_project",
                parent_project_id.into(),
            ));
        } else {
            dispatcher.apply(ProjectActions::SetParentProject(None));
        }
        named_requests
    }

    fn can_submit(&self) -> bool {
        !self.has_errors()
            && self.name.is_some()
            && self.description.is_some()
            && self.public.is_some()
            && self.state.is_some()
            && self.icon.is_some()
            && self.color.is_some()
    }
}

impl From<ProjectBuilder> for NewProject {
    fn from(builder: ProjectBuilder) -> Self {
        Self {
            name: builder.name.as_deref().cloned().unwrap(),
            description: builder.description.as_deref().cloned().unwrap(),
            public: builder.public.unwrap(),
            state_id: builder.state.as_deref().cloned().unwrap().inner.id,
            icon_id: builder.icon.as_deref().cloned().unwrap().id,
            color_id: builder.color.as_deref().cloned().unwrap().id,
            parent_project_id: builder
                .parent_project
                .as_deref()
                .cloned()
                .map(|parent_project| parent_project.inner.id),
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
            name: builder.name.as_deref().cloned().unwrap(),
            description: builder.description.as_deref().cloned().unwrap(),
            public: builder.public.unwrap(),
            state_id: builder.state.as_deref().cloned().unwrap().inner.id,
            icon_id: builder.icon.as_deref().cloned().unwrap().id,
            color_id: builder.color.as_deref().cloned().unwrap().id,
            parent_project_id: builder
                .parent_project
                .as_deref()
                .cloned()
                .map(|parent_project| parent_project.inner.id),
            budget: builder.budget,
            expenses: builder.expenses,
            expected_end_date: builder.expected_end_date,
            end_date: builder.end_date,
        }
    }
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

#[derive(Clone, PartialEq, Properties)]
pub struct CreateProjectFormProp {
    #[prop_or(1)]
    pub state_id: i32,
    #[prop_or(415)]
    pub icon_id: i32,
    #[prop_or(1)]
    pub color_id: i32,
    #[prop_or_default]
    pub parent_project_id: Option<i32>,
}

#[function_component(CreateProjectForm)]
pub fn create_project_form(props: &CreateProjectFormProp) -> Html {
    let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = yewdux::use_store::<ProjectBuilder>();
    named_requests.push(ComponentMessage::get_named::<&str, ProjectState>(
        "state",
        props.state_id.into(),
    ));
    named_requests.push(ComponentMessage::get_named::<&str, FontAwesomeIcon>(
        "icon",
        props.icon_id.into(),
    ));
    named_requests.push(ComponentMessage::get_named::<&str, Color>(
        "color",
        props.color_id.into(),
    ));
    if let Some(parent_project_id) = props.parent_project_id {
        named_requests.push(ComponentMessage::get_named::<&str, Project>(
            "parent_project",
            parent_project_id.into(),
        ));
    }
    let set_name =
        builder_dispatch.apply_callback(|name: Option<String>| ProjectActions::SetName(name));
    let set_description = builder_dispatch
        .apply_callback(|description: Option<String>| ProjectActions::SetDescription(description));
    let set_public =
        builder_dispatch.apply_callback(|public: bool| ProjectActions::SetPublic(Some(public)));
    let set_budget =
        builder_dispatch.apply_callback(|budget: Option<String>| ProjectActions::SetBudget(budget));
    let set_expenses = builder_dispatch
        .apply_callback(|expenses: Option<String>| ProjectActions::SetExpenses(expenses));
    let set_expected_end_date =
        builder_dispatch.apply_callback(|expected_end_date: Option<String>| {
            ProjectActions::SetExpectedEndDate(expected_end_date)
        });
    let set_end_date = builder_dispatch
        .apply_callback(|end_date: Option<String>| ProjectActions::SetEndDate(end_date));
    let set_state = builder_dispatch.apply_callback(
        |state: Option<Rc<web_common::database::nested_variants::NestedProjectState>>| {
            ProjectActions::SetState(state)
        },
    );
    let set_icon = builder_dispatch.apply_callback(
        |icon: Option<Rc<web_common::database::flat_variants::FontAwesomeIcon>>| {
            ProjectActions::SetIcon(icon)
        },
    );
    let set_color = builder_dispatch.apply_callback(
        |color: Option<Rc<web_common::database::flat_variants::Color>>| {
            ProjectActions::SetColor(color)
        },
    );
    let set_parent_project = builder_dispatch.apply_callback(
        |parent_project: Option<Rc<web_common::database::nested_variants::NestedProject>>| {
            ProjectActions::SetParentProject(parent_project)
        },
    );
    html! {
        <BasicForm<NewProject>
            method={FormMethod::POST}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Name" optional={false} errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<String> label="Description" optional={false} errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
            <Checkbox label="Public" errors={builder_store.errors_public.clone()} builder={set_public} value={builder_store.public.unwrap_or(false)} />
            <BasicInput<f64> label="Budget" optional={true} errors={builder_store.errors_budget.clone()} builder={set_budget} value={builder_store.budget.clone().map(Rc::from)} />
            <BasicInput<f64> label="Expenses" optional={true} errors={builder_store.errors_expenses.clone()} builder={set_expenses} value={builder_store.expenses.clone().map(Rc::from)} />
            <BasicInput<chrono::NaiveDateTime> label="Expected end date" optional={true} errors={builder_store.errors_expected_end_date.clone()} builder={set_expected_end_date} value={builder_store.expected_end_date.clone().map(Rc::from)} />
            <BasicInput<chrono::NaiveDateTime> label="End date" optional={true} errors={builder_store.errors_end_date.clone()} builder={set_end_date} value={builder_store.end_date.clone().map(Rc::from)} />
            <Datalist<web_common::database::nested_variants::NestedProjectState, false> builder={set_state} optional={false} errors={builder_store.errors_state.clone()} value={builder_store.state.clone()} label="State" scanner={false} />
            <Datalist<web_common::database::flat_variants::FontAwesomeIcon, false> builder={set_icon} optional={false} errors={builder_store.errors_icon.clone()} value={builder_store.icon.clone()} label="Icon" scanner={false} />
            <Datalist<web_common::database::flat_variants::Color, false> builder={set_color} optional={false} errors={builder_store.errors_color.clone()} value={builder_store.color.clone()} label="Color" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedProject, true> builder={set_parent_project} optional={true} errors={builder_store.errors_parent_project.clone()} value={builder_store.parent_project.clone()} label="Parent project" scanner={false} />
        </BasicForm<NewProject>>
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UpdateProjectFormProp {
    pub id: i32,
}

#[function_component(UpdateProjectForm)]
pub fn update_project_form(props: &UpdateProjectFormProp) -> Html {
    let mut named_requests: Vec<ComponentMessage> = Vec::new();
    let (builder_store, builder_dispatch) = yewdux::use_store::<ProjectBuilder>();
    // We push the ID of the row to the named requests.
    let props = props.clone();
    named_requests.push(ComponentMessage::get::<UpdateProject>(props.id.into()));
    let set_name =
        builder_dispatch.apply_callback(|name: Option<String>| ProjectActions::SetName(name));
    let set_description = builder_dispatch
        .apply_callback(|description: Option<String>| ProjectActions::SetDescription(description));
    let set_public =
        builder_dispatch.apply_callback(|public: bool| ProjectActions::SetPublic(Some(public)));
    let set_budget =
        builder_dispatch.apply_callback(|budget: Option<String>| ProjectActions::SetBudget(budget));
    let set_expenses = builder_dispatch
        .apply_callback(|expenses: Option<String>| ProjectActions::SetExpenses(expenses));
    let set_expected_end_date =
        builder_dispatch.apply_callback(|expected_end_date: Option<String>| {
            ProjectActions::SetExpectedEndDate(expected_end_date)
        });
    let set_end_date = builder_dispatch
        .apply_callback(|end_date: Option<String>| ProjectActions::SetEndDate(end_date));
    let set_state = builder_dispatch.apply_callback(
        |state: Option<Rc<web_common::database::nested_variants::NestedProjectState>>| {
            ProjectActions::SetState(state)
        },
    );
    let set_icon = builder_dispatch.apply_callback(
        |icon: Option<Rc<web_common::database::flat_variants::FontAwesomeIcon>>| {
            ProjectActions::SetIcon(icon)
        },
    );
    let set_color = builder_dispatch.apply_callback(
        |color: Option<Rc<web_common::database::flat_variants::Color>>| {
            ProjectActions::SetColor(color)
        },
    );
    let set_parent_project = builder_dispatch.apply_callback(
        |parent_project: Option<Rc<web_common::database::nested_variants::NestedProject>>| {
            ProjectActions::SetParentProject(parent_project)
        },
    );
    html! {
        <BasicForm<UpdateProject>
            method={FormMethod::PUT}
            named_requests={named_requests}
            builder={builder_store.deref().clone()} builder_dispatch={builder_dispatch}>
            <BasicInput<String> label="Name" optional={false} errors={builder_store.errors_name.clone()} builder={set_name} value={builder_store.name.clone()} />
            <BasicInput<String> label="Description" optional={false} errors={builder_store.errors_description.clone()} builder={set_description} value={builder_store.description.clone()} />
            <Checkbox label="Public" errors={builder_store.errors_public.clone()} builder={set_public} value={builder_store.public.unwrap_or(false)} />
            <BasicInput<f64> label="Budget" optional={true} errors={builder_store.errors_budget.clone()} builder={set_budget} value={builder_store.budget.clone().map(Rc::from)} />
            <BasicInput<f64> label="Expenses" optional={true} errors={builder_store.errors_expenses.clone()} builder={set_expenses} value={builder_store.expenses.clone().map(Rc::from)} />
            <BasicInput<chrono::NaiveDateTime> label="Expected end date" optional={true} errors={builder_store.errors_expected_end_date.clone()} builder={set_expected_end_date} value={builder_store.expected_end_date.clone().map(Rc::from)} />
            <BasicInput<chrono::NaiveDateTime> label="End date" optional={true} errors={builder_store.errors_end_date.clone()} builder={set_end_date} value={builder_store.end_date.clone().map(Rc::from)} />
            <Datalist<web_common::database::nested_variants::NestedProjectState, false> builder={set_state} optional={false} errors={builder_store.errors_state.clone()} value={builder_store.state.clone()} label="State" scanner={false} />
            <Datalist<web_common::database::flat_variants::FontAwesomeIcon, false> builder={set_icon} optional={false} errors={builder_store.errors_icon.clone()} value={builder_store.icon.clone()} label="Icon" scanner={false} />
            <Datalist<web_common::database::flat_variants::Color, false> builder={set_color} optional={false} errors={builder_store.errors_color.clone()} value={builder_store.color.clone()} label="Color" scanner={false} />
            <Datalist<web_common::database::nested_variants::NestedProject, true> builder={set_parent_project} optional={true} errors={builder_store.errors_parent_project.clone()} value={builder_store.parent_project.clone()} label="Parent project" scanner={false} />
        </BasicForm<UpdateProject>>
    }
}
