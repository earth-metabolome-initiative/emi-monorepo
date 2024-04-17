//! Module providing a yew component that handles a basic input, which is meant to be used in combination with BasicForm.

use std::collections::HashSet;
use std::fmt::Display;

use super::InputErrors;
use gloo::timers::callback::Timeout;
use validator::Validate;
use wasm_bindgen::JsCast;
use web_common::custom_validators::validation_errors::ValidationErrorToString;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum InputType {
    Text,
    Number,
    Checkbox,
    Textarea,
}

impl Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputType::Text => write!(f, "text"),
            InputType::Number => write!(f, "number"),
            InputType::Checkbox => write!(f, "checkbox"),
            InputType::Textarea => write!(f, "textarea"),
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct InputProp<Data>
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
    pub input_type: InputType,
    #[prop_or_default]
    pub step: Option<f64>,
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

    pub fn normalized_label(&self) -> String {
        self.label.replace(" ", "_").to_lowercase()
    }

    pub fn value(&self) -> Option<Data> {
        self.value.clone()
    }

    pub fn input_type(&self) -> InputType {
        self.input_type.clone()
    }
}

pub struct BasicInput<Data> {
    errors: HashSet<String>,
    current_value: Option<String>,
    is_valid: Option<bool>,
    validation_timeout: Option<Timeout>,
    _data: std::marker::PhantomData<Data>,
}

pub enum InputMessage<Data> {
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

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            errors: HashSet::new(),
            is_valid: None,
            current_value: None,
            validation_timeout: None,
            _data: std::marker::PhantomData,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
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

        let extra_classes: Option<&str> = match self.is_valid {
            Some(true) => {
                if let (Some(previous), Some(current)) = (
                    ctx.props().value(),
                    self.current_value
                        .as_ref()
                        .and_then(|value| Data::try_from(value.clone()).ok()),
                ) {
                    if previous != current {
                        Some("input-group-valid")
                    } else {
                        None
                    }
                } else {
                    Some("input-group-valid")
                }
            }
            Some(false) => Some("input-group-invalid"),
            None => None,
        };
        let classes = format!(
            "input-group {}{}",
            props.input_type(),
            extra_classes.map_or("".to_string(), |classes| format!(" {}", classes))
        );

        // We create a timeout so that when the user stops typing for at least
        // 500ms, the input field is validated.

        let on_input: Callback<InputEvent> = {
            let link = ctx.link().clone();
            let props = ctx.props().clone();
            Callback::from(move |input_event: InputEvent| {
                input_event.prevent_default();

                // We extract the current value of the input field
                let value = match props.input_type() {
                    InputType::Textarea => input_event
                        .target()
                        .unwrap()
                        .dyn_into::<web_sys::HtmlTextAreaElement>()
                        .unwrap()
                        .value(),
                    _ => input_event
                        .target()
                        .unwrap()
                        .dyn_into::<web_sys::HtmlInputElement>()
                        .unwrap()
                        .value(),
                };

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
                let value = match props.input_type() {
                    InputType::Textarea => input_event
                        .target()
                        .unwrap()
                        .dyn_into::<web_sys::HtmlTextAreaElement>()
                        .unwrap()
                        .value(),
                    _ => input_event
                        .target()
                        .unwrap()
                        .dyn_into::<web_sys::HtmlInputElement>()
                        .unwrap()
                        .value(),
                };

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

        let on_delete = {
            let link = ctx.link().clone();
            Callback::from(move |error: String| {
                link.send_message(InputMessage::RemoveError(error));
            })
        };

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
                {match props.input_type() {
                    InputType::Textarea => html! {
                        <textarea
                            class="input-control"
                            name={props.normalized_label()}
                            id={props.normalized_label()}
                            value={input_value}
                            placeholder={props.placeholder.clone().unwrap_or_else(|| props.label())}
                            oninput={on_input}
                            onblur={on_blur}
                        ></textarea>
                    },
                    InputType::Checkbox => html! {
                        <>
                        <label for={props.normalized_label()} class="checkbox"></label>
                        <input
                            type="checkbox"
                            class="input-control"
                            name={props.normalized_label()}
                            id={props.normalized_label()}
                            checked={input_value == "on"}
                            placeholder={props.placeholder.clone().unwrap_or_else(|| props.label())}
                            oninput={on_input}
                            onblur={on_blur}
                        />
                        </>
                    },
                    InputType::Number | InputType::Text => html! {
                        <input
                            type={props.input_type().to_string()}
                            class="input-control"
                            name={props.normalized_label()}
                            id={props.normalized_label()}
                            value={input_value}
                            placeholder={props.placeholder.clone().unwrap_or_else(|| props.label())}
                            step={props.step.map_or_else(|| "".to_string(), |step| step.to_string())}
                            oninput={on_input}
                            onblur={on_blur}
                        />
                    }
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
