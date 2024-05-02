//! Module providing a yew component that handles a basic input, which is meant to be used in combination with BasicForm.

use std::fmt::Display;

use super::InputErrors;
use wasm_bindgen::JsCast;
use web_common::api::ApiError;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum InputType {
    Text,
    Number,
    Textarea,
}

impl Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputType::Text => write!(f, "text"),
            InputType::Number => write!(f, "number"),
            InputType::Textarea => write!(f, "textarea"),
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct InputProp {
    pub label: String,
    pub builder: Callback<Option<String>>,
    pub errors: Vec<ApiError>,
    #[prop_or(true)]
    pub show_label: bool,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub value: Option<String>,
    pub input_type: InputType,
    #[prop_or_default]
    pub step: Option<f64>,
    #[prop_or(false)]
    pub optional: bool,
}

impl InputProp {
    pub fn label(&self) -> String {
        self.label.clone()
    }

    pub fn normalized_label(&self) -> String {
        self.label.replace(" ", "_").to_lowercase()
    }

    pub fn input_type(&self) -> InputType {
        self.input_type.clone()
    }
}

#[function_component(BasicInput)]
pub fn basic_input(props: &InputProp) -> Html {
    let extra_classes: Option<&str> = if props.errors.is_empty() {
        if props.value.is_some() {
            Some("input-group-valid")
        } else {
            None
        }
    } else {
        Some("input-group-invalid")
    };

    let classes = format!(
        "input-group {}{}",
        props.input_type(),
        extra_classes.map_or("".to_string(), |classes| format!(" {}", classes))
    );

    // We create a timeout so that when the user stops typing for at least
    // 500ms, the input field is validated.

    let on_input: Callback<InputEvent> = {
        let props = props.clone();
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

            // We update the current value of the input field
            props
                .builder
                .emit(if value.is_empty() { None } else { Some(value) });
        })
    };

    let input_value = props
        .value
        .as_ref()
        .map_or_else(|| "".to_string(), |value| value.to_string());
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
                    ></textarea>
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
            <InputErrors errors={props.errors.clone()} />
        </div>
    }
}
