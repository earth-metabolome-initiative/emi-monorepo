//! This file is auto-generated. Do not edit it manually.
//!
//! This file contains the router for the frontend.
use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::*;
use uuid::Uuid;
use crate::components::BasicList;
use web_common::database::*;
use crate::components::forms::automatic_forms::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum AppRoute {
    #[at("/bio_ott_ranks")]
    BioOttRanks,
    #[at("/bio_ott_ranks/:id")]
    BioOttRanksView{id: i32},
    #[at("/bio_ott_taxon_items")]
    BioOttTaxonItems,
    #[at("/bio_ott_taxon_items/:id")]
    BioOttTaxonItemsView{id: i32},
    #[at("/countries")]
    Countries,
    #[at("/countries/:id")]
    CountriesView{id: i32},
    #[at("/observations")]
    Observations,
    #[at("/observations/:id")]
    ObservationsView{id: Uuid},
    #[at("/observations/new")]
    ObservationsNew,
    #[at("/observations/new/project/:project_id")]
    ObservationsNewWithProject{project_id: i32},
    #[at("/observations/new/individual/:individual_id")]
    ObservationsNewWithIndividual{individual_id: Uuid},
    #[at("/observations/:id/update")]
    ObservationsUpdate{id: Uuid},
    #[at("/organizations")]
    Organizations,
    #[at("/organizations/:id")]
    OrganizationsView{id: i32},
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
    #[at("/sample_containers")]
    SampleContainers,
    #[at("/sample_containers/:id")]
    SampleContainersView{id: i32},
    #[at("/sample_containers/new")]
    SampleContainersNew,
    #[at("/sample_containers/new/project/:project_id")]
    SampleContainersNewWithProject{project_id: i32},
    #[at("/sample_containers/:id/update")]
    SampleContainersUpdate{id: i32},
    #[at("/sample_states")]
    SampleStates,
    #[at("/sample_states/:id")]
    SampleStatesView{id: i32},
    #[at("/sampled_individuals")]
    SampledIndividuals,
    #[at("/sampled_individuals/:id")]
    SampledIndividualsView{id: Uuid},
    #[at("/sampled_individuals/new")]
    SampledIndividualsNew,
    #[at("/sampled_individuals/new/project/:project_id")]
    SampledIndividualsNewWithProject{project_id: i32},
    #[at("/sampled_individuals/:id/update")]
    SampledIndividualsUpdate{id: Uuid},
    #[at("/samples")]
    Samples,
    #[at("/samples/:id")]
    SamplesView{id: Uuid},
    #[at("/samples/new")]
    SamplesNew,
    #[at("/samples/new/container/:container_id")]
    SamplesNewWithContainer{container_id: i32},
    #[at("/samples/new/project/:project_id")]
    SamplesNewWithProject{project_id: i32},
    #[at("/samples/new/sampled_by/:sampled_by")]
    SamplesNewWithSampledBy{sampled_by: i32},
    #[at("/samples/:id/update")]
    SamplesUpdate{id: Uuid},
    #[at("/spectra_collections")]
    SpectraCollections,
    #[at("/spectra_collections/:id")]
    SpectraCollectionsView{id: i32},
    #[at("/spectra_collections/new")]
    SpectraCollectionsNew,
    #[at("/spectra_collections/new/sample/:sample_id")]
    SpectraCollectionsNewWithSample{sample_id: Uuid},
    #[at("/spectra_collections/:id/update")]
    SpectraCollectionsUpdate{id: i32},
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
        AppRoute::BioOttRanks => {
            html! { <BasicList<NestedBioOttRank> /> }
        }
        AppRoute::BioOttRanksView{id} => {
            html! { <BioOttRankPage id = {id} /> }
        }
        AppRoute::BioOttTaxonItems => {
            html! { <BasicList<NestedBioOttTaxonItem> /> }
        }
        AppRoute::BioOttTaxonItemsView{id} => {
            html! { <BioOttTaxonItemPage id = {id} /> }
        }
        AppRoute::Countries => {
            html! { <BasicList<Country> /> }
        }
        AppRoute::CountriesView{id} => {
            html! { <CountryPage id = {id} /> }
        }
        AppRoute::Observations => {
            html! { <BasicList<NestedObservation> /> }
        }
        AppRoute::ObservationsView{id} => {
            html! { <ObservationPage id = {id} /> }
        }
        AppRoute::ObservationsNew => {
            html! { <CreateObservationForm /> }
        }
        AppRoute::ObservationsNewWithProject{project_id} => {
            html! { <CreateObservationForm project_id={project_id} /> }
        }
        AppRoute::ObservationsNewWithIndividual{individual_id} => {
            html! { <CreateObservationForm individual_id={individual_id} /> }
        }
        AppRoute::ObservationsUpdate{id} => {
            html! { <UpdateObservationForm id={id} /> }
        }
        AppRoute::Organizations => {
            html! { <BasicList<NestedOrganization> /> }
        }
        AppRoute::OrganizationsView{id} => {
            html! { <OrganizationPage id = {id} /> }
        }
        AppRoute::Projects => {
            html! { <BasicList<NestedProject> /> }
        }
        AppRoute::ProjectsView{id} => {
            html! { <ProjectPage id = {id} /> }
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
        AppRoute::SampleContainers => {
            html! { <BasicList<NestedSampleContainer> /> }
        }
        AppRoute::SampleContainersView{id} => {
            html! { <SampleContainerPage id = {id} /> }
        }
        AppRoute::SampleContainersNew => {
            html! { <CreateSampleContainerForm /> }
        }
        AppRoute::SampleContainersNewWithProject{project_id} => {
            html! { <CreateSampleContainerForm project_id={project_id} /> }
        }
        AppRoute::SampleContainersUpdate{id} => {
            html! { <UpdateSampleContainerForm id={id} /> }
        }
        AppRoute::SampleStates => {
            html! { <BasicList<NestedSampleState> /> }
        }
        AppRoute::SampleStatesView{id} => {
            html! { <SampleStatePage id = {id} /> }
        }
        AppRoute::SampledIndividuals => {
            html! { <BasicList<NestedSampledIndividual> /> }
        }
        AppRoute::SampledIndividualsView{id} => {
            html! { <SampledIndividualPage id = {id} /> }
        }
        AppRoute::SampledIndividualsNew => {
            html! { <CreateSampledIndividualForm /> }
        }
        AppRoute::SampledIndividualsNewWithProject{project_id} => {
            html! { <CreateSampledIndividualForm project_id={project_id} /> }
        }
        AppRoute::SampledIndividualsUpdate{id} => {
            html! { <UpdateSampledIndividualForm id={id} /> }
        }
        AppRoute::Samples => {
            html! { <BasicList<NestedSample> /> }
        }
        AppRoute::SamplesView{id} => {
            html! { <SamplePage id = {id} /> }
        }
        AppRoute::SamplesNew => {
            html! { <CreateSampleForm /> }
        }
        AppRoute::SamplesNewWithContainer{container_id} => {
            html! { <CreateSampleForm container_id={container_id} /> }
        }
        AppRoute::SamplesNewWithProject{project_id} => {
            html! { <CreateSampleForm project_id={project_id} /> }
        }
        AppRoute::SamplesNewWithSampledBy{sampled_by} => {
            html! { <CreateSampleForm sampled_by={sampled_by} /> }
        }
        AppRoute::SamplesUpdate{id} => {
            html! { <UpdateSampleForm id={id} /> }
        }
        AppRoute::SpectraCollections => {
            html! { <BasicList<NestedSpectraCollection> /> }
        }
        AppRoute::SpectraCollectionsView{id} => {
            html! { <SpectraCollectionPage id = {id} /> }
        }
        AppRoute::SpectraCollectionsNew => {
            html! { <CreateSpectraCollectionForm /> }
        }
        AppRoute::SpectraCollectionsNewWithSample{sample_id} => {
            html! { <CreateSpectraCollectionForm sample_id={sample_id} /> }
        }
        AppRoute::SpectraCollectionsUpdate{id} => {
            html! { <UpdateSpectraCollectionForm id={id} /> }
        }
        AppRoute::Teams => {
            html! { <BasicList<NestedTeam> /> }
        }
        AppRoute::TeamsView{id} => {
            html! { <TeamPage id = {id} /> }
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
            html! { <UserPage id = {id} /> }
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
