//! File describing the hamburger menu component.
//!
//! On the click event, the hamburger should get the additional class "is-active" and the navigation should be shown.

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct HamburgerProperties {
    pub is_active: bool,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Hamburger)]
pub fn hamburger(props: &HamburgerProperties) -> Html {
    let classes = if props.is_active {
        "hamburger hamburger--elastic is-active"
    } else {
        "hamburger hamburger--elastic"
    };

    let onclick = props.onclick.reform(|mouse_event| mouse_event);

    html! {
        <button class={classes} type="button" aria-label="Menu" aria-controls="navigation" onclick={onclick}>
            <span class="hamburger-box">
                <span class="hamburger-inner"></span>
            </span>
        </button>
    }
}
