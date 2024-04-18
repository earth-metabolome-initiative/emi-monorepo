//! Module providing a yew component that handles a basic input, which is meant to be used in combination with BasicForm.

use std::collections::HashSet;

use super::InputErrors;
use crate::components::database::row_to_badge::RowToBadge;
use crate::workers::WebsocketWorker;
use gloo::timers::callback::Timeout;
use sublime_fuzzy::{best_match, format_simple};
use validator::Validate;
use wasm_bindgen::JsCast;
use web_common::api::ws::messages::*;
use web_common::custom_validators::validation_errors::ValidationErrorToString;
use web_common::database::SearchTable;
use web_common::database::SearcheableTable;
use web_common::database::SearcheableTableRow;
use web_common::database::TableRow;

use yew::prelude::*;
use yew_agent::prelude::WorkerBridgeHandle;
use yew_agent::scope_ext::AgentScopeExt;

#[derive(Clone, PartialEq, Properties)]
pub struct DatalistProp<Data>
where
    Data: 'static + Clone + PartialEq,
{
    pub label: String,
    #[prop_or(true)]
    pub show_label: bool,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub value: Option<Data>,
    #[prop_or_default]
    pub step: Option<f64>,
    #[prop_or(false)]
    pub optional: bool,
}

impl<Data> DatalistProp<Data>
where
    Data: 'static + Clone + PartialEq + ToString,
{
    pub fn label(&self) -> String {
        self.label.clone()
    }

    pub fn normalized_label(&self) -> String {
        self.label.replace(" ", "_").to_lowercase()
    }

    pub fn value(&self) -> Option<Data> {
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
    candidates: Vec<Data>,
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
}

impl<Data> Component for Datalist<Data>
where
    Data: 'static
        + Clone
        + PartialEq
        + SearchTable
        + Into<SearcheableTableRow>
        + TryFrom<SearcheableTableRow, Error = &'static str>
        + ToString
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
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DatalistMessage::Backend(message) => match message {
                BackendMessage::SearchTable(_task_id, results) => match results {
                    Ok(results) => {
                        self.candidates = results
                            .into_iter()
                            .map(|row| Data::try_from(row).expect("Failed to convert row to data"))
                            .collect();

                        // We sort the candidates by their similarity to the current value
                        if let Some(current_value) = self.current_value.as_ref() {
                            self.candidates.sort_by(|a, b| {
                                let a = a.to_string();
                                let b = b.to_string();
                                let a = best_match(current_value, &a);
                                let b = best_match(current_value, &b);
                                b.partial_cmp(&a).unwrap()
                            });
                        }

                        true
                    }
                    Err(error) => {
                        log::error!("Error searching table: {:?}", error);
                        false
                    }
                },
                _ => false,
            },
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

                // If the current value of the input field is equal to
                // the default value, we do not want to display the field
                // as being valid, but back to its initial state.
                if let Some(value) = ctx.props().value() {
                    if value == data {
                        self.is_valid = None;
                        change = true;
                    }
                }

                change
            }
            DatalistMessage::StartValidationTimeout(data) => {
                let link = ctx.link().clone();
                if let Some(timeout) = self.validation_timeout.take() {
                    timeout.cancel();
                }
                self.validation_timeout = Some(Timeout::new(300, move || {
                    link.send_message(DatalistMessage::Validate(data));
                }));
                false
            }
            DatalistMessage::SearchCandidates => {
                if let Some(value) = self.current_value.as_ref() {
                    self.websocket
                        .send(Data::search(value.clone().into()).into());
                }
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
            DatalistMessage::UpdateCurrentValue(value) => {
                self.current_value = Some(value);
                if self.candidates.is_empty() {
                    ctx.link().send_message(DatalistMessage::SearchCandidates);
                } else {
                    ctx.link()
                        .send_message(DatalistMessage::SearchCandidatesTimeout);
                }
                false
            }
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

                // link.send_message(DatalistMessage::StartValidationTimeout(data));
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

                // link.send_message(DatalistMessage::StartValidationTimeout(data));
            })
        };

        let input_value = self.current_value.as_ref().map_or_else(
            || {
                props
                    .value()
                    .map_or_else(|| "".to_string(), |value| value.to_string())
            },
            |value| value.to_string(),
        );

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
                <input
                    type="text"
                    class="input-control"
                    value={input_value}
                    placeholder={props.placeholder.clone().unwrap_or_else(|| props.label())}
                    oninput={on_input}
                    onblur={on_blur}
                    id={props.normalized_label()}
                    name={props.normalized_label()}
                />
                {if self.candidates.is_empty() {
                    html! {}
                } else {
                    html!{<ul>
                        {for self.candidates.iter().enumerate().map(|(i, candidate)| {
                            html! {
                                <li>{candidate.to_badge(self.current_value.as_ref().map(|val| val.as_str()))}</li>
                            }
                        })}
                    </ul>}
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
