//! A sidebar Yew component is responsible for rendering the sidebar on the left side of the page.
//!

use super::logout::Logout;
use crate::stores::UserState;
use web_common::api::oauth::jwt_cookies::FULL_LOGOUT_ENDPOINT;
use yew::prelude::*;
use yewdux::use_store;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct SidebarProps {
    pub visible: bool,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let (user, _) = use_store::<UserState>();

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
                    if user.is_logged_in() {
                        <li><Logout /></li>
                    }
                </ul>
            </div>
        </div>
    }
}
