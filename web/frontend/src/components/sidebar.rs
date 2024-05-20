//! This file is auto-generated. Do not edit it manually.
//!
//! This file contains the sidebar for the frontend.
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::use_store;
use crate::router::AppRoute;
use super::logout::Logout;
use crate::stores::user_state::UserState;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct SidebarProps {
    pub visible: bool,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let (user, _) = use_store::<UserState>();
    let route: AppRoute = use_route().unwrap_or_default();

    let sidebar_class = if props.visible {
        "sidebar"
    } else {
        "sidebar hidden"
    };

    html! {
        <div class={sidebar_class}>
            <div class="sidebar-content">
                <ul>
                    <li class={if route == AppRoute::Projects { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::Projects}>
                            {"Projects"}
                        </Link<AppRoute>>
                    </li>
                    <li class={if route == AppRoute::SampledIndividuals { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::SampledIndividuals}>
                            {"Sampled individuals"}
                        </Link<AppRoute>>
                    </li>
                    <li class={if route == AppRoute::Samples { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::Samples}>
                            {"Samples"}
                        </Link<AppRoute>>
                    </li>
                    <li class={if route == AppRoute::SpectraCollections { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::SpectraCollections}>
                            {"Spectra collections"}
                        </Link<AppRoute>>
                    </li>
                    <li class={if route == AppRoute::Teams { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::Teams}>
                            {"Teams"}
                        </Link<AppRoute>>
                    </li>
                    <li class={if route == AppRoute::Users { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::Users}>
                            {"Users"}
                        </Link<AppRoute>>
                    </li>
                    {if user.has_user() {
                        html! {
                            <li><Logout /></li>
                        }
                    } else {
                        html! {
                            <li>
                                <Link<AppRoute> to={AppRoute::Login}>
                                    {"Login"}
                                </Link<AppRoute>>
                            </li>
                        }
                    }}
                </ul>
            </div>
        </div>
    }
}
