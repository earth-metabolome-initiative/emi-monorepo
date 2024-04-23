//! Component for the form requiring user name and surname.

use crate::components::forms::*;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, rc::Rc};
use validator::Validate;
use web_common::api::form_traits::FormMethod;
use web_common::custom_validators::NotEmpty;
use web_common::database::inserts::new_sample::NewSampleName;
use web_common::database::inserts::NewSample;
use web_common::database::*;
use web_common::database::{NestedSample, SampleState};
use yew::prelude::*;
use yewdux::{use_store, Reducer, Store};

#[derive(Debug, PartialEq, Clone, Default, Store, Serialize, Deserialize)]
#[store(storage = "session")]
pub struct NewSampleBuilder {
    pub name: NewSampleName,
    pub description: NotEmpty,
    pub public: bool,
    pub parent_sample: Option<NestedSample>,
    pub sample_state: Option<SampleState>,
}

impl FormBuilder for NewSampleBuilder {
    type Data = NewSample;
    type Actions = NewSampleBuilderActions;

    fn buildable(&self) -> Result<(), web_common::api::ApiError> {
        self.name.validate()?;
        self.description.validate()?;
        if self.sample_state.is_none() {
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
        NewSample {
            name: self.name.clone(),
            description: self.description.clone(),
            public: self.public,
            sample_state: self.sample_state.clone().unwrap(),
        }
    }
}

pub enum NewSampleBuilderActions {
    SetName(NewSampleName),
    SetDescription(NotEmpty),
    SetPublic(bool),
    SetParentProject(Option<NestedSample>),
    SetSampleState(Option<SampleState>),
}

impl Reducer<NewSampleBuilder> for NewSampleBuilderActions {
    fn apply(self, mut state: std::rc::Rc<NewSampleBuilder>) -> std::rc::Rc<NewSampleBuilder> {
        let state_mut = Rc::make_mut(&mut state);
        match self {
            NewSampleBuilderActions::SetName(name) => {
                state_mut.name = name;
            }
            NewSampleBuilderActions::SetDescription(description) => {
                state_mut.description = description;
            }
            NewSampleBuilderActions::SetPublic(public) => {
                state_mut.public = public;
            }
            NewSampleBuilderActions::SetParentProject(parent_sample) => {
                state_mut.parent_sample = parent_sample;
            }
            NewSampleBuilderActions::SetSampleState(sample_state) => {
                state_mut.sample_state = sample_state;
            }
        }
        state
    }
}

impl FormBuildable for NewSample {
    type Builder = NewSampleBuilder;

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


#[function_component(NewSampleForm)]
pub fn complete_profile_form() -> Html {
    // The use_reducer hook takes an initialization function which will be called only once.
    let (store, dispatch) = use_store::<NewSampleBuilder>();

    let set_name = dispatch.apply_callback(|name| NewSampleBuilderActions::SetName(name));
    let set_description = dispatch
        .apply_callback(|description| NewSampleBuilderActions::SetDescription(description));
    let set_public = dispatch.apply_callback(|public| NewSampleBuilderActions::SetPublic(public));
    let set_parent_sample = dispatch.apply_callback(|mut samples: Vec<NestedSample>| {
        NewSampleBuilderActions::SetParentProject(samples.pop())
    });
    let set_sample_state = dispatch.apply_callback(|mut sample_states: Vec<SampleState>| {
        NewSampleBuilderActions::SetSampleState(sample_states.pop())
    });

    html! {
        <BasicForm<NewSample> builder={store.deref().clone()}>
            <BasicInput<NewSampleName> label="Name" builder={set_name} value={store.name.clone()} input_type={InputType::Text} />
            <BasicInput<NotEmpty> label="Description" builder={set_description} value={store.description.clone()} input_type={InputType::Textarea} />
            <Checkbox label="Public" builder={set_public} value={store.public} />
            // <Datalist<Editable<web_common::database::NestedSample>> builder={set_parent_sample} value={store.parent_sample.clone().map_or_else(|| Vec::new(), |value| vec![value])} label="Project" />
            <Datalist<web_common::database::SampleState> builder={set_sample_state} value={store.sample_state.clone().map_or_else(|| Vec::new(), |value| vec![value])} label="Project State" />
        </BasicForm<NewSample>>
    }
}
