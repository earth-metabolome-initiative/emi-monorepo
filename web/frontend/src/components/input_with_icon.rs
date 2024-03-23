//! A reusable component showing a flat-designed input field with an icon on the right that looks like
//! part of the input field. The icon is specified using Font Awesome classes. The input field is
//! responsive and will shrink and grow depending on the size of the screen.

use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct InputWithIconProperties {
    pub icon: String,
    pub placeholder: Option<String>,
    pub onsubmit: Callback<MouseEvent>,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub button_disabled: bool,
    #[prop_or_default]
    pub input_disabled: bool,
}

#[function_component(InputWithIcon)]
pub fn input_with_icon(props: &InputWithIconProperties) -> Html {
    let onsubmit = props.onsubmit.reform(|mouse_event| mouse_event);
    let oninput = props.oninput.reform(|input_event| input_event);

    html! {
        <div class="input-with-icon">
            <input type="text" placeholder={props.placeholder.clone().unwrap_or_else(||"".to_string())} disabled={props.input_disabled} oninput={oninput} />
            <button type="button" disabled={props.button_disabled} onclick={onsubmit}>
                <i class={format!("fa fa-{}", props.icon)}></i>
            </button>
        </div>
    }
}
