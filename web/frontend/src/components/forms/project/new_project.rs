//! Component for the form requiring user name and surname.

use crate::components::forms::*;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, rc::Rc};
use validator::Validate;
use web_common::api::form_traits::FormMethod;
use web_common::custom_validators::NotEmpty;
use web_common::database::inserts::new_project::NewProjectName;
use web_common::database::inserts::NewProject;
use web_common::database::*;
use web_common::database::{NestedProject, ProjectState};
use yew::prelude::*;
use yewdux::{use_store, Reducer, Store};

#[derive(Debug, PartialEq, Clone, Default, Store, Serialize, Deserialize)]
#[store(storage = "session")]
pub struct NewProjectBuilder {
    pub name: NewProjectName,
    pub description: NotEmpty,
    pub public: bool,
    pub parent_project: Option<NestedProject>,
    pub project_state: Option<ProjectState>,
}

impl FormBuilder for NewProjectBuilder {
    type Data = NewProject;
    type Actions = NewProjectBuilderActions;

    fn buildable(&self) -> Result<(), web_common::api::ApiError> {
        self.name.validate()?;
        self.description.validate()?;
        if self.project_state.is_none() {
            return Err(web_common::api::ApiError::BadRequest(vec![
                "Project state is required.".to_string(),
            ]));
        }
        Ok(())
    }

    fn form_level_errors(&self) -> Vec<String> {
        vec![]
    }

    fn build(&self) -> Self::Data {
        NewProject {
            name: self.name.clone(),
            description: self.description.clone(),
            public: self.public,
            project_state: self.project_state.clone().unwrap(),
        }
    }
}

pub enum NewProjectBuilderActions {
    SetName(NewProjectName),
    SetDescription(NotEmpty),
    SetPublic(bool),
    SetParentProject(Option<NestedProject>),
    SetProjectState(Option<ProjectState>),
}

impl Reducer<NewProjectBuilder> for NewProjectBuilderActions {
    fn apply(self, mut state: std::rc::Rc<NewProjectBuilder>) -> std::rc::Rc<NewProjectBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NewProjectBuilderActions::SetName(name) => {
                state_mut.name = name;
            }
            NewProjectBuilderActions::SetDescription(description) => {
                state_mut.description = description;
            }
            NewProjectBuilderActions::SetPublic(public) => {
                state_mut.public = public;
            }
            NewProjectBuilderActions::SetParentProject(parent_project) => {
                state_mut.parent_project = parent_project;
            }
            NewProjectBuilderActions::SetProjectState(project_state) => {
                state_mut.project_state = project_state;
            }
        }
        state
    }
}

impl FormBuildable for NewProject {
    type Builder = NewProjectBuilder;

    const METHOD: FormMethod = FormMethod::POST;

    fn title() -> &'static str {
        "New Project"
    }

    fn task_target() -> &'static str {
        "Project"
    }

    fn description() -> &'static str {
        concat!("Create a new project.\n",)
    }

    fn requires_authentication() -> bool {
        true
    }
}


#[function_component(NewProjectForm)]
pub fn complete_profile_form() -> Html {
    // The use_reducer hook takes an initialization function which will be called only once.
    let (store, dispatch) = use_store::<NewProjectBuilder>();

    let set_name = dispatch.apply_callback(|name| NewProjectBuilderActions::SetName(name));
    let set_description = dispatch
        .apply_callback(|description| NewProjectBuilderActions::SetDescription(description));
    let set_public = dispatch.apply_callback(|public| NewProjectBuilderActions::SetPublic(public));
    let set_parent_project = dispatch.apply_callback(|mut projects: Vec<NestedProject>| {
        NewProjectBuilderActions::SetParentProject(projects.pop())
    });
    let set_project_state = dispatch.apply_callback(|mut project_states: Vec<ProjectState>| {
        NewProjectBuilderActions::SetProjectState(project_states.pop())
    });

    html! {
        <BasicForm<NewProject> builder={store.deref().clone()}>
            <BasicInput<NewProjectName> label="Name" builder={set_name} value={store.name.clone()} input_type={InputType::Text} />
            <BasicInput<NotEmpty> label="Description" builder={set_description} value={store.description.clone()} input_type={InputType::Textarea} />
            <Checkbox label="Public" builder={set_public} value={store.public} />
            // <Datalist<Editable<web_common::database::NestedProject>> builder={set_parent_project} value={store.parent_project.clone().map_or_else(|| Vec::new(), |value| vec![value])} label="Project" />
            <Datalist<web_common::database::ProjectState> builder={set_project_state} value={store.project_state.clone().map_or_else(|| Vec::new(), |value| vec![value])} label="Project State" />
        </BasicForm<NewProject>>
    }
}
