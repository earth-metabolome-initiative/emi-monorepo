//! Component for the form requiring user name and surname.

use crate::components::forms::*;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, rc::Rc};
use validator::Validate;
use web_common::api::form_traits::FormMethod;
use web_common::custom_validators::NotEmpty;
use web_common::database::inserts::new_sample::NewSampleName;
use web_common::database::inserts::NewSample;
use web_common::database::SampleState;
use web_common::database::*;
use yew::prelude::*;
use yewdux::{use_store, Reducer, Store};

#[derive(Debug, PartialEq, Clone, Default, Store, Serialize, Deserialize)]
#[store(storage = "session")]
pub struct NewSampleBuilder {
    pub name: NewSampleName,
    pub description: NotEmpty,
    pub public: bool,
    pub parent_project: Option<NestedProject>,
    pub sample_state: Option<SampleState>,
    pub collector: Option<NestedPublicUser>,
    pub sampling_procedure: Option<SamplingProcedure>,
    pub taxa: Vec<Taxa>,
}

impl FormBuilder for NewSampleBuilder {
    type Data = NewSample;
    type Actions = NewSampleBuilderActions;

    fn buildable(&self) -> Result<(), web_common::api::ApiError> {
        self.name.validate()?;
        self.description.validate()?;
        if self.sample_state.is_none() {
            return Err(web_common::api::ApiError::BadRequest(vec![
                "Sample state is required.".to_string(),
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
            collector: self.collector.clone().unwrap(),
            sampling_procedure: self.sampling_procedure.clone().unwrap(),
            taxa: self.taxa.clone(),
        }
    }
}

pub enum NewSampleBuilderActions {
    SetName(NewSampleName),
    SetDescription(NotEmpty),
    SetPublic(bool),
    SetParentProject(Option<NestedProject>),
    SetSampleState(Option<SampleState>),
    SetCollector(Option<NestedPublicUser>),
    SetSamplingProcedure(Option<SamplingProcedure>),
    SetTaxa(Vec<Taxa>),
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
            NewSampleBuilderActions::SetParentProject(parent_project) => {
                state_mut.parent_project = parent_project;
            }
            NewSampleBuilderActions::SetSampleState(sample_state) => {
                state_mut.sample_state = sample_state;
            }
            NewSampleBuilderActions::SetCollector(collector) => {
                state_mut.collector = collector;
            }
            NewSampleBuilderActions::SetSamplingProcedure(sampling_procedure) => {
                state_mut.sampling_procedure = sampling_procedure;
            }
            NewSampleBuilderActions::SetTaxa(taxa) => {
                state_mut.taxa = taxa;
            }
        }
        state
    }
}

impl FormBuildable for NewSample {
    type Builder = NewSampleBuilder;

    const METHOD: FormMethod = FormMethod::POST;

    fn title() -> &'static str {
        "New Sample"
    }

    fn task_target() -> &'static str {
        "Sample"
    }

    fn description() -> &'static str {
        concat!("Create a new sample.\n",)
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
    let set_description =
        dispatch.apply_callback(|description| NewSampleBuilderActions::SetDescription(description));
    let set_public = dispatch.apply_callback(|public| NewSampleBuilderActions::SetPublic(public));
    let set_parent_project = dispatch.apply_callback(|mut projects: Vec<NestedProject>| {
        NewSampleBuilderActions::SetParentProject(projects.pop())
    });
    let set_sample_state = dispatch.apply_callback(|mut sample_states: Vec<SampleState>| {
        NewSampleBuilderActions::SetSampleState(sample_states.pop())
    });
    let set_collector = dispatch.apply_callback(|mut users: Vec<NestedPublicUser>| {
        NewSampleBuilderActions::SetCollector(users.pop())
    });
    let set_sampling_procedure =
        dispatch.apply_callback(|mut sampling_procedures: Vec<SamplingProcedure>| {
            NewSampleBuilderActions::SetSamplingProcedure(sampling_procedures.pop())
        });
    let set_taxa =
        dispatch.apply_callback(|taxa: Vec<Taxa>| NewSampleBuilderActions::SetTaxa(taxa));

    html! {
        <BasicForm<NewSample> builder={store.deref().clone()}>
            <BasicInput<NewSampleName> label="Name" builder={set_name} value={store.name.clone()} input_type={InputType::Text} />
            <BasicInput<NotEmpty> label="Description" builder={set_description} value={store.description.clone()} input_type={InputType::Textarea} />
            <Checkbox label="Public" builder={set_public} value={store.public} />
            <Datalist<web_common::database::NestedProject> builder={set_parent_project} optional = {true} value={store.parent_project.clone().map_or_else(|| Vec::new(), |value| vec![value])} label="Project" />
            <Datalist<web_common::database::SampleState> builder={set_sample_state} value={store.sample_state.clone().map_or_else(|| Vec::new(), |value| vec![value])} label="Sample State" />
            <Datalist<web_common::database::NestedPublicUser> builder={set_collector} value={store.collector.clone().map_or_else(|| Vec::new(), |value| vec![value])} label="Collector" />
            <Datalist<web_common::database::SamplingProcedure> builder={set_sampling_procedure} value={store.sampling_procedure.clone().map_or_else(|| Vec::new(), |value| vec![value])} label="Sampling Procedure" />
            <Datalist<web_common::database::Taxa> builder={set_taxa} optional = {true} number_of_choices = {5} value={store.taxa.clone()} label="Taxa" />
        </BasicForm<NewSample>>
    }
}
