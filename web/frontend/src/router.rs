//! This file is auto-generated. Do not edit it manually.
//!
//! This file contains the router for the frontend.
use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::*;
use uuid::Uuid;
use crate::components::BasicList;
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
    #[at("/projects/new/parent_project/:parent_project_id")]
    ProjectsNewWithParentProject{parent_project_id: i32},
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
    #[at("/samples/new/sampled_by/:sampled_by")]
    SamplesNewWithSampledBy{sampled_by: i32},
    #[at("/samples/:id/update")]
    SamplesUpdate{id: Uuid},
    #[at("/teams")]
    Teams,
    #[at("/teams/:id")]
    TeamsView{id: i32},
    #[at("/teams/new")]
    TeamsNew,
    #[at("/teams/new/parent_team/:parent_team_id")]
    TeamsNewWithParentTeam{parent_team_id: i32},
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
            html! { <BasicList<NestedProject> /> }
        }
        AppRoute::ProjectsView{id} => {
            html! { <BasicPage<NestedProject> id={PrimaryKey::from(id)} /> }
        }
        AppRoute::ProjectsNew => {
            html! { <CreateProjectForm /> }
        }
        AppRoute::ProjectsNewWithParentProject{parent_project_id} => {
            html! { <CreateProjectForm parent_project_id={parent_project_id} /> }
        }
        AppRoute::ProjectsUpdate{id} => {
            html! { <UpdateProjectForm id={id} /> }
        }
        AppRoute::SampledIndividuals => {
            html! { <BasicList<NestedSampledIndividual> /> }
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
            html! { <BasicList<NestedSample> /> }
        }
        AppRoute::SamplesView{id} => {
            html! { <BasicPage<NestedSample> id={PrimaryKey::from(id)} /> }
        }
        AppRoute::SamplesNew => {
            html! { <CreateSampleForm /> }
        }
        AppRoute::SamplesNewWithSampledBy{sampled_by} => {
            html! { <CreateSampleForm sampled_by={sampled_by} /> }
        }
        AppRoute::SamplesUpdate{id} => {
            html! { <UpdateSampleForm id={id} /> }
        }
        AppRoute::Teams => {
            html! { <BasicList<NestedTeam> /> }
        }
        AppRoute::TeamsView{id} => {
            html! { <BasicPage<NestedTeam> id={PrimaryKey::from(id)} /> }
        }
        AppRoute::TeamsNew => {
            html! { <CreateTeamForm /> }
        }
        AppRoute::TeamsNewWithParentTeam{parent_team_id} => {
            html! { <CreateTeamForm parent_team_id={parent_team_id} /> }
        }
        AppRoute::TeamsUpdate{id} => {
            html! { <UpdateTeamForm id={id} /> }
        }
        AppRoute::Users => {
            html! { <BasicList<User> /> }
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
