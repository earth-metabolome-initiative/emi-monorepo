//! This file is auto-generated. Do not edit it manually.
//!
//! This file contains the sidebar for the frontend.
use yew::prelude::*;
use yew_hooks::use_click_away;
use yew_router::prelude::*;
use yewdux::use_store;

use super::logout::Logout;
use crate::{components::basic_page::PageLike, router::AppRoute, stores::user_state::UserState};

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct SidebarProps {
    pub visible: bool,
    pub onclose: Callback<bool>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let (user, _) = use_store::<UserState>();
    let route: AppRoute = use_route().unwrap_or_default();
    let node = use_node_ref();
    let onclose = props.onclose.clone();
    let visible = props.visible;
    use_click_away(node.clone(), move |_: Event| {
        if visible {
            onclose.emit(!visible);
        }
    });

    let sidebar_class = if props.visible { "sidebar" } else { "sidebar hidden" };
    let on_click_close = {
        let onclose = props.onclose.clone();
        Callback::from(move |_| {
            onclose.emit(false);
        })
    };

    html! {
        <div ref={node} class={sidebar_class}>
            <div class="sidebar-content">
                <ul>
                    <li class={if route == AppRoute::Home {{ "active" }} else {{ "" }}} onclick={&on_click_close}>
                        <Link<AppRoute> to={AppRoute::Home}>
                            <i class="fas fa-home"></i>
                             {'\u{00a0}'}
                            <span>{"Home"}</span>
                        </Link<AppRoute>>
                    </li>
                    {if user.has_user() {
                        html! {
                            <li><Logout /></li>
                        }
                    } else {
                        html! {
                            <li onclick={&on_click_close}>
                                <Link<AppRoute> to={AppRoute::Login}>
                                    <i class="fas fa-right-to-bracket"></i>
                                     {'\u{00a0}'}
                                    <span>{"Login"}</span>
                                </Link<AppRoute>>
                            </li>
                        }
                    }}
                </ul>
            </div>
        </div>
    }
}
