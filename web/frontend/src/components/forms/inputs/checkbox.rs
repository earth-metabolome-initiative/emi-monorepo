//! Module providing a yew component that handles a basic input, which is meant
//! to be used in combination with BasicForm.

use wasm_bindgen::JsCast;
use web_common::api::ApiError;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct CheckboxProp {
    pub label: String,
    pub builder: Callback<bool>,
    pub errors: Vec<ApiError>,
    #[prop_or(true)]
    pub show_label: bool,
    #[prop_or_default]
    pub value: bool,
}

impl CheckboxProp {
    pub fn label(&self) -> String {
        self.label.clone()
    }

    pub fn normalized_label(&self) -> String {
        self.label.replace(" ", "_").to_lowercase()
    }
}

#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProp) -> Html {
    let on_input: Callback<InputEvent> = {
        let props = props.clone();
        Callback::from(move |input_event: InputEvent| {
            // We extract the current value of the input field
            props.builder.emit(
                input_event
                    .target()
                    .unwrap()
                    .dyn_into::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .checked(),
            );
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
            <input
                type="checkbox"
                class="input-control"
                name={props.normalized_label()}
                id={props.normalized_label()}
                oninput={on_input}
                checked={props.value}
            />
            <label for={props.normalized_label()} class="checkbox"></label>
        </>
    };

    html! {
        <div class="input-group">
            {if props.show_label {
                html! {
                    <div class="input-container">
                        {input_field}
                    </div>
                }
            } else {
                input_field
            }}
        </div>
    }
}
