//! HTML rendering of an OAuth login provider object.
use yew::prelude::*;

#[function_component(BrandIcon)]
/// Renders a Font Awesome icon.
pub fn brand_icon(icon: &String) -> Html {
    html! {
        <i class={format!("fa-brands fa-{icon}")} aria-hidden="true"></i>
    }
}
