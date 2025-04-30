//! HTML rendering of an OAuth login provider object.
use font_awesome_icons::FAIcon as FontAwesomeIcon;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
/// Properties for the Font Awesome icon component.
pub struct FAIconProp {
    /// The underlying Font Awesome icon name.
    pub icon: FontAwesomeIcon,
}

#[function_component(FAIcon)]
/// Renders a Font Awesome icon.
pub fn fa_icon(props: &FAIconProp) -> Html {
    html! {
        <i class={format!("fa-solid fa-{}", props.icon.class())} aria-hidden="true"></i>
    }
}
