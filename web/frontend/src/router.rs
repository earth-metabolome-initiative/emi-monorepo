//! This file is auto-generated. Do not edit it manually.
//!
//! This file contains the router for the frontend.
use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::*;
use uuid::Uuid;
use crate::components::BasicPages;
use crate::components::BasicPage;
use web_common::database::*;
use crate::components::forms::automatic_forms::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum AppRoute {
    #[at("/projects")]
    Projects,
    #[at("/projects/:id")]
    ProjectsView{id: i32},
    #[at("/projects/new")]
    ProjectsNew,
    #[at("/projects/:id/update")]
    ProjectsUpdate{id: i32},
    #[at("/sampled_individuals")]
    SampledIndividuals,
    #[at("/sampled_individuals/:id")]
    SampledIndividualsView{id: Uuid},
    #[at("/sampled_individuals/new")]
    SampledIndividualsNew,
    #[at("/sampled_individuals/:id/update")]
    SampledIndividualsUpdate{id: Uuid},
    #[at("/samples")]
    Samples,
    #[at("/samples/:id")]
    SamplesView{id: Uuid},
    #[at("/samples/new")]
    SamplesNew,
    #[at("/samples/:id/update")]
    SamplesUpdate{id: Uuid},
    #[at("/teams")]
    Teams,
    #[at("/teams/:id")]
    TeamsView{id: i32},
    #[at("/teams/new")]
    TeamsNew,
    #[at("/teams/:id/update")]
    TeamsUpdate{id: i32},
    #[at("/users")]
    Users,
    #[at("/users/:id")]
    UsersView{id: i32},
    #[at("/users/new")]
    UsersNew,
    #[at("/users/:id/update")]
    UsersUpdate{id: i32},
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// The switch to map each instance of the AppRoute to the corresponding page.
///
/// # Arguments
/// * `route` - The route to map.
///
pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Projects => {
            html! { <BasicPages<NestedProject> /> }
        }
        AppRoute::ProjectsView{id} => {
            html! { <BasicPage<NestedProject> id={PrimaryKey::from(id)} /> }
        }
        AppRoute::ProjectsNew => {
            html! { <CreateProjectForm /> }
        }
        AppRoute::ProjectsUpdate{id} => {
            html! { <UpdateProjectForm id={id} /> }
        }
        AppRoute::SampledIndividuals => {
            html! { <BasicPages<NestedSampledIndividual> /> }
        }
        AppRoute::SampledIndividualsView{id} => {
            html! { <BasicPage<NestedSampledIndividual> id={PrimaryKey::from(id)} /> }
        }
        AppRoute::SampledIndividualsNew => {
            html! { <CreateSampledIndividualForm /> }
        }
        AppRoute::SampledIndividualsUpdate{id} => {
            html! { <UpdateSampledIndividualForm id={id} /> }
        }
        AppRoute::Samples => {
            html! { <BasicPages<NestedSample> /> }
        }
        AppRoute::SamplesView{id} => {
            html! { <BasicPage<NestedSample> id={PrimaryKey::from(id)} /> }
        }
        AppRoute::SamplesNew => {
            html! { <CreateSampleForm /> }
        }
        AppRoute::SamplesUpdate{id} => {
            html! { <UpdateSampleForm id={id} /> }
        }
        AppRoute::Teams => {
            html! { <BasicPages<NestedTeam> /> }
        }
        AppRoute::TeamsView{id} => {
            html! { <BasicPage<NestedTeam> id={PrimaryKey::from(id)} /> }
        }
        AppRoute::TeamsNew => {
            html! { <CreateTeamForm /> }
        }
        AppRoute::TeamsUpdate{id} => {
            html! { <UpdateTeamForm id={id} /> }
        }
        AppRoute::Users => {
            html! { <BasicPages<User> /> }
        }
        AppRoute::UsersView{id} => {
            html! { <BasicPage<User> id={PrimaryKey::from(id)} /> }
        }
        AppRoute::UsersNew => {
            html! { <CreateUserForm /> }
        }
        AppRoute::UsersUpdate{id} => {
            html! { <UpdateUserForm id={id} /> }
        }
        AppRoute::Home => {
            html! { <Home /> }
        }
        AppRoute::Login => {
            html! { <Login /> }
        }
        AppRoute::NotFound => {
            html! { <NotFound /> }
        }
    }
}
