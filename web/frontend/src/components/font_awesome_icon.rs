//! HTML rendering of an OAuth login provider object.
use yew::prelude::*;

/// Properties for the `FontAwesomeIcon` components
#[derive(Clone, PartialEq, Properties)]
pub struct FAProps {
    /// The icon name to be rendered.
    pub icon: AttrValue,
}

#[function_component(BrandIcon)]
/// Renders a Font Awesome brand icon.
pub fn brand_icon(FAProps { icon }: &FAProps) -> Html {
    html! {
        <i class={format!("fa-brands fa-{icon}")} aria-hidden="true"></i>
    }
}

#[function_component(SolidIcon)]
/// Renders a Font Awesome solid icon.
pub fn solid_icon(FAProps { icon }: &FAProps) -> Html {
    html! {
        <i class={format!("fa-solid fa-{icon}")} aria-hidden="true"></i>
    }
}
