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
                    <li class={if route == AppRoute::ContainerHorizontalRules { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::ContainerHorizontalRules}>
                            {"Container horizontal rules"}
                        </Link<AppRoute>>
                        {if route == AppRoute::ContainerHorizontalRules && user.has_user() {
                            html! {
                                <ul>
                                    <li>
                                        <Link<AppRoute> to={AppRoute::ContainerHorizontalRulesNew}>
                                            {"New Container horizontal rule"}
                                        </Link<AppRoute>>
                                    </li>
                                </ul>
                            }
                        } else {
                            html! {<></>}
                        }}
                    </li>
                    <li class={if route == AppRoute::ContainerVerticalRules { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::ContainerVerticalRules}>
                            {"Container vertical rules"}
                        </Link<AppRoute>>
                        {if route == AppRoute::ContainerVerticalRules && user.has_user() {
                            html! {
                                <ul>
                                    <li>
                                        <Link<AppRoute> to={AppRoute::ContainerVerticalRulesNew}>
                                            {"New Container vertical rule"}
                                        </Link<AppRoute>>
                                    </li>
                                </ul>
                            }
                        } else {
                            html! {<></>}
                        }}
                    </li>
                    <li class={if route == AppRoute::ItemCategories { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::ItemCategories}>
                            {"Item categories"}
                        </Link<AppRoute>>
                        {if route == AppRoute::ItemCategories && user.has_user() {
                            html! {
                                <ul>
                                    <li>
                                        <Link<AppRoute> to={AppRoute::ItemCategoriesNew}>
                                            {"New Item category"}
                                        </Link<AppRoute>>
                                    </li>
                                </ul>
                            }
                        } else {
                            html! {<></>}
                        }}
                    </li>
                    <li class={if route == AppRoute::Procedures { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::Procedures}>
                            {"Procedures"}
                        </Link<AppRoute>>
                        {if route == AppRoute::Procedures && user.has_user() {
                            html! {
                                <ul>
                                    <li>
                                        <Link<AppRoute> to={AppRoute::ProceduresNew}>
                                            {"New Procedure"}
                                        </Link<AppRoute>>
                                    </li>
                                </ul>
                            }
                        } else {
                            html! {<></>}
                        }}
                    </li>
                    <li class={if route == AppRoute::ProjectRequirements { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::ProjectRequirements}>
                            {"Project requirements"}
                        </Link<AppRoute>>
                        {if route == AppRoute::ProjectRequirements && user.has_user() {
                            html! {
                                <ul>
                                    <li>
                                        <Link<AppRoute> to={AppRoute::ProjectRequirementsNew}>
                                            {"New Project requirement"}
                                        </Link<AppRoute>>
                                    </li>
                                </ul>
                            }
                        } else {
                            html! {<></>}
                        }}
                    </li>
                    <li class={if route == AppRoute::Projects { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::Projects}>
                            {"Projects"}
                        </Link<AppRoute>>
                        {if route == AppRoute::Projects && user.has_user() {
                            html! {
                                <ul>
                                    <li>
                                        <Link<AppRoute> to={AppRoute::ProjectsNew}>
                                            {"New Project"}
                                        </Link<AppRoute>>
                                    </li>
                                </ul>
                            }
                        } else {
                            html! {<></>}
                        }}
                    </li>
                    <li class={if route == AppRoute::SampledIndividuals { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::SampledIndividuals}>
                            {"Sampled individuals"}
                        </Link<AppRoute>>
                        {if route == AppRoute::SampledIndividuals && user.has_user() {
                            html! {
                                <ul>
                                    <li>
                                        <Link<AppRoute> to={AppRoute::SampledIndividualsNew}>
                                            {"New Sampled individual"}
                                        </Link<AppRoute>>
                                    </li>
                                </ul>
                            }
                        } else {
                            html! {<></>}
                        }}
                    </li>
                    <li class={if route == AppRoute::Samples { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::Samples}>
                            {"Samples"}
                        </Link<AppRoute>>
                        {if route == AppRoute::Samples && user.has_user() {
                            html! {
                                <ul>
                                    <li>
                                        <Link<AppRoute> to={AppRoute::SamplesNew}>
                                            {"New Sample"}
                                        </Link<AppRoute>>
                                    </li>
                                </ul>
                            }
                        } else {
                            html! {<></>}
                        }}
                    </li>
                    <li class={if route == AppRoute::SamplingProcedures { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::SamplingProcedures}>
                            {"Sampling procedures"}
                        </Link<AppRoute>>
                        {if route == AppRoute::SamplingProcedures && user.has_user() {
                            html! {
                                <ul>
                                    <li>
                                        <Link<AppRoute> to={AppRoute::SamplingProceduresNew}>
                                            {"New Sampling procedure"}
                                        </Link<AppRoute>>
                                    </li>
                                </ul>
                            }
                        } else {
                            html! {<></>}
                        }}
                    </li>
                    <li class={if route == AppRoute::Teams { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::Teams}>
                            {"Teams"}
                        </Link<AppRoute>>
                        {if route == AppRoute::Teams && user.has_user() {
                            html! {
                                <ul>
                                    <li>
                                        <Link<AppRoute> to={AppRoute::TeamsNew}>
                                            {"New Team"}
                                        </Link<AppRoute>>
                                    </li>
                                </ul>
                            }
                        } else {
                            html! {<></>}
                        }}
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
            <Logout />
        </div>
    }
}
