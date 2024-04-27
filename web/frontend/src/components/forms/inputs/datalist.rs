//! Module providing a yew component that handles a basic input, which is meant to be used in combination with BasicForm.

use std::collections::HashSet;

use super::InputErrors;
use crate::components::database::row_to_badge::RowToBadge;
use crate::workers::WebsocketWorker;
use gloo::timers::callback::Timeout;
use serde::de::DeserializeOwned;
use wasm_bindgen::JsCast;
use web_common::api::ws::messages::*;

use web_common::database::Searchable;
use yew::prelude::*;
use yew_agent::prelude::WorkerBridgeHandle;
use yew_agent::scope_ext::AgentScopeExt;

#[derive(Clone, PartialEq, Properties)]
pub struct DatalistProp<Data>
where
    Data: 'static + Clone + PartialEq,
{
    pub label: String,
    pub builder: Callback<Vec<Data>>,
    #[prop_or(true)]
    pub show_label: bool,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub value: Vec<Data>,
    #[prop_or(false)]
    pub optional: bool,
    #[prop_or(1)]
    pub number_of_choices: usize,
    #[prop_or(10)]
    pub number_of_candidates: u32,
}

impl<Data> DatalistProp<Data>
where
    Data: 'static + Clone + PartialEq,
{
    pub fn label(&self) -> String {
        self.label.clone()
    }

    pub fn normalized_label(&self) -> String {
        self.label.replace(" ", "_").to_lowercase()
    }

    pub fn value(&self) -> Vec<Data> {
        self.value.clone()
    }
}

pub struct Datalist<Data> {
    websocket: WorkerBridgeHandle<WebsocketWorker<FrontendMessage, BackendMessage>>,
    errors: HashSet<String>,
    current_value: Option<String>,
    is_valid: Option<bool>,
    validation_timeout: Option<Timeout>,
    search_timeout: Option<Timeout>,
    /// The candidates that are displayed in the datalist.
    candidates: Vec<Data>,
    /// The selected candidates.
    selections: Vec<Data>,
    /// The list of selections being deleted.
    selections_to_delete: Vec<usize>,
    /// Whether it is currently on focus.
    is_focused: bool,
    /// The number of search queries that are currently being processed.
    number_of_search_queries: usize,
}

pub enum DatalistMessage<Data> {
    Backend(BackendMessage),
    RemoveError(String),
    RemoveErrors,
    Validate(Result<Data, Vec<String>>),
    StartValidationTimeout(Result<Data, Vec<String>>),
    UpdateCurrentValue(String),
    SearchCandidatesTimeout,
    SearchCandidates,
    UpdateCandidates(Vec<Data>),
    SelectCandidate(usize),
    DeleteSelection(usize),
    StartDeleteSelectionTimeout(usize),
    Focus,
    Blur,
}

