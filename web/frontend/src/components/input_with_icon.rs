//! A reusable component showing a flat-designed input field with an icon on the right that looks like
//! part of the input field. The icon is specified using Font Awesome classes. The input field is
//! responsive and will shrink and grow depending on the size of the screen.
//!

use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct InputWithIconProperties {
    pub icon: String,
    pub placeholder: Option<String>,
}

#[function_component(InputWithIcon)]
pub fn input_with_icon(props: &InputWithIconProperties) -> Html {
    html! {
        <div class="input-with-icon">
            <input type="text" placeholder={props.placeholder.clone().unwrap_or_else(||"".to_string())} />
            <i class={format!("fa fa-{}", props.icon)}></i>
        </div>
    }
}
