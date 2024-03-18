//! Module providing a yew component that handles a basic input, which is meant to be used in combination with BasicForm.

use std::collections::HashSet;

use super::InputErrors;
use crate::workers::WebsocketWorker;
use gloo::timers::callback::Timeout;
use validator::Validate;
use wasm_bindgen::JsCast;
use web_common::api::ws::messages::*;
use web_common::custom_validators::validation_errors::ValidationErrorToString;
use yew::prelude::*;
use yew_agent::prelude::WorkerBridgeHandle;
use yew_agent::scope_ext::AgentScopeExt;

#[derive(Clone, PartialEq, Properties)]
pub struct InputProp<Data>
where
    Data: 'static + Clone + PartialEq,
{
    pub label: String,
    #[prop_or_default]
    pub value: Option<Data>,
    pub input_type: String,
    #[prop_or(false)]
    pub optional: bool,
}

impl<Data> InputProp<Data>
where
    Data: 'static + Clone + PartialEq + ToString,
{
    pub fn label(&self) -> String {
        self.label.clone()
    }

    pub fn value(&self) -> Option<Data> {
        self.value.clone()
    }

    pub fn input_type(&self) -> String {
        self.input_type.clone()
    }
}

pub struct BasicInput<Data> {
    _websocket: WorkerBridgeHandle<WebsocketWorker<FrontendMessage, BackendMessage>>,
    errors: HashSet<String>,
    current_value: Option<String>,
    is_valid: Option<bool>,
    validation_timeout: Option<Timeout>,
    _data: std::marker::PhantomData<Data>,
}

pub enum InputMessage<Data> {
    Backend(BackendMessage),
    RemoveError(String),
    RemoveErrors,
    Validate(Result<Data, Vec<String>>),
    StartValidationTimeout(Result<Data, Vec<String>>),
    UpdateCurrentValue(String),
}

impl<Data> Component for BasicInput<Data>
where
    Data: 'static
        + Clone
        + PartialEq
        + Default
        + Validate
        + TryFrom<String, Error = Vec<String>>
        + ToString,
{
    type Message = InputMessage<Data>;
    type Properties = InputProp<Data>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            _websocket: ctx.link().bridge_worker(Callback::from({
                let link = ctx.link().clone();
                move |message: BackendMessage| {
                    link.send_message(InputMessage::Backend(message));
                }
            })),
            errors: HashSet::new(),
            is_valid: None,
            current_value: None,
            validation_timeout: None,
            _data: std::marker::PhantomData,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            InputMessage::Backend(_bm) => false,
            InputMessage::RemoveErrors => {
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
            InputMessage::RemoveError(error) => {
                self.errors.remove(&error);
                true
            }
            InputMessage::Validate(data) => {
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

                if let Err(errors) = data.validate() {
                    for error in errors.convert_to_string() {
                        self.errors.insert(error);
                    }
                    self.is_valid = Some(false);
                    change = true;
                }

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
            InputMessage::StartValidationTimeout(data) => {
                let link = ctx.link().clone();
                if let Some(timeout) = self.validation_timeout.take() {
                    timeout.cancel();
                }
                self.validation_timeout = Some(Timeout::new(300, move || {
                    link.send_message(InputMessage::Validate(data));
                }));
                false
            }
            InputMessage::UpdateCurrentValue(value) => {
                self.current_value = Some(value);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        let classes = match self.is_valid {
            Some(true) => {
                if let (Some(previous), Some(current)) = (
                    ctx.props().value(),
                    self.current_value.as_ref().and_then(|value| Data::try_from(value.clone()).ok()),
                ) {
                    if previous != current {
                        "input-group input-group-valid"
                    } else {
                        "input-group"
                    }
                } else {
                    "input-group input-group-valid"
                }
            },
            Some(false) => "input-group input-group-invalid",
            None => "input-group",
        };

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

                link.send_message(InputMessage::UpdateCurrentValue(value.clone()));

                if props.optional && value.is_empty() {
                    link.send_message(InputMessage::RemoveErrors);
                    return;
                }

                let data = Data::try_from(value);

                link.send_message(InputMessage::StartValidationTimeout(data));
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
                    link.send_message(InputMessage::RemoveErrors);
                    return;
                }

                let data = Data::try_from(value);

                link.send_message(InputMessage::StartValidationTimeout(data));
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

        let on_delete ={
            let link = ctx.link().clone();
            Callback::from(move |error: String| {
                link.send_message(InputMessage::RemoveError(error));
            })
        };
        
        html! {
            <div class={classes}>
                <label for={props.label()}>{format!("{}:", props.label())}</label>
                <input
                    type={props.input_type()}
                    class="input-control"
                    name={props.label().to_lowercase().replace(" ", "_")}
                    value={input_value}
                    oninput={on_input}
                    onblur={on_blur}
                />
                <InputErrors errors={self.errors.clone()} on_delete={on_delete} />
            </div>
        }
    }
}