impl<Data> Component for Datalist<Data>
where
    Data: 'static
        + Clone
        + PartialEq
        + DeserializeOwned
        + Searchable
        + RowToBadge,
{
    type Message = DatalistMessage<Data>;
    type Properties = DatalistProp<Data>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            websocket: ctx.link().bridge_worker(Callback::from({
                let link = ctx.link().clone();
                move |message: BackendMessage| {
                    link.send_message(DatalistMessage::Backend(message));
                }
            })),
            errors: HashSet::new(),
            is_valid: None,
            current_value: None,
            validation_timeout: None,
            search_timeout: None,
            candidates: Vec::new(),
            selections: ctx.props().value(),
            selections_to_delete: Vec::new(),
            is_focused: false,
            number_of_search_queries: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DatalistMessage::Backend(message) => match message {
                BackendMessage::SearchTable(_task_id, results) => {
                    self.number_of_search_queries -= 1;
                    match results {
                        Ok(results) => {
                            ctx.link().send_message(DatalistMessage::UpdateCandidates(results
                                .iter()
                                .map(|row| {
                                    bincode::deserialize(row).expect("Failed to convert row to data")
                                })
                                .collect()));

                            true
                        }
                        Err(error) => {
                            log::error!("Error searching table: {:?}", error);
                            false
                        }
                    }
                }
                _ => false,
            },
            DatalistMessage::UpdateCandidates(candidates) => {
                if candidates.is_empty() {
                    self.errors.insert("No candidates found".to_string());
                } else {
                    self.errors.remove("No candidates found");
                }
                if self.candidates == candidates {
                    return false;
                }
                self.candidates = candidates;
                true
            }
            DatalistMessage::RemoveErrors => {
                let mut changes = false;

                if !self.errors.is_empty() {
                    self.errors.clear();
                    changes = true;
                }

                if self.is_valid.is_some() {
                    self.is_valid = None;
                    changes = true;
                }

                changes
            }
            DatalistMessage::RemoveError(error) => {
                self.errors.remove(&error);
                true
            }
            DatalistMessage::Validate(data) => {
                if let Some(timeout) = self.validation_timeout.take() {
                    timeout.cancel();
                }

                let mut change = false;

                if !self.errors.is_empty() {
                    self.errors.clear();
                    change = true;
                }

                if let Err(errors) = data {
                    for error in errors {
                        self.errors.insert(error);
                    }
                    self.is_valid = Some(false);
                    return true;
                }

                let data = data.unwrap();

                // TODO! CONVERT INTO A CHECK ON THE DB!
                // if let Err(errors) = data.validate() {
                //     for error in errors.convert_to_string() {
                //         self.errors.insert(error);
                //     }
                //     self.is_valid = Some(false);
                //     change = true;
                // }

                if self.is_valid != Some(true) {
                    self.is_valid = Some(true);
                    change = true;
                }

                change
            }
            DatalistMessage::StartValidationTimeout(data) => {
                let link = ctx.link().clone();
                if let Some(timeout) = self.validation_timeout.take() {
                    timeout.cancel();
                }
                self.validation_timeout = Some(Timeout::new(200, move || {
                    link.send_message(DatalistMessage::Validate(data));
                }));
                false
            }
            DatalistMessage::SearchCandidates => {
                // If ths system has already returned a list of candidates that is
                // less than the maximum number of candidates, we do not want to
                // pester the server with another request, as the list will not change.
                // if !self.candidates.is_empty()
                //     && self.candidates.len() <= ctx.props().number_of_candidates as usize
                // {
                //     return false;
                // }
                let query = self.current_value.clone().unwrap_or_else(|| "".to_string());
                self.websocket
                    .send(Data::search_task(query, ctx.props().number_of_candidates).into());
                self.number_of_search_queries += 1;
                false
            }
            DatalistMessage::SearchCandidatesTimeout => {
                let link = ctx.link().clone();
                if let Some(timeout) = self.search_timeout.take() {
                    timeout.cancel();
                }
                self.search_timeout = Some(Timeout::new(100, move || {
                    link.send_message(DatalistMessage::SearchCandidates);
                }));
                false
            }
            DatalistMessage::SelectCandidate(index) => {
                let candidate = self.candidates.get(index).unwrap();
                if ctx.props().number_of_choices == 1 {
                    self.selections.clear();
                }
                self.selections.push(candidate.clone());
                if self.selections.len() == ctx.props().number_of_choices {
                    ctx.props().builder.emit(self.selections.clone());
                    self.is_focused = false;
                }
                self.current_value = None;
                true
            }
            DatalistMessage::DeleteSelection(index) => {
                self.selections_to_delete.retain(|&i| i != index);
                self.selections.remove(index);

                // If the user has deleted all selections, we want to
                // reset the current value to None.
                if self.selections.is_empty() {
                    self.current_value = None;
                }

                ctx.props().builder.emit(self.selections.clone());

                true
            }
            DatalistMessage::UpdateCurrentValue(value) => {
                // We check if any of the candidates match the current value
                // exactly. If so, we select that candidate.
                if let Some(candidate_index) = self
                    .candidates
                    .iter()
                    .position(|candidate| candidate.matches(&value))
                {
                    ctx.link().send_message(DatalistMessage::SelectCandidate(candidate_index));
                } else {
                    self.current_value = Some(value);
                }
                ctx.link()
                    .send_message(DatalistMessage::SearchCandidatesTimeout);

                true
            }
            DatalistMessage::StartDeleteSelectionTimeout(index) => {
                self.selections_to_delete.push(index);
                let link = ctx.link().clone();
                Timeout::new(200, move || {
                    link.send_message(DatalistMessage::DeleteSelection(index));
                })
                .forget();
                true
            }
            DatalistMessage::Blur => {
                self.is_focused = false;
                true
            }
            DatalistMessage::Focus => {
                self.is_focused = true;
                true
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        // If this is the first time the component is rendered, we want to
        // send a message to the server to get the list of candidates.
        if first_render {
            ctx.link().send_message(DatalistMessage::SearchCandidates);
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let classes = "input-group datalist";

        // We create a timeout so that when the user stops typing for at least
        // 500ms, the input field is validated.

        let on_input: Callback<InputEvent> = {
            let link = ctx.link().clone();
            let props = ctx.props().clone();
            Callback::from(move |input_event: InputEvent| {
                input_event.prevent_default();

                // We extract the current value of the input field
                let value = input_event
                    .target()
                    .unwrap()
                    .dyn_into::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value();

                link.send_message(DatalistMessage::UpdateCurrentValue(value.clone()));

                if props.optional && value.is_empty() {
                    link.send_message(DatalistMessage::RemoveErrors);
                    return;
                }

            })
        };

        let on_focus = {
            let link = ctx.link().clone();
            Callback::from(move |_: FocusEvent| {
                link.send_message(DatalistMessage::Focus);
            })
        };

        let on_blur = {
            let link = ctx.link().clone();
            let props = ctx.props().clone();
            Callback::from(move |input_event: FocusEvent| {
                input_event.prevent_default();

                // We extract the current value of the input field
                let value = input_event
                    .target()
                    .unwrap()
                    .dyn_into::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value();

                if props.optional && value.is_empty() {
                    link.send_message(DatalistMessage::RemoveErrors);
                    return;
                }
                // link.send_message(DatalistMessage::Blur);
                // link.send_message(DatalistMessage::StartValidationTimeout(data));
            })
        };

        let input_value = self.current_value.clone().unwrap_or_else(|| "".to_string());

        let on_delete = {
            let link = ctx.link().clone();
            Callback::from(move |error: String| {
                link.send_message(DatalistMessage::RemoveError(error));
            })
        };

        let current_value = self
            .current_value
            .as_ref()
            .map_or_else(|| "".to_string(), |value| value.clone());

        let input_field = html! {
            <>
                {if props.show_label {
                    html! {
                        <label for={props.normalized_label()} class={"input-label"}>
                            {props.label()}
                        </label>
                    }
                } else {
                    html! {}
                }}
                {if self.is_focused || self.selections.is_empty(){
                html!{
                    <input
                        type="search"
                        class="input-control"
                        value={input_value}
                        placeholder={props.placeholder.clone().unwrap_or_else(|| props.label())}
                        oninput={on_input}
                        onfocus={on_focus}
                        onblur={on_blur}
                        id={props.normalized_label()}
                        name={props.normalized_label()}
                    />
                }
                } else {
                    html!{}
                }}
                {if !self.selections.is_empty() {
                    html! {
                        <ul class="selected-datalist-badges">
                        {for self.selections.iter().enumerate().map(|(i, selection)| {
                            let on_click = {
                                let link = ctx.link().clone();
                                Callback::from(move |e: MouseEvent| {
                                    e.prevent_default();
                                    e.stop_propagation();
                                    link.send_message(DatalistMessage::StartDeleteSelectionTimeout(i));
                                })
                            };
                            let classes = format!("selected-datalist-badge {}{}", selection.primary_color_class(), if self.selections_to_delete.contains(&i) {" deleting"} else {""});
                            html! {
                                <li class={classes} title={format!("{}", selection.description())}>
                                    {selection.to_selected_datalist_badge()}
                                    <button onclick={on_click} class="delete-button">
                                        <i class="fas fa-times"></i>
                                    </button>
                                </li>
                            }
                        })}
                        {
                            // If this entry needs more than one selection, we display
                            // the button to add more selections.
                            if !self.is_focused && self.selections.len() < ctx.props().number_of_choices {
                                let classes = format!("datalist-add{}", if self.selections_to_delete.len() == self.selections.len() {" deleting"} else {""});
                                let on_click = {
                                    let link = ctx.link().clone();
                                    Callback::from(move |e: MouseEvent| {
                                        e.prevent_default();
                                        e.stop_propagation();
                                        link.send_message(DatalistMessage::Focus);
                                    })
                                };
                                html! {
                                    <li onclick={on_click} class={classes}>
                                        <button>
                                            <i class="fas fa-plus"></i>
                                        </button>
                                    </li>
                                }
                            } else {
                                html! {}
                            }
                        }
                        </ul>
                    }
                } else {
                    html!{}
                }}
                {if self.number_of_search_queries > 0{
                    // We display a loading spinner if the system is currently searching for candidates.
                    html! {
                        <div class="loading-spinner"></div>
                    }
                } else {
                    {if !self.is_focused || self.candidates.is_empty() || self.selections.len() == ctx.props().number_of_choices{
                        html! {}
                    } else {
                        let mut total_candidate_score = 0.0;
                        let candidate_score: Vec<isize> = self.candidates.iter().map(|candidate| {
                            let candidate_score = candidate.similarity_score(&current_value);
                            total_candidate_score += candidate_score as f64;
                            candidate_score
                        }).collect();
                        let mean_candidate_score = total_candidate_score / self.candidates.len() as f64;
                        let mut indices_to_sort: Vec<usize> = (0..self.candidates.len()).collect::<Vec<usize>>();
                        indices_to_sort.sort_by_key(|&i| candidate_score[i]);
                        let filtered_indices = indices_to_sort.into_iter().filter(|&i| {
                            candidate_score[i] as f64 >= mean_candidate_score
                        }).filter(|&i|{
                            // If the current candidate has already been selected,
                            // we do not want to display it.
                            !self.selections.iter().any(|selection| selection == &self.candidates[i])
                        }).collect::<Vec<usize>>();
                        if filtered_indices.is_empty() {
                            html!{}
                        } else {
                            html!{<ul class="datalist-candidates">
                            {for filtered_indices.iter().rev().map(|&i| {
                               let candidate = &self.candidates[i];
                                let on_click = {
                                    let link = ctx.link().clone();
                                    Callback::from(move |e: MouseEvent| {
                                        e.prevent_default();
                                        e.stop_propagation();
                                        link.send_message(DatalistMessage::SelectCandidate(i));
                                    })
                                };
                                html! {
                                    <li onclick={on_click} class={format!("datalist-candidate {}", candidate.primary_color_class())}>{candidate.to_datalist_badge(&current_value)}</li>
                                }
                            })}
                        </ul>
                        }
                        }
                    }}
                }}

            </>
        };

        html! {
            <div class={classes}>
                {if props.show_label {
                    html! {
                        <div class="input-container">
                            {input_field}
                        </div>
                    }
                } else {
                    input_field
                }}
                <InputErrors errors={self.errors.clone()} on_delete={on_delete} />
            </div>
        }
    }
}
