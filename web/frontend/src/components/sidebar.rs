//! A sidebar Yew component is responsible for rendering the sidebar on the left side of the page.
//!

use super::logout::Logout;
use crate::stores::user_state::UserState;
use yew::prelude::*;
use yewdux::use_store;
use crate::router::AppRoute;
use yew_router::prelude::*;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct SidebarProps {
    pub visible: bool,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let (user, _) = use_store::<UserState>();
    let navigator = use_navigator().unwrap();

    let sidebar_class = if props.visible {
        "sidebar"
    } else {
        "sidebar hidden"
    };

    let onclick_home = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&AppRoute::Home);
        })
    };

    html! {
        <div class={sidebar_class}>
            <div class="sidebar-content">
                <ul>
                    <li>
                        <button onclick={onclick_home}>
                            {"Home"}
                        </button>
                    </li>
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
