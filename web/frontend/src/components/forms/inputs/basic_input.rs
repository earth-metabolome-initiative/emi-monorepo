//! Module providing a yew component that handles a basic input, which is meant to be used in combination with BasicForm.

use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct InputProp {
    pub label: String,
    pub value: String,
    pub input_type: String,
}

impl InputProp {
    pub fn label(&self) -> String {
        self.label.clone()
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn input_type(&self) -> String {
        self.input_type.clone()
    }
}

#[function_component(BasicInput)]
pub fn basic_input(input: &InputProp) -> Html {
    html! {
        <div class="form-group">
            <label for={input.label()}>{format!("{}:", input.label())}</label>
            <input
                type={input.input_type()}
                class="form-control"
                name={input.label().to_lowercase().replace(" ", "_")}
                value={input.value().to_string()}
            />
        </div>
    }
}
