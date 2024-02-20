//! A sidebar Yew component is responsible for rendering the sidebar on the left side of the page.
//!

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct SidebarProps {
    pub visible: bool,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let sidebar_class = if props.visible {
        "sidebar"
    } else {
        "sidebar hidden"
    };

    html! {
        <div class={sidebar_class}>
            <div class="sidebar-content">
                <ul>
                    <li><a href="#">{"Home"}</a></li>
                    <li><a href="#">{"About"}</a></li>
                    <li><a href="#">{"Services"}</a></li>
                    <li><a href="#">{"Contact"}</a></li>
                </ul>
            </div>
        </div>
    }
}
