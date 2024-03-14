//! Module providing a yew component that handles a basic input, which is meant to be used in combination with BasicForm.

use std::collections::HashSet;

use super::InputError;
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
    pub value: Data,
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

    pub fn value(&self) -> String {
        self.value.clone().to_string()
    }

    pub fn input_type(&self) -> String {
        self.input_type.clone()
    }
}

pub struct BasicInput<Data> {
    _websocket: WorkerBridgeHandle<WebsocketWorker<FrontendMessage, BackendMessage>>,
    errors: HashSet<String>,
    current_value: Option<String>,
    validation_timeout: Option<Timeout>,
    _data: std::marker::PhantomData<Data>,
}

pub enum InputMessage<Data> {
    Backend(BackendMessage),
    RemoveError(String),
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
            current_value: None,
            validation_timeout: None,
            _data: std::marker::PhantomData,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            InputMessage::Backend(_bm) => false,
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
                    return true;
                }

                let data = data.unwrap();

                if let Err(errors) = data.validate() {
                    for error in errors.convert_to_string() {
                        self.errors.insert(error);
                    }
                    change = true;
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

        let classes = if self.errors.is_empty() {
            "input-group"
        } else {
            "input-group error"
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
                    return;
                }

                let data = Data::try_from(value);

                link.send_message(InputMessage::StartValidationTimeout(data));
            })
        };

        html! {
            <div class={classes}>
                <label for={props.label()}>{format!("{}:", props.label())}</label>
                <input
                    type={props.input_type()}
                    class="input-control"
                    name={props.label().to_lowercase().replace(" ", "_")}
                    value={self.current_value.as_ref().map_or_else(|| props.value().to_string(), |value| value.to_string())}
                    oninput={on_input}
                    onblur={on_blur}
                />
                <ul class="form-errors">
                    { for self.errors.iter().map(|error| {
                        let link = ctx.link().clone();

                        let on_delete =
                            Callback::from(move |error: String| {
                                link.send_message(InputMessage::RemoveError(error));
                            });
                        html! {
                            <InputError error={error.clone()} on_delete={on_delete}/>
                        }
                    })
                    }
                </ul>
            </div>
        }
    }
}
