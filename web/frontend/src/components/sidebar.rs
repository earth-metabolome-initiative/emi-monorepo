//! This file is auto-generated. Do not edit it manually.
//!
//! This file contains the sidebar for the frontend.
use super::logout::Logout;
use crate::components::basic_page::PageLike;
use crate::router::AppRoute;
use crate::stores::user_state::UserState;
use web_common::database::*;
use yew::prelude::*;
use yew_hooks::use_click_away;
use yew_router::prelude::*;
use yewdux::use_store;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct SidebarProps {
    pub visible: bool,
    pub onclose: Callback<()>,
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
            onclose.emit(());
        }
    });

    let sidebar_class = if props.visible {
        "sidebar"
    } else {
        "sidebar hidden"
    };

    html! {
        <div ref={node} class={sidebar_class}>
            <div class="sidebar-content">
                <ul>
                    <li class={if route == AppRoute::BioOttRanks { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::BioOttRanks}>
                            <i class={format!("fas fa-{}", NestedBioOttRank::icon())}></i>
                             {'\u{00a0}'}
                            <span>{NestedBioOttRank::section()}</span>
                        </Link<AppRoute>>
                    </li>
                    <li class={if route == AppRoute::BioOttTaxonItems { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::BioOttTaxonItems}>
                            <i class={format!("fas fa-{}", NestedBioOttTaxonItem::icon())}></i>
                             {'\u{00a0}'}
                            <span>{NestedBioOttTaxonItem::section()}</span>
                        </Link<AppRoute>>
                    </li>
                    <li class={if route == AppRoute::Countries { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::Countries}>
                            <i class={format!("fas fa-{}", Country::icon())}></i>
                             {'\u{00a0}'}
                            <span>{Country::section()}</span>
                        </Link<AppRoute>>
                    </li>
                    <li class={if route == AppRoute::Nameplates { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::Nameplates}>
                            <i class={format!("fas fa-{}", NestedNameplate::icon())}></i>
                             {'\u{00a0}'}
                            <span>{NestedNameplate::section()}</span>
                        </Link<AppRoute>>
                    </li>
                    <li class={if route == AppRoute::ObservationSubjects { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::ObservationSubjects}>
                            <i class={format!("fas fa-{}", NestedObservationSubject::icon())}></i>
                             {'\u{00a0}'}
                            <span>{NestedObservationSubject::section()}</span>
                        </Link<AppRoute>>
                    </li>
                    <li class={if route == AppRoute::Observations { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::Observations}>
                            <i class={format!("fas fa-{}", NestedObservation::icon())}></i>
                             {'\u{00a0}'}
                            <span>{NestedObservation::section()}</span>
                        </Link<AppRoute>>
                    </li>
                    <li class={if route == AppRoute::Organisms { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::Organisms}>
                            <i class={format!("fas fa-{}", NestedOrganism::icon())}></i>
                             {'\u{00a0}'}
                            <span>{NestedOrganism::section()}</span>
                        </Link<AppRoute>>
                    </li>
                    <li class={if route == AppRoute::Organizations { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::Organizations}>
                            <i class={format!("fas fa-{}", NestedOrganization::icon())}></i>
                             {'\u{00a0}'}
                            <span>{NestedOrganization::section()}</span>
                        </Link<AppRoute>>
                    </li>
                    <li class={if route == AppRoute::Projects { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::Projects}>
                            <i class={format!("fas fa-{}", NestedProject::icon())}></i>
                             {'\u{00a0}'}
                            <span>{NestedProject::section()}</span>
                        </Link<AppRoute>>
                    </li>
                    <li class={if route == AppRoute::SampleContainers { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::SampleContainers}>
                            <i class={format!("fas fa-{}", NestedSampleContainer::icon())}></i>
                             {'\u{00a0}'}
                            <span>{NestedSampleContainer::section()}</span>
                        </Link<AppRoute>>
                    </li>
                    <li class={if route == AppRoute::SampleStates { "active" } else { "" }}>
                        <Link<AppRoute> to={AppRoute::SampleStates}>
                            <i class={format!("fas fa-{}", NestedSampleState::icon())}></i>
                             {'\u{00a0}'}
                            <span>{NestedSampleState::section()}</span>
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
                            <i class={format!("fas fa-{}", NestedUser::icon())}></i>
                             {'\u{00a0}'}
                            <span>{NestedUser::section()}</span>
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
