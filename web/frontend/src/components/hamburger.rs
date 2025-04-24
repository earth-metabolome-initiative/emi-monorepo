//! File describing the hamburger menu component.
//!
//! On the click event, the hamburger should get the additional class
//! "is-active" and the navigation should be shown.

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
/// Properties for the Hamburger component.
pub struct HamburgerProperties {
    /// Whether the hamburger menu is active or not.
    pub is_active: bool,
    /// Callback function to handle the click event.
    pub onclick: Callback<bool>,
}

#[function_component(Hamburger)]
/// Hamburger menu component.
pub fn hamburger(props: &HamburgerProperties) -> Html {
    let classes = if props.is_active {
        "hamburger hamburger--elastic is-active"
    } else {
        "hamburger hamburger--elastic"
    };

    let visible = props.is_active;
    let onclick = props.onclick.reform(move |e: MouseEvent| {
        e.prevent_default();
        e.stop_immediate_propagation();
        !visible
    });

    html! {
        <button class={classes} type="button" aria-label="Menu" aria-controls="navigation" onclick={onclick}>
            <span class="hamburger-box">
                <span class="hamburger-inner"></span>
            </span>
        </button>
    }
}
