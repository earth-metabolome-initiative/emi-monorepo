//! This file is auto-generated. Do not edit it manually.
//!
//! This file contains the sidebar for the frontend.
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::use_store;
use web_common::database::*;
use crate::router::AppRoute;
use super::logout::Logout;
use crate::components::basic_page::PageLike;
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
                    <li class={if route == AppRoute::Observations { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::Observations}>
                            <i class={format!("fas fa-{}", NestedObservation::icon())}></i>
                             {'\u{00a0}'}
                            <span>{NestedObservation::section()}</span>
                        </Link<AppRoute>>
                    </li>
                    <li class={if route == AppRoute::Projects { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::Projects}>
                            <i class={format!("fas fa-{}", NestedProject::icon())}></i>
                             {'\u{00a0}'}
                            <span>{NestedProject::section()}</span>
                        </Link<AppRoute>>
                    </li>
                    <li class={if route == AppRoute::SampledIndividuals { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::SampledIndividuals}>
                            <i class={format!("fas fa-{}", NestedSampledIndividual::icon())}></i>
                             {'\u{00a0}'}
                            <span>{NestedSampledIndividual::section()}</span>
                        </Link<AppRoute>>
                    </li>
                    <li class={if route == AppRoute::Samples { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::Samples}>
                            <i class={format!("fas fa-{}", NestedSample::icon())}></i>
                             {'\u{00a0}'}
                            <span>{NestedSample::section()}</span>
                        </Link<AppRoute>>
                    </li>
                    <li class={if route == AppRoute::SpectraCollections { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::SpectraCollections}>
                            <i class={format!("fas fa-{}", NestedSpectraCollection::icon())}></i>
                             {'\u{00a0}'}
                            <span>{NestedSpectraCollection::section()}</span>
                        </Link<AppRoute>>
                    </li>
                    <li class={if route == AppRoute::Teams { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::Teams}>
                            <i class={format!("fas fa-{}", NestedTeam::icon())}></i>
                             {'\u{00a0}'}
                            <span>{NestedTeam::section()}</span>
                        </Link<AppRoute>>
                    </li>
                    <li class={if route == AppRoute::Users { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::Users}>
                            <i class={format!("fas fa-{}", User::icon())}></i>
                             {'\u{00a0}'}
                            <span>{User::section()}</span>
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
