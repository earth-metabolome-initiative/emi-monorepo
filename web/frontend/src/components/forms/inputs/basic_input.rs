//! Module providing a yew component that handles a basic input, which is meant to be used in combination with BasicForm.

use std::fmt::Display;

use super::InputErrors;
use chrono::NaiveDateTime;
use wasm_bindgen::JsCast;
use web_common::api::ApiError;
use yew::html::IntoPropValue;
use yew::prelude::*;

#[derive(Clone, PartialEq, Copy, Eq, Debug)]
pub enum InputType {
    Text,
    Number,
    Textarea,
    DateTime,
}

impl Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputType::Text => write!(f, "text"),
            InputType::Number => write!(f, "number"),
            InputType::Textarea => write!(f, "textarea"),
            InputType::DateTime => write!(f, "datetime-local"),
        }
    }
}

impl IntoPropValue<std::option::Option<implicit_clone::unsync::IString>> for InputType {
    fn into_prop_value(self) -> std::option::Option<implicit_clone::unsync::IString> {
        Some(self.to_string().into())
    }
}

pub trait Inputtable: Clone + ToString + PartialEq {
    const INPUT_TYPE: InputType;
}

impl Inputtable for String {
    const INPUT_TYPE: InputType = InputType::Text;
}

impl Inputtable for i32 {
    const INPUT_TYPE: InputType = InputType::Number;
}

impl Inputtable for f64 {
    const INPUT_TYPE: InputType = InputType::Number;
}

impl Inputtable for NaiveDateTime {
    const INPUT_TYPE: InputType = InputType::DateTime;
}

#[derive(Clone, PartialEq, Properties)]
pub struct InputProp<Data: Inputtable> {
    pub label: String,
    pub builder: Callback<Option<String>>,
    pub errors: Vec<ApiError>,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub value: Option<Data>,
    #[prop_or(false)]
    pub optional: bool,
}

impl<Data: Inputtable> InputProp<Data> {
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

#[function_component(BasicInput)]
pub fn basic_input<Data: Inputtable>(props: &InputProp<Data>) -> Html {
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
        Data::INPUT_TYPE,
        extra_classes.map_or("".to_string(), |classes| format!(" {}", classes))
    );

    // We create a timeout so that when the user stops typing for at least
    // 500ms, the input field is validated.

    let on_input: Callback<InputEvent> = {
        let props = props.clone();
        Callback::from(move |input_event: InputEvent| {
            input_event.prevent_default();

            // We extract the current value of the input field
            let value = match Data::INPUT_TYPE {
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

    let value = props
        .value()
        .map_or_else(|| "".to_string(), |value| value.to_string());

    let label_classes = format!(
        "input-label{}",
        if props.optional {
            ""
        } else {
            " input-label-mandatory"
        }
    );

    let input_field = html! {
        <>
            <label for={props.normalized_label()} class={label_classes}>
                {props.label()}
            </label>
            {match Data::INPUT_TYPE {
                InputType::Textarea => html! {
                    <textarea
                        class="input-control"
                        name={props.normalized_label()}
                        id={props.normalized_label()}
                        value={value}
                        placeholder={props.placeholder.clone().unwrap_or_else(|| props.label())}
                        oninput={on_input}
                    ></textarea>
                },
                InputType::Number | InputType::Text => html! {
                    <input
                        type={InputType::Text}
                        class="input-control"
                        name={props.normalized_label()}
                        id={props.normalized_label()}
                        value={value}
                        placeholder={props.placeholder.clone().unwrap_or_else(|| props.label())}
                        oninput={on_input}
                    />
                },
                InputType::DateTime => html! {
                    <input
                        type={InputType::DateTime}
                        class="input-control"
                        name={props.normalized_label()}
                        id={props.normalized_label()}
                        value={value}
                        placeholder={props.placeholder.clone().unwrap_or_else(|| props.label())}
                        oninput={on_input}
                    />
                }
            }}
        </>
    };

    html! {
        <div class={classes}>
            <div class="input-container">
                {input_field}
            </div>
            <InputErrors errors={props.errors.clone()} />
        </div>
    }
}
