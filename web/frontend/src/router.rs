//! This file is auto-generated. Do not edit it manually.
//!
//! This file contains the router for the frontend.
use crate::components::forms::automatic_forms::*;
use crate::components::BasicList;
use crate::pages::*;
use web_common::database::*;
use yew::prelude::*;
use yew_router::prelude::*;

/// Trait defining a struct whose page is be visitable by the router.
pub trait Viewable {
    /// Returns the route associated to the page with the overall struct list.
    fn list_route() -> AppRoute;
    /// Returns the route associated with the struct.
    fn view_route(&self) -> AppRoute;
}

impl Viewable for BioOttRank {
    fn list_route() -> AppRoute {
        AppRoute::BioOttRanks {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::BioOttRanksView { id: self.id }
    }
}

impl Viewable for NestedBioOttRank {
    fn list_route() -> AppRoute {
        AppRoute::BioOttRanks {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::BioOttRanksView { id: self.inner.id }
    }
}

impl Viewable for BioOttTaxonItem {
    fn list_route() -> AppRoute {
        AppRoute::BioOttTaxonItems {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::BioOttTaxonItemsView { id: self.id }
    }
}

impl Viewable for NestedBioOttTaxonItem {
    fn list_route() -> AppRoute {
        AppRoute::BioOttTaxonItems {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::BioOttTaxonItemsView { id: self.inner.id }
    }
}

impl Viewable for Country {
    fn list_route() -> AppRoute {
        AppRoute::Countries {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::CountriesView { id: self.id }
    }
}

impl Viewable for Nameplate {
    fn list_route() -> AppRoute {
        AppRoute::Nameplates {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::NameplatesView { id: self.id }
    }
}

impl Viewable for NestedNameplate {
    fn list_route() -> AppRoute {
        AppRoute::Nameplates {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::NameplatesView { id: self.inner.id }
    }
}

impl Viewable for ObservationSubject {
    fn list_route() -> AppRoute {
        AppRoute::ObservationSubjects {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::ObservationSubjectsView { id: self.id }
    }
}

impl Viewable for NestedObservationSubject {
    fn list_route() -> AppRoute {
        AppRoute::ObservationSubjects {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::ObservationSubjectsView { id: self.inner.id }
    }
}

impl Viewable for Observation {
    fn list_route() -> AppRoute {
        AppRoute::Observations {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::ObservationsView { id: self.id }
    }
}

impl Viewable for NestedObservation {
    fn list_route() -> AppRoute {
        AppRoute::Observations {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::ObservationsView { id: self.inner.id }
    }
}

impl Viewable for Organism {
    fn list_route() -> AppRoute {
        AppRoute::Organisms {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::OrganismsView { id: self.id }
    }
}

impl Viewable for NestedOrganism {
    fn list_route() -> AppRoute {
        AppRoute::Organisms {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::OrganismsView { id: self.inner.id }
    }
}

impl Viewable for Organization {
    fn list_route() -> AppRoute {
        AppRoute::Organizations {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::OrganizationsView { id: self.id }
    }
}

impl Viewable for NestedOrganization {
    fn list_route() -> AppRoute {
        AppRoute::Organizations {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::OrganizationsView { id: self.inner.id }
    }
}

impl Viewable for Project {
    fn list_route() -> AppRoute {
        AppRoute::Projects {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::ProjectsView { id: self.id }
    }
}

impl Viewable for NestedProject {
    fn list_route() -> AppRoute {
        AppRoute::Projects {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::ProjectsView { id: self.inner.id }
    }
}

impl Viewable for SampleContainer {
    fn list_route() -> AppRoute {
        AppRoute::SampleContainers {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::SampleContainersView { id: self.id }
    }
}

impl Viewable for NestedSampleContainer {
    fn list_route() -> AppRoute {
        AppRoute::SampleContainers {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::SampleContainersView { id: self.inner.id }
    }
}

impl Viewable for SampleState {
    fn list_route() -> AppRoute {
        AppRoute::SampleStates {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::SampleStatesView { id: self.id }
    }
}

impl Viewable for NestedSampleState {
    fn list_route() -> AppRoute {
        AppRoute::SampleStates {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::SampleStatesView { id: self.inner.id }
    }
}

impl Viewable for Sample {
    fn list_route() -> AppRoute {
        AppRoute::Samples {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::SamplesView { id: self.id }
    }
}

impl Viewable for NestedSample {
    fn list_route() -> AppRoute {
        AppRoute::Samples {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::SamplesView { id: self.inner.id }
    }
}

impl Viewable for Spectra {
    fn list_route() -> AppRoute {
        AppRoute::Spectra {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::SpectraView { id: self.id }
    }
}

impl Viewable for NestedSpectra {
    fn list_route() -> AppRoute {
        AppRoute::Spectra {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::SpectraView { id: self.inner.id }
    }
}

impl Viewable for SpectraCollection {
    fn list_route() -> AppRoute {
        AppRoute::SpectraCollections {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::SpectraCollectionsView { id: self.id }
    }
}

impl Viewable for NestedSpectraCollection {
    fn list_route() -> AppRoute {
        AppRoute::SpectraCollections {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::SpectraCollectionsView { id: self.inner.id }
    }
}

impl Viewable for Team {
    fn list_route() -> AppRoute {
        AppRoute::Teams {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::TeamsView { id: self.id }
    }
}

impl Viewable for NestedTeam {
    fn list_route() -> AppRoute {
        AppRoute::Teams {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::TeamsView { id: self.inner.id }
    }
}

impl Viewable for User {
    fn list_route() -> AppRoute {
        AppRoute::Users {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::UsersView { id: self.id }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum AppRoute {
    #[at("/bio_ott_ranks")]
    BioOttRanks,
    #[at("/bio_ott_ranks/:id")]
    BioOttRanksView { id: i32 },
    #[at("/bio_ott_taxon_items")]
    BioOttTaxonItems,
    #[at("/bio_ott_taxon_items/:id")]
    BioOttTaxonItemsView { id: i32 },
    #[at("/countries")]
    Countries,
    #[at("/countries/:id")]
    CountriesView { id: i32 },
    #[at("/nameplates")]
    Nameplates,
    #[at("/nameplates/:id")]
    NameplatesView { id: i32 },
    #[at("/nameplates/new")]
    NameplatesNew,
    #[at("/nameplates/new/project/:project_id")]
    NameplatesNewWithProject { project_id: i32 },
    #[at("/nameplates/:id/update")]
    NameplatesUpdate { id: i32 },
    #[at("/observation_subjects")]
    ObservationSubjects,
    #[at("/observation_subjects/:id")]
    ObservationSubjectsView { id: i32 },
    #[at("/observations")]
    Observations,
    #[at("/observations/:id")]
    ObservationsView { id: uuid::Uuid },
    #[at("/observations/new")]
    ObservationsNew,
    #[at("/observations/new/parent_observation/:parent_observation_id")]
    ObservationsNewWithParentObservation { parent_observation_id: uuid::Uuid },
    #[at("/observations/new/project/:project_id")]
    ObservationsNewWithProject { project_id: i32 },
    #[at("/observations/new/organism/:organism_id")]
    ObservationsNewWithOrganism { organism_id: uuid::Uuid },
    #[at("/observations/new/sample/:sample_id")]
    ObservationsNewWithSample { sample_id: uuid::Uuid },
    #[at("/observations/:id/update")]
    ObservationsUpdate { id: uuid::Uuid },
    #[at("/organisms")]
    Organisms,
    #[at("/organisms/:id")]
    OrganismsView { id: uuid::Uuid },
    #[at("/organisms/new")]
    OrganismsNew,
    #[at("/organisms/new/host_organism/:host_organism_id")]
    OrganismsNewWithHostOrganism { host_organism_id: uuid::Uuid },
    #[at("/organisms/new/sample/:sample_id")]
    OrganismsNewWithSample { sample_id: uuid::Uuid },
    #[at("/organisms/new/nameplate/:nameplate_id")]
    OrganismsNewWithNameplate { nameplate_id: i32 },
    #[at("/organisms/new/project/:project_id")]
    OrganismsNewWithProject { project_id: i32 },
    #[at("/organisms/:id/update")]
    OrganismsUpdate { id: uuid::Uuid },
    #[at("/organizations")]
    Organizations,
    #[at("/organizations/:id")]
    OrganizationsView { id: i32 },
    #[at("/projects")]
    Projects,
    #[at("/projects/:id")]
    ProjectsView { id: i32 },
    #[at("/projects/new")]
    ProjectsNew,
    #[at("/projects/new/parent_project/:parent_project_id")]
    ProjectsNewWithParentProject { parent_project_id: i32 },
    #[at("/projects/:id/update")]
    ProjectsUpdate { id: i32 },
    #[at("/sample_containers")]
    SampleContainers,
    #[at("/sample_containers/:id")]
    SampleContainersView { id: i32 },
    #[at("/sample_containers/new")]
    SampleContainersNew,
    #[at("/sample_containers/new/project/:project_id")]
    SampleContainersNewWithProject { project_id: i32 },
    #[at("/sample_containers/:id/update")]
    SampleContainersUpdate { id: i32 },
    #[at("/sample_states")]
    SampleStates,
    #[at("/sample_states/:id")]
    SampleStatesView { id: i32 },
    #[at("/samples")]
    Samples,
    #[at("/samples/:id")]
    SamplesView { id: uuid::Uuid },
    #[at("/samples/new")]
    SamplesNew,
    #[at("/samples/new/container/:container_id")]
    SamplesNewWithContainer { container_id: i32 },
    #[at("/samples/new/project/:project_id")]
    SamplesNewWithProject { project_id: i32 },
    #[at("/samples/new/sampled_by/:sampled_by")]
    SamplesNewWithSampledBy { sampled_by: i32 },
    #[at("/samples/:id/update")]
    SamplesUpdate { id: uuid::Uuid },
    #[at("/spectra")]
    Spectra,
    #[at("/spectra/:id")]
    SpectraView { id: i32 },
    #[at("/spectra/new")]
    SpectraNew,
    #[at("/spectra/new/spectra_collection/:spectra_collection_id")]
    SpectraNewWithSpectraCollection { spectra_collection_id: i32 },
    #[at("/spectra/:id/update")]
    SpectraUpdate { id: i32 },
    #[at("/spectra_collections")]
    SpectraCollections,
    #[at("/spectra_collections/:id")]
    SpectraCollectionsView { id: i32 },
    #[at("/spectra_collections/new")]
    SpectraCollectionsNew,
    #[at("/spectra_collections/new/sample/:sample_id")]
    SpectraCollectionsNewWithSample { sample_id: uuid::Uuid },
    #[at("/spectra_collections/:id/update")]
    SpectraCollectionsUpdate { id: i32 },
    #[at("/teams")]
    Teams,
    #[at("/teams/:id")]
    TeamsView { id: i32 },
    #[at("/teams/new")]
    TeamsNew,
    #[at("/teams/new/parent_team/:parent_team_id")]
    TeamsNewWithParentTeam { parent_team_id: i32 },
    #[at("/teams/:id/update")]
    TeamsUpdate { id: i32 },
    #[at("/users")]
    Users,
    #[at("/users/:id")]
    UsersView { id: i32 },
    #[at("/users/:id/update")]
    UsersUpdate { id: i32 },
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
pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::BioOttRanks => {
            html! { <BasicList<NestedBioOttRank> /> }
        }
        AppRoute::BioOttRanksView { id } => {
            html! { <BioOttRankPage id = {id} /> }
        }
        AppRoute::BioOttTaxonItems => {
            html! { <BasicList<NestedBioOttTaxonItem> /> }
        }
        AppRoute::BioOttTaxonItemsView { id } => {
            html! { <BioOttTaxonItemPage id = {id} /> }
        }
        AppRoute::Countries => {
            html! { <BasicList<Country> /> }
        }
        AppRoute::CountriesView { id } => {
            html! { <CountryPage id = {id} /> }
        }
        AppRoute::Nameplates => {
            html! { <BasicList<NestedNameplate> /> }
        }
        AppRoute::NameplatesView { id } => {
            html! { <NameplatePage id = {id} /> }
        }
        AppRoute::NameplatesNew => {
            html! { <CreateNameplateForm /> }
        }
        AppRoute::NameplatesNewWithProject { project_id } => {
            html! { <CreateNameplateForm project_id={project_id} /> }
        }
        AppRoute::NameplatesUpdate { id } => {
            html! { <UpdateNameplateForm id={id} /> }
        }
        AppRoute::ObservationSubjects => {
            html! { <BasicList<NestedObservationSubject> /> }
        }
        AppRoute::ObservationSubjectsView { id } => {
            html! { <ObservationSubjectPage id = {id} /> }
        }
        AppRoute::Observations => {
            html! { <BasicList<NestedObservation> /> }
        }
        AppRoute::ObservationsView { id } => {
            html! { <ObservationPage id = {id} /> }
        }
        AppRoute::ObservationsNew => {
            html! { <CreateObservationForm /> }
        }
        AppRoute::ObservationsNewWithParentObservation {
            parent_observation_id,
        } => {
            html! { <CreateObservationForm parent_observation_id={parent_observation_id} /> }
        }
        AppRoute::ObservationsNewWithProject { project_id } => {
            html! { <CreateObservationForm project_id={project_id} /> }
        }
        AppRoute::ObservationsNewWithOrganism { organism_id } => {
            html! { <CreateObservationForm organism_id={organism_id} /> }
        }
        AppRoute::ObservationsNewWithSample { sample_id } => {
            html! { <CreateObservationForm sample_id={sample_id} /> }
        }
        AppRoute::ObservationsUpdate { id } => {
            html! { <UpdateObservationForm id={id} /> }
        }
        AppRoute::Organisms => {
            html! { <BasicList<NestedOrganism> /> }
        }
        AppRoute::OrganismsView { id } => {
            html! { <OrganismPage id = {id} /> }
        }
        AppRoute::OrganismsNew => {
            html! { <CreateOrganismForm /> }
        }
        AppRoute::OrganismsNewWithHostOrganism { host_organism_id } => {
            html! { <CreateOrganismForm host_organism_id={host_organism_id} /> }
        }
        AppRoute::OrganismsNewWithSample { sample_id } => {
            html! { <CreateOrganismForm sample_id={sample_id} /> }
        }
        AppRoute::OrganismsNewWithNameplate { nameplate_id } => {
            html! { <CreateOrganismForm nameplate_id={nameplate_id} /> }
        }
        AppRoute::OrganismsNewWithProject { project_id } => {
            html! { <CreateOrganismForm project_id={project_id} /> }
        }
        AppRoute::OrganismsUpdate { id } => {
            html! { <UpdateOrganismForm id={id} /> }
        }
        AppRoute::Organizations => {
            html! { <BasicList<NestedOrganization> /> }
        }
        AppRoute::OrganizationsView { id } => {
            html! { <OrganizationPage id = {id} /> }
        }
        AppRoute::Projects => {
            html! { <BasicList<NestedProject> /> }
        }
        AppRoute::ProjectsView { id } => {
            html! { <ProjectPage id = {id} /> }
        }
        AppRoute::ProjectsNew => {
            html! { <CreateProjectForm /> }
        }
        AppRoute::ProjectsNewWithParentProject { parent_project_id } => {
            html! { <CreateProjectForm parent_project_id={parent_project_id} /> }
        }
        AppRoute::ProjectsUpdate { id } => {
            html! { <UpdateProjectForm id={id} /> }
        }
        AppRoute::SampleContainers => {
            html! { <BasicList<NestedSampleContainer> /> }
        }
        AppRoute::SampleContainersView { id } => {
            html! { <SampleContainerPage id = {id} /> }
        }
        AppRoute::SampleContainersNew => {
            html! { <CreateSampleContainerForm /> }
        }
        AppRoute::SampleContainersNewWithProject { project_id } => {
            html! { <CreateSampleContainerForm project_id={project_id} /> }
        }
        AppRoute::SampleContainersUpdate { id } => {
            html! { <UpdateSampleContainerForm id={id} /> }
        }
        AppRoute::SampleStates => {
            html! { <BasicList<NestedSampleState> /> }
        }
        AppRoute::SampleStatesView { id } => {
            html! { <SampleStatePage id = {id} /> }
        }
        AppRoute::Samples => {
            html! { <BasicList<NestedSample> /> }
        }
        AppRoute::SamplesView { id } => {
            html! { <SamplePage id = {id} /> }
        }
        AppRoute::SamplesNew => {
            html! { <CreateSampleForm /> }
        }
        AppRoute::SamplesNewWithContainer { container_id } => {
            html! { <CreateSampleForm container_id={container_id} /> }
        }
        AppRoute::SamplesNewWithProject { project_id } => {
            html! { <CreateSampleForm project_id={project_id} /> }
        }
        AppRoute::SamplesNewWithSampledBy { sampled_by } => {
            html! { <CreateSampleForm sampled_by={sampled_by} /> }
        }
        AppRoute::SamplesUpdate { id } => {
            html! { <UpdateSampleForm id={id} /> }
        }
        AppRoute::Spectra => {
            html! { <BasicList<NestedSpectra> /> }
        }
        AppRoute::SpectraView { id } => {
            html! { <SpectraPage id = {id} /> }
        }
        AppRoute::SpectraNew => {
            html! { <CreateSpectraForm /> }
        }
        AppRoute::SpectraNewWithSpectraCollection {
            spectra_collection_id,
        } => {
            html! { <CreateSpectraForm spectra_collection_id={spectra_collection_id} /> }
        }
        AppRoute::SpectraUpdate { id } => {
            html! { <UpdateSpectraForm id={id} /> }
        }
        AppRoute::SpectraCollections => {
            html! { <BasicList<NestedSpectraCollection> /> }
        }
        AppRoute::SpectraCollectionsView { id } => {
            html! { <SpectraCollectionPage id = {id} /> }
        }
        AppRoute::SpectraCollectionsNew => {
            html! { <CreateSpectraCollectionForm /> }
        }
        AppRoute::SpectraCollectionsNewWithSample { sample_id } => {
            html! { <CreateSpectraCollectionForm sample_id={sample_id} /> }
        }
        AppRoute::SpectraCollectionsUpdate { id } => {
            html! { <UpdateSpectraCollectionForm id={id} /> }
        }
        AppRoute::Teams => {
            html! { <BasicList<NestedTeam> /> }
        }
        AppRoute::TeamsView { id } => {
            html! { <TeamPage id = {id} /> }
        }
        AppRoute::TeamsNew => {
            html! { <CreateTeamForm /> }
        }
        AppRoute::TeamsNewWithParentTeam { parent_team_id } => {
            html! { <CreateTeamForm parent_team_id={parent_team_id} /> }
        }
        AppRoute::TeamsUpdate { id } => {
            html! { <UpdateTeamForm id={id} /> }
        }
        AppRoute::Users => {
            html! { <BasicList<User> /> }
        }
        AppRoute::UsersView { id } => {
            html! { <UserPage id = {id} /> }
        }
        AppRoute::UsersUpdate { id } => {
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
