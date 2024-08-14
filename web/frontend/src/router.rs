//! This file is auto-generated. Do not edit it manually.
//!
//! This file contains the router for the frontend.
use crate::components::forms::automatic_forms::*;
use crate::components::BasicList;
use crate::pages::*;
use web_common::database::*;
use yew::prelude::*;
use yew_router::prelude::*;

/// The trait defining whether a struct has a role request struct associated with it.
pub(crate) trait RoleRequestable {
    /// The role request struct associated with the struct.
    type RoleRequest: serde::Serialize + Tabular;
    /// Returns the role request struct associated with the struct.
    fn role_request(&self, user_id: i32, role: i32) -> Self::RoleRequest;
    /// Returns the role request operation associated with the struct.
    fn role_request_operation(&self, user_id: i32, role: i32) -> Operation {
        Operation::Insert(
            Self::RoleRequest::TABLE.to_string(),
            bincode::serialize(&self.role_request(user_id, role)).unwrap(),
        )
    }
    /// Returns the route associated with the struct role request page.
    fn edit_role_request(&self, user_id: i32) -> Operation {
        self.role_request_operation(user_id, 2)
    }
    /// Returns the route associated with the struct role request page.
    fn admin_role_request(&self, user_id: i32) -> Operation {
        self.role_request_operation(user_id, 1)
    }
}

/// Trait defining a struct whose page is be visitable by the router.
pub(crate) trait Viewable {
    /// Returns the route associated to the page with the overall struct list.
    fn list_route() -> AppRoute;
    /// Returns the route associated with the struct.
    fn view_route(&self) -> AppRoute;
}

/// Trait defining a struct that can be created, and therefore has a new page.
pub(crate) trait Insertable: Filtrable {
    /// Returns the route associated with the struct new page.
    ///
    /// # Arguments
    /// * `filter` - The optional filter to use to populate part of the form.
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute;
}

/// Trait defining a struct that can be updated, and therefore has an update page.
pub(crate) trait Updatable {
    /// Returns the route associated with the struct update page.
    fn update_route(&self) -> AppRoute;
}

impl Viewable for Country {
    fn list_route() -> AppRoute {
        AppRoute::Countries {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::CountriesView { id: self.id }
    }
}

impl Viewable for DerivedSample {
    fn list_route() -> AppRoute {
        AppRoute::DerivedSamples {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::DerivedSamplesView {
            parent_sample_id: self.parent_sample_id,
            child_sample_id: self.child_sample_id,
        }
    }
}

impl Insertable for DerivedSample {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(parent_sample_id) = filter.parent_sample_id {
                return AppRoute::DerivedSamplesNewWithParentSample { parent_sample_id };
            }
            if let Some(child_sample_id) = filter.child_sample_id {
                return AppRoute::DerivedSamplesNewWithChildSample { child_sample_id };
            }
        }
        AppRoute::DerivedSamplesNew
    }
}

impl Updatable for DerivedSample {
    fn update_route(&self) -> AppRoute {
        AppRoute::DerivedSamplesUpdate {
            parent_sample_id: self.parent_sample_id,
            child_sample_id: self.child_sample_id,
        }
    }
}

impl Viewable for NestedDerivedSample {
    fn list_route() -> AppRoute {
        AppRoute::DerivedSamples {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::DerivedSamplesView {
            parent_sample_id: self.inner.parent_sample_id,
            child_sample_id: self.inner.child_sample_id,
        }
    }
}

impl Insertable for NestedDerivedSample {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(parent_sample_id) = filter.parent_sample_id {
                return AppRoute::DerivedSamplesNewWithParentSample { parent_sample_id };
            }
            if let Some(child_sample_id) = filter.child_sample_id {
                return AppRoute::DerivedSamplesNewWithChildSample { child_sample_id };
            }
        }
        AppRoute::DerivedSamplesNew
    }
}

impl Updatable for NestedDerivedSample {
    fn update_route(&self) -> AppRoute {
        AppRoute::DerivedSamplesUpdate {
            parent_sample_id: self.inner.parent_sample_id,
            child_sample_id: self.inner.child_sample_id,
        }
    }
}

impl RoleRequestable for NestedNameplate {
    type RoleRequest = <NestedProject as RoleRequestable>::RoleRequest;
    fn role_request(&self, user_id: i32, role: i32) -> Self::RoleRequest {
        <NestedProject as RoleRequestable>::role_request(self.project.as_ref(), user_id, role)
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

impl Insertable for Nameplate {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(project_id) = filter.project_id {
                return AppRoute::NameplatesNewWithProject { project_id };
            }
        }
        AppRoute::NameplatesNew
    }
}

impl Updatable for Nameplate {
    fn update_route(&self) -> AppRoute {
        AppRoute::NameplatesUpdate { id: self.id }
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

impl Insertable for NestedNameplate {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(project_id) = filter.project_id {
                return AppRoute::NameplatesNewWithProject { project_id };
            }
        }
        AppRoute::NameplatesNew
    }
}

impl Updatable for NestedNameplate {
    fn update_route(&self) -> AppRoute {
        AppRoute::NameplatesUpdate { id: self.inner.id }
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

impl RoleRequestable for NestedObservation {
    type RoleRequest = <NestedProject as RoleRequestable>::RoleRequest;
    fn role_request(&self, user_id: i32, role: i32) -> Self::RoleRequest {
        <NestedProject as RoleRequestable>::role_request(self.project.as_ref(), user_id, role)
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

impl Insertable for Observation {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(parent_observation_id) = filter.parent_observation_id {
                return AppRoute::ObservationsNewWithParentObservation {
                    parent_observation_id,
                };
            }
            if let Some(project_id) = filter.project_id {
                return AppRoute::ObservationsNewWithProject { project_id };
            }
            if let Some(organism_id) = filter.organism_id {
                return AppRoute::ObservationsNewWithOrganism { organism_id };
            }
            if let Some(sample_id) = filter.sample_id {
                return AppRoute::ObservationsNewWithSample { sample_id };
            }
        }
        AppRoute::ObservationsNew
    }
}

impl Updatable for Observation {
    fn update_route(&self) -> AppRoute {
        AppRoute::ObservationsUpdate { id: self.id }
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

impl Insertable for NestedObservation {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(parent_observation_id) = filter.parent_observation_id {
                return AppRoute::ObservationsNewWithParentObservation {
                    parent_observation_id,
                };
            }
            if let Some(project_id) = filter.project_id {
                return AppRoute::ObservationsNewWithProject { project_id };
            }
            if let Some(organism_id) = filter.organism_id {
                return AppRoute::ObservationsNewWithOrganism { organism_id };
            }
            if let Some(sample_id) = filter.sample_id {
                return AppRoute::ObservationsNewWithSample { sample_id };
            }
        }
        AppRoute::ObservationsNew
    }
}

impl Updatable for NestedObservation {
    fn update_route(&self) -> AppRoute {
        AppRoute::ObservationsUpdate { id: self.inner.id }
    }
}

impl Viewable for OrganismTaxon {
    fn list_route() -> AppRoute {
        AppRoute::OrganismTaxa {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::OrganismTaxaView {
            organism_id: self.organism_id,
            taxon_id: self.taxon_id,
        }
    }
}

impl Insertable for OrganismTaxon {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(organism_id) = filter.organism_id {
                return AppRoute::OrganismTaxaNewWithOrganism { organism_id };
            }
        }
        AppRoute::OrganismTaxaNew
    }
}

impl Viewable for NestedOrganismTaxon {
    fn list_route() -> AppRoute {
        AppRoute::OrganismTaxa {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::OrganismTaxaView {
            organism_id: self.inner.organism_id,
            taxon_id: self.inner.taxon_id,
        }
    }
}

impl Insertable for NestedOrganismTaxon {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(organism_id) = filter.organism_id {
                return AppRoute::OrganismTaxaNewWithOrganism { organism_id };
            }
        }
        AppRoute::OrganismTaxaNew
    }
}

impl RoleRequestable for NestedOrganism {
    type RoleRequest = <NestedProject as RoleRequestable>::RoleRequest;
    fn role_request(&self, user_id: i32, role: i32) -> Self::RoleRequest {
        <NestedProject as RoleRequestable>::role_request(
            self.nameplate.project.as_ref(),
            user_id,
            role,
        )
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

impl Insertable for Organism {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(host_organism_id) = filter.host_organism_id {
                return AppRoute::OrganismsNewWithHostOrganism { host_organism_id };
            }
            if let Some(sample_id) = filter.sample_id {
                return AppRoute::OrganismsNewWithSample { sample_id };
            }
            if let Some(nameplate_id) = filter.nameplate_id {
                return AppRoute::OrganismsNewWithNameplate { nameplate_id };
            }
            if let Some(project_id) = filter.project_id {
                return AppRoute::OrganismsNewWithProject { project_id };
            }
        }
        AppRoute::OrganismsNew
    }
}

impl Updatable for Organism {
    fn update_route(&self) -> AppRoute {
        AppRoute::OrganismsUpdate { id: self.id }
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

impl Insertable for NestedOrganism {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(host_organism_id) = filter.host_organism_id {
                return AppRoute::OrganismsNewWithHostOrganism { host_organism_id };
            }
            if let Some(sample_id) = filter.sample_id {
                return AppRoute::OrganismsNewWithSample { sample_id };
            }
            if let Some(nameplate_id) = filter.nameplate_id {
                return AppRoute::OrganismsNewWithNameplate { nameplate_id };
            }
            if let Some(project_id) = filter.project_id {
                return AppRoute::OrganismsNewWithProject { project_id };
            }
        }
        AppRoute::OrganismsNew
    }
}

impl Updatable for NestedOrganism {
    fn update_route(&self) -> AppRoute {
        AppRoute::OrganismsUpdate { id: self.inner.id }
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

impl RoleRequestable for Project {
    type RoleRequest = NewProjectsUsersRoleRequest;
    fn role_request(&self, user_id: i32, role: i32) -> Self::RoleRequest {
        NewProjectsUsersRoleRequest {
            table_id: self.id,
            user_id,
            role_id: role,
        }
    }
}

impl RoleRequestable for NestedProject {
    type RoleRequest = <Project as RoleRequestable>::RoleRequest;
    fn role_request(&self, user_id: i32, role: i32) -> Self::RoleRequest {
        <Project as RoleRequestable>::role_request(self.inner.as_ref(), user_id, role)
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

impl Insertable for Project {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(parent_project_id) = filter.parent_project_id {
                return AppRoute::ProjectsNewWithParentProject { parent_project_id };
            }
        }
        AppRoute::ProjectsNew
    }
}

impl Updatable for Project {
    fn update_route(&self) -> AppRoute {
        AppRoute::ProjectsUpdate { id: self.id }
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

impl Insertable for NestedProject {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(parent_project_id) = filter.parent_project_id {
                return AppRoute::ProjectsNewWithParentProject { parent_project_id };
            }
        }
        AppRoute::ProjectsNew
    }
}

impl Updatable for NestedProject {
    fn update_route(&self) -> AppRoute {
        AppRoute::ProjectsUpdate { id: self.inner.id }
    }
}

impl Viewable for ProjectsTeamsRoleInvitation {
    fn list_route() -> AppRoute {
        AppRoute::ProjectsTeamsRoleInvitations {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::ProjectsTeamsRoleInvitationsView {
            table_id: self.table_id,
            team_id: self.team_id,
        }
    }
}

impl Insertable for ProjectsTeamsRoleInvitation {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::ProjectsTeamsRoleInvitationsNewWithTable { table_id };
            }
            if let Some(team_id) = filter.team_id {
                return AppRoute::ProjectsTeamsRoleInvitationsNewWithTeam { team_id };
            }
        }
        AppRoute::ProjectsTeamsRoleInvitationsNew
    }
}

impl Viewable for NestedProjectsTeamsRoleInvitation {
    fn list_route() -> AppRoute {
        AppRoute::ProjectsTeamsRoleInvitations {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::ProjectsTeamsRoleInvitationsView {
            table_id: self.inner.table_id,
            team_id: self.inner.team_id,
        }
    }
}

impl Insertable for NestedProjectsTeamsRoleInvitation {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::ProjectsTeamsRoleInvitationsNewWithTable { table_id };
            }
            if let Some(team_id) = filter.team_id {
                return AppRoute::ProjectsTeamsRoleInvitationsNewWithTeam { team_id };
            }
        }
        AppRoute::ProjectsTeamsRoleInvitationsNew
    }
}

impl Viewable for ProjectsTeamsRoleRequest {
    fn list_route() -> AppRoute {
        AppRoute::ProjectsTeamsRoleRequests {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::ProjectsTeamsRoleRequestsView {
            table_id: self.table_id,
            team_id: self.team_id,
        }
    }
}

impl Viewable for NestedProjectsTeamsRoleRequest {
    fn list_route() -> AppRoute {
        AppRoute::ProjectsTeamsRoleRequests {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::ProjectsTeamsRoleRequestsView {
            table_id: self.inner.table_id,
            team_id: self.inner.team_id,
        }
    }
}

impl Viewable for ProjectsTeamsRole {
    fn list_route() -> AppRoute {
        AppRoute::ProjectsTeamsRoles {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::ProjectsTeamsRolesView {
            table_id: self.table_id,
            team_id: self.team_id,
        }
    }
}

impl Insertable for ProjectsTeamsRole {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::ProjectsTeamsRolesNewWithTable { table_id };
            }
            if let Some(team_id) = filter.team_id {
                return AppRoute::ProjectsTeamsRolesNewWithTeam { team_id };
            }
        }
        AppRoute::ProjectsTeamsRolesNew
    }
}

impl Viewable for NestedProjectsTeamsRole {
    fn list_route() -> AppRoute {
        AppRoute::ProjectsTeamsRoles {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::ProjectsTeamsRolesView {
            table_id: self.inner.table_id,
            team_id: self.inner.team_id,
        }
    }
}

impl Insertable for NestedProjectsTeamsRole {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::ProjectsTeamsRolesNewWithTable { table_id };
            }
            if let Some(team_id) = filter.team_id {
                return AppRoute::ProjectsTeamsRolesNewWithTeam { team_id };
            }
        }
        AppRoute::ProjectsTeamsRolesNew
    }
}

impl Viewable for ProjectsUsersRoleInvitation {
    fn list_route() -> AppRoute {
        AppRoute::ProjectsUsersRoleInvitations {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::ProjectsUsersRoleInvitationsView {
            table_id: self.table_id,
            user_id: self.user_id,
        }
    }
}

impl Insertable for ProjectsUsersRoleInvitation {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::ProjectsUsersRoleInvitationsNewWithTable { table_id };
            }
            if let Some(user_id) = filter.user_id {
                return AppRoute::ProjectsUsersRoleInvitationsNewWithUser { user_id };
            }
        }
        AppRoute::ProjectsUsersRoleInvitationsNew
    }
}

impl Viewable for NestedProjectsUsersRoleInvitation {
    fn list_route() -> AppRoute {
        AppRoute::ProjectsUsersRoleInvitations {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::ProjectsUsersRoleInvitationsView {
            table_id: self.inner.table_id,
            user_id: self.inner.user_id,
        }
    }
}

impl Insertable for NestedProjectsUsersRoleInvitation {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::ProjectsUsersRoleInvitationsNewWithTable { table_id };
            }
            if let Some(user_id) = filter.user_id {
                return AppRoute::ProjectsUsersRoleInvitationsNewWithUser { user_id };
            }
        }
        AppRoute::ProjectsUsersRoleInvitationsNew
    }
}

impl Viewable for ProjectsUsersRoleRequest {
    fn list_route() -> AppRoute {
        AppRoute::ProjectsUsersRoleRequests {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::ProjectsUsersRoleRequestsView {
            table_id: self.table_id,
            user_id: self.user_id,
        }
    }
}

impl Viewable for NestedProjectsUsersRoleRequest {
    fn list_route() -> AppRoute {
        AppRoute::ProjectsUsersRoleRequests {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::ProjectsUsersRoleRequestsView {
            table_id: self.inner.table_id,
            user_id: self.inner.user_id,
        }
    }
}

impl Viewable for ProjectsUsersRole {
    fn list_route() -> AppRoute {
        AppRoute::ProjectsUsersRoles {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::ProjectsUsersRolesView {
            table_id: self.table_id,
            user_id: self.user_id,
        }
    }
}

impl Insertable for ProjectsUsersRole {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::ProjectsUsersRolesNewWithTable { table_id };
            }
            if let Some(user_id) = filter.user_id {
                return AppRoute::ProjectsUsersRolesNewWithUser { user_id };
            }
        }
        AppRoute::ProjectsUsersRolesNew
    }
}

impl Viewable for NestedProjectsUsersRole {
    fn list_route() -> AppRoute {
        AppRoute::ProjectsUsersRoles {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::ProjectsUsersRolesView {
            table_id: self.inner.table_id,
            user_id: self.inner.user_id,
        }
    }
}

impl Insertable for NestedProjectsUsersRole {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::ProjectsUsersRolesNewWithTable { table_id };
            }
            if let Some(user_id) = filter.user_id {
                return AppRoute::ProjectsUsersRolesNewWithUser { user_id };
            }
        }
        AppRoute::ProjectsUsersRolesNew
    }
}

impl Viewable for Rank {
    fn list_route() -> AppRoute {
        AppRoute::Ranks {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::RanksView { id: self.id }
    }
}

impl Viewable for NestedRank {
    fn list_route() -> AppRoute {
        AppRoute::Ranks {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::RanksView { id: self.inner.id }
    }
}

impl RoleRequestable for NestedSampleContainer {
    type RoleRequest = <NestedProject as RoleRequestable>::RoleRequest;
    fn role_request(&self, user_id: i32, role: i32) -> Self::RoleRequest {
        <NestedProject as RoleRequestable>::role_request(self.project.as_ref(), user_id, role)
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

impl Insertable for SampleContainer {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(project_id) = filter.project_id {
                return AppRoute::SampleContainersNewWithProject { project_id };
            }
        }
        AppRoute::SampleContainersNew
    }
}

impl Updatable for SampleContainer {
    fn update_route(&self) -> AppRoute {
        AppRoute::SampleContainersUpdate { id: self.id }
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

impl Insertable for NestedSampleContainer {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(project_id) = filter.project_id {
                return AppRoute::SampleContainersNewWithProject { project_id };
            }
        }
        AppRoute::SampleContainersNew
    }
}

impl Updatable for NestedSampleContainer {
    fn update_route(&self) -> AppRoute {
        AppRoute::SampleContainersUpdate { id: self.inner.id }
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

impl Viewable for SampleTaxon {
    fn list_route() -> AppRoute {
        AppRoute::SampleTaxa {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::SampleTaxaView {
            sample_id: self.sample_id,
            taxon_id: self.taxon_id,
        }
    }
}

impl Insertable for SampleTaxon {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(sample_id) = filter.sample_id {
                return AppRoute::SampleTaxaNewWithSample { sample_id };
            }
        }
        AppRoute::SampleTaxaNew
    }
}

impl Viewable for NestedSampleTaxon {
    fn list_route() -> AppRoute {
        AppRoute::SampleTaxa {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::SampleTaxaView {
            sample_id: self.inner.sample_id,
            taxon_id: self.inner.taxon_id,
        }
    }
}

impl Insertable for NestedSampleTaxon {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(sample_id) = filter.sample_id {
                return AppRoute::SampleTaxaNewWithSample { sample_id };
            }
        }
        AppRoute::SampleTaxaNew
    }
}

impl RoleRequestable for NestedSample {
    type RoleRequest = <NestedProject as RoleRequestable>::RoleRequest;
    fn role_request(&self, user_id: i32, role: i32) -> Self::RoleRequest {
        <NestedProject as RoleRequestable>::role_request(
            self.container.project.as_ref(),
            user_id,
            role,
        )
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

impl Insertable for Sample {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(container_id) = filter.container_id {
                return AppRoute::SamplesNewWithContainer { container_id };
            }
            if let Some(project_id) = filter.project_id {
                return AppRoute::SamplesNewWithProject { project_id };
            }
            if let Some(sampled_by) = filter.sampled_by {
                return AppRoute::SamplesNewWithSampledBy { sampled_by };
            }
        }
        AppRoute::SamplesNew
    }
}

impl Updatable for Sample {
    fn update_route(&self) -> AppRoute {
        AppRoute::SamplesUpdate { id: self.id }
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

impl Insertable for NestedSample {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(container_id) = filter.container_id {
                return AppRoute::SamplesNewWithContainer { container_id };
            }
            if let Some(project_id) = filter.project_id {
                return AppRoute::SamplesNewWithProject { project_id };
            }
            if let Some(sampled_by) = filter.sampled_by {
                return AppRoute::SamplesNewWithSampledBy { sampled_by };
            }
        }
        AppRoute::SamplesNew
    }
}

impl Updatable for NestedSample {
    fn update_route(&self) -> AppRoute {
        AppRoute::SamplesUpdate { id: self.inner.id }
    }
}

impl RoleRequestable for NestedSpectraCollection {
    type RoleRequest = <NestedProject as RoleRequestable>::RoleRequest;
    fn role_request(&self, user_id: i32, role: i32) -> Self::RoleRequest {
        <NestedProject as RoleRequestable>::role_request(
            self.sample.container.project.as_ref(),
            user_id,
            role,
        )
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

impl Insertable for SpectraCollection {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(sample_id) = filter.sample_id {
                return AppRoute::SpectraCollectionsNewWithSample { sample_id };
            }
        }
        AppRoute::SpectraCollectionsNew
    }
}

impl Updatable for SpectraCollection {
    fn update_route(&self) -> AppRoute {
        AppRoute::SpectraCollectionsUpdate { id: self.id }
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

impl Insertable for NestedSpectraCollection {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(sample_id) = filter.sample_id {
                return AppRoute::SpectraCollectionsNewWithSample { sample_id };
            }
        }
        AppRoute::SpectraCollectionsNew
    }
}

impl Updatable for NestedSpectraCollection {
    fn update_route(&self) -> AppRoute {
        AppRoute::SpectraCollectionsUpdate { id: self.inner.id }
    }
}

impl Viewable for Taxon {
    fn list_route() -> AppRoute {
        AppRoute::Taxa {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::TaxaView { id: self.id }
    }
}

impl Viewable for NestedTaxon {
    fn list_route() -> AppRoute {
        AppRoute::Taxa {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::TaxaView { id: self.inner.id }
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

impl Insertable for Team {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(parent_team_id) = filter.parent_team_id {
                return AppRoute::TeamsNewWithParentTeam { parent_team_id };
            }
        }
        AppRoute::TeamsNew
    }
}

impl Updatable for Team {
    fn update_route(&self) -> AppRoute {
        AppRoute::TeamsUpdate { id: self.id }
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

impl Insertable for NestedTeam {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(parent_team_id) = filter.parent_team_id {
                return AppRoute::TeamsNewWithParentTeam { parent_team_id };
            }
        }
        AppRoute::TeamsNew
    }
}

impl Updatable for NestedTeam {
    fn update_route(&self) -> AppRoute {
        AppRoute::TeamsUpdate { id: self.inner.id }
    }
}

impl Viewable for TeamsTeamsRoleInvitation {
    fn list_route() -> AppRoute {
        AppRoute::TeamsTeamsRoleInvitations {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::TeamsTeamsRoleInvitationsView {
            table_id: self.table_id,
            team_id: self.team_id,
        }
    }
}

impl Insertable for TeamsTeamsRoleInvitation {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::TeamsTeamsRoleInvitationsNewWithTable { table_id };
            }
            if let Some(team_id) = filter.team_id {
                return AppRoute::TeamsTeamsRoleInvitationsNewWithTeam { team_id };
            }
        }
        AppRoute::TeamsTeamsRoleInvitationsNew
    }
}

impl Viewable for NestedTeamsTeamsRoleInvitation {
    fn list_route() -> AppRoute {
        AppRoute::TeamsTeamsRoleInvitations {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::TeamsTeamsRoleInvitationsView {
            table_id: self.inner.table_id,
            team_id: self.inner.team_id,
        }
    }
}

impl Insertable for NestedTeamsTeamsRoleInvitation {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::TeamsTeamsRoleInvitationsNewWithTable { table_id };
            }
            if let Some(team_id) = filter.team_id {
                return AppRoute::TeamsTeamsRoleInvitationsNewWithTeam { team_id };
            }
        }
        AppRoute::TeamsTeamsRoleInvitationsNew
    }
}

impl Viewable for TeamsUsersRoleInvitation {
    fn list_route() -> AppRoute {
        AppRoute::TeamsUsersRoleInvitations {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::TeamsUsersRoleInvitationsView {
            table_id: self.table_id,
            user_id: self.user_id,
        }
    }
}

impl Insertable for TeamsUsersRoleInvitation {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::TeamsUsersRoleInvitationsNewWithTable { table_id };
            }
            if let Some(user_id) = filter.user_id {
                return AppRoute::TeamsUsersRoleInvitationsNewWithUser { user_id };
            }
        }
        AppRoute::TeamsUsersRoleInvitationsNew
    }
}

impl Viewable for NestedTeamsUsersRoleInvitation {
    fn list_route() -> AppRoute {
        AppRoute::TeamsUsersRoleInvitations {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::TeamsUsersRoleInvitationsView {
            table_id: self.inner.table_id,
            user_id: self.inner.user_id,
        }
    }
}

impl Insertable for NestedTeamsUsersRoleInvitation {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::TeamsUsersRoleInvitationsNewWithTable { table_id };
            }
            if let Some(user_id) = filter.user_id {
                return AppRoute::TeamsUsersRoleInvitationsNewWithUser { user_id };
            }
        }
        AppRoute::TeamsUsersRoleInvitationsNew
    }
}

impl Viewable for TeamsUsersRoleRequest {
    fn list_route() -> AppRoute {
        AppRoute::TeamsUsersRoleRequests {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::TeamsUsersRoleRequestsView {
            table_id: self.table_id,
            user_id: self.user_id,
        }
    }
}

impl Viewable for NestedTeamsUsersRoleRequest {
    fn list_route() -> AppRoute {
        AppRoute::TeamsUsersRoleRequests {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::TeamsUsersRoleRequestsView {
            table_id: self.inner.table_id,
            user_id: self.inner.user_id,
        }
    }
}

impl Viewable for TeamsUsersRole {
    fn list_route() -> AppRoute {
        AppRoute::TeamsUsersRoles {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::TeamsUsersRolesView {
            table_id: self.table_id,
            user_id: self.user_id,
        }
    }
}

impl Insertable for TeamsUsersRole {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::TeamsUsersRolesNewWithTable { table_id };
            }
            if let Some(user_id) = filter.user_id {
                return AppRoute::TeamsUsersRolesNewWithUser { user_id };
            }
        }
        AppRoute::TeamsUsersRolesNew
    }
}

impl Viewable for NestedTeamsUsersRole {
    fn list_route() -> AppRoute {
        AppRoute::TeamsUsersRoles {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::TeamsUsersRolesView {
            table_id: self.inner.table_id,
            user_id: self.inner.user_id,
        }
    }
}

impl Insertable for NestedTeamsUsersRole {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::TeamsUsersRolesNewWithTable { table_id };
            }
            if let Some(user_id) = filter.user_id {
                return AppRoute::TeamsUsersRolesNewWithUser { user_id };
            }
        }
        AppRoute::TeamsUsersRolesNew
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

impl Updatable for User {
    fn update_route(&self) -> AppRoute {
        AppRoute::UsersUpdate { id: self.id }
    }
}

impl Viewable for NestedUser {
    fn list_route() -> AppRoute {
        AppRoute::Users {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::UsersView { id: self.inner.id }
    }
}

impl Updatable for NestedUser {
    fn update_route(&self) -> AppRoute {
        AppRoute::UsersUpdate { id: self.inner.id }
    }
}

impl Viewable for UsersUsersRoleInvitation {
    fn list_route() -> AppRoute {
        AppRoute::UsersUsersRoleInvitations {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::UsersUsersRoleInvitationsView {
            table_id: self.table_id,
            user_id: self.user_id,
        }
    }
}

impl Insertable for UsersUsersRoleInvitation {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::UsersUsersRoleInvitationsNewWithTable { table_id };
            }
            if let Some(user_id) = filter.user_id {
                return AppRoute::UsersUsersRoleInvitationsNewWithUser { user_id };
            }
        }
        AppRoute::UsersUsersRoleInvitationsNew
    }
}

impl Viewable for NestedUsersUsersRoleInvitation {
    fn list_route() -> AppRoute {
        AppRoute::UsersUsersRoleInvitations {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::UsersUsersRoleInvitationsView {
            table_id: self.inner.table_id,
            user_id: self.inner.user_id,
        }
    }
}

impl Insertable for NestedUsersUsersRoleInvitation {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::UsersUsersRoleInvitationsNewWithTable { table_id };
            }
            if let Some(user_id) = filter.user_id {
                return AppRoute::UsersUsersRoleInvitationsNewWithUser { user_id };
            }
        }
        AppRoute::UsersUsersRoleInvitationsNew
    }
}

impl Viewable for UsersUsersRoleRequest {
    fn list_route() -> AppRoute {
        AppRoute::UsersUsersRoleRequests {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::UsersUsersRoleRequestsView {
            table_id: self.table_id,
            user_id: self.user_id,
        }
    }
}

impl Viewable for NestedUsersUsersRoleRequest {
    fn list_route() -> AppRoute {
        AppRoute::UsersUsersRoleRequests {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::UsersUsersRoleRequestsView {
            table_id: self.inner.table_id,
            user_id: self.inner.user_id,
        }
    }
}

impl Viewable for UsersUsersRole {
    fn list_route() -> AppRoute {
        AppRoute::UsersUsersRoles {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::UsersUsersRolesView {
            table_id: self.table_id,
            user_id: self.user_id,
        }
    }
}

impl Insertable for UsersUsersRole {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::UsersUsersRolesNewWithTable { table_id };
            }
            if let Some(user_id) = filter.user_id {
                return AppRoute::UsersUsersRolesNewWithUser { user_id };
            }
        }
        AppRoute::UsersUsersRolesNew
    }
}

impl Viewable for NestedUsersUsersRole {
    fn list_route() -> AppRoute {
        AppRoute::UsersUsersRoles {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::UsersUsersRolesView {
            table_id: self.inner.table_id,
            user_id: self.inner.user_id,
        }
    }
}

impl Insertable for NestedUsersUsersRole {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::UsersUsersRolesNewWithTable { table_id };
            }
            if let Some(user_id) = filter.user_id {
                return AppRoute::UsersUsersRolesNewWithUser { user_id };
            }
        }
        AppRoute::UsersUsersRolesNew
    }
}

#[derive(Debug, Clone, PartialEq, Routable, Eq)]
pub enum AppRoute {
    #[at("/countries")]
    Countries,
    #[at("/countries/:id")]
    CountriesView { id: i32 },
    #[at("/derived_samples")]
    DerivedSamples,
    #[at("/derived_samples/:parent_sample_id/:child_sample_id")]
    DerivedSamplesView {
        parent_sample_id: uuid::Uuid,
        child_sample_id: uuid::Uuid,
    },
    #[at("/derived_samples/new")]
    DerivedSamplesNew,
    #[at("/derived_samples/new/parent_sample/:parent_sample_id")]
    DerivedSamplesNewWithParentSample { parent_sample_id: uuid::Uuid },
    #[at("/derived_samples/new/child_sample/:child_sample_id")]
    DerivedSamplesNewWithChildSample { child_sample_id: uuid::Uuid },
    #[at("/derived_samples/:parent_sample_id/:child_sample_id/update")]
    DerivedSamplesUpdate {
        parent_sample_id: uuid::Uuid,
        child_sample_id: uuid::Uuid,
    },
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
    #[at("/organism_taxa")]
    OrganismTaxa,
    #[at("/organism_taxa/:organism_id/:taxon_id")]
    OrganismTaxaView {
        organism_id: uuid::Uuid,
        taxon_id: i32,
    },
    #[at("/organism_taxa/new")]
    OrganismTaxaNew,
    #[at("/organism_taxa/new/organism/:organism_id")]
    OrganismTaxaNewWithOrganism { organism_id: uuid::Uuid },
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
    #[at("/projects_teams_role_invitations")]
    ProjectsTeamsRoleInvitations,
    #[at("/projects_teams_role_invitations/:table_id/:team_id")]
    ProjectsTeamsRoleInvitationsView { table_id: i32, team_id: i32 },
    #[at("/projects_teams_role_invitations/new")]
    ProjectsTeamsRoleInvitationsNew,
    #[at("/projects_teams_role_invitations/new/table/:table_id")]
    ProjectsTeamsRoleInvitationsNewWithTable { table_id: i32 },
    #[at("/projects_teams_role_invitations/new/team/:team_id")]
    ProjectsTeamsRoleInvitationsNewWithTeam { team_id: i32 },
    #[at("/projects_teams_role_requests")]
    ProjectsTeamsRoleRequests,
    #[at("/projects_teams_role_requests/:table_id/:team_id")]
    ProjectsTeamsRoleRequestsView { table_id: i32, team_id: i32 },
    #[at("/projects_teams_roles")]
    ProjectsTeamsRoles,
    #[at("/projects_teams_roles/:table_id/:team_id")]
    ProjectsTeamsRolesView { table_id: i32, team_id: i32 },
    #[at("/projects_teams_roles/new")]
    ProjectsTeamsRolesNew,
    #[at("/projects_teams_roles/new/table/:table_id")]
    ProjectsTeamsRolesNewWithTable { table_id: i32 },
    #[at("/projects_teams_roles/new/team/:team_id")]
    ProjectsTeamsRolesNewWithTeam { team_id: i32 },
    #[at("/projects_users_role_invitations")]
    ProjectsUsersRoleInvitations,
    #[at("/projects_users_role_invitations/:table_id/:user_id")]
    ProjectsUsersRoleInvitationsView { table_id: i32, user_id: i32 },
    #[at("/projects_users_role_invitations/new")]
    ProjectsUsersRoleInvitationsNew,
    #[at("/projects_users_role_invitations/new/table/:table_id")]
    ProjectsUsersRoleInvitationsNewWithTable { table_id: i32 },
    #[at("/projects_users_role_invitations/new/user/:user_id")]
    ProjectsUsersRoleInvitationsNewWithUser { user_id: i32 },
    #[at("/projects_users_role_requests")]
    ProjectsUsersRoleRequests,
    #[at("/projects_users_role_requests/:table_id/:user_id")]
    ProjectsUsersRoleRequestsView { table_id: i32, user_id: i32 },
    #[at("/projects_users_roles")]
    ProjectsUsersRoles,
    #[at("/projects_users_roles/:table_id/:user_id")]
    ProjectsUsersRolesView { table_id: i32, user_id: i32 },
    #[at("/projects_users_roles/new")]
    ProjectsUsersRolesNew,
    #[at("/projects_users_roles/new/table/:table_id")]
    ProjectsUsersRolesNewWithTable { table_id: i32 },
    #[at("/projects_users_roles/new/user/:user_id")]
    ProjectsUsersRolesNewWithUser { user_id: i32 },
    #[at("/ranks")]
    Ranks,
    #[at("/ranks/:id")]
    RanksView { id: i32 },
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
    #[at("/sample_taxa")]
    SampleTaxa,
    #[at("/sample_taxa/:sample_id/:taxon_id")]
    SampleTaxaView {
        sample_id: uuid::Uuid,
        taxon_id: i32,
    },
    #[at("/sample_taxa/new")]
    SampleTaxaNew,
    #[at("/sample_taxa/new/sample/:sample_id")]
    SampleTaxaNewWithSample { sample_id: uuid::Uuid },
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
    #[at("/taxa")]
    Taxa,
    #[at("/taxa/:id")]
    TaxaView { id: i32 },
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
    #[at("/teams_teams_role_invitations")]
    TeamsTeamsRoleInvitations,
    #[at("/teams_teams_role_invitations/:table_id/:team_id")]
    TeamsTeamsRoleInvitationsView { table_id: i32, team_id: i32 },
    #[at("/teams_teams_role_invitations/new")]
    TeamsTeamsRoleInvitationsNew,
    #[at("/teams_teams_role_invitations/new/table/:table_id")]
    TeamsTeamsRoleInvitationsNewWithTable { table_id: i32 },
    #[at("/teams_teams_role_invitations/new/team/:team_id")]
    TeamsTeamsRoleInvitationsNewWithTeam { team_id: i32 },
    #[at("/teams_users_role_invitations")]
    TeamsUsersRoleInvitations,
    #[at("/teams_users_role_invitations/:table_id/:user_id")]
    TeamsUsersRoleInvitationsView { table_id: i32, user_id: i32 },
    #[at("/teams_users_role_invitations/new")]
    TeamsUsersRoleInvitationsNew,
    #[at("/teams_users_role_invitations/new/table/:table_id")]
    TeamsUsersRoleInvitationsNewWithTable { table_id: i32 },
    #[at("/teams_users_role_invitations/new/user/:user_id")]
    TeamsUsersRoleInvitationsNewWithUser { user_id: i32 },
    #[at("/teams_users_role_requests")]
    TeamsUsersRoleRequests,
    #[at("/teams_users_role_requests/:table_id/:user_id")]
    TeamsUsersRoleRequestsView { table_id: i32, user_id: i32 },
    #[at("/teams_users_roles")]
    TeamsUsersRoles,
    #[at("/teams_users_roles/:table_id/:user_id")]
    TeamsUsersRolesView { table_id: i32, user_id: i32 },
    #[at("/teams_users_roles/new")]
    TeamsUsersRolesNew,
    #[at("/teams_users_roles/new/table/:table_id")]
    TeamsUsersRolesNewWithTable { table_id: i32 },
    #[at("/teams_users_roles/new/user/:user_id")]
    TeamsUsersRolesNewWithUser { user_id: i32 },
    #[at("/users")]
    Users,
    #[at("/users/:id")]
    UsersView { id: i32 },
    #[at("/users/:id/update")]
    UsersUpdate { id: i32 },
    #[at("/users_users_role_invitations")]
    UsersUsersRoleInvitations,
    #[at("/users_users_role_invitations/:table_id/:user_id")]
    UsersUsersRoleInvitationsView { table_id: i32, user_id: i32 },
    #[at("/users_users_role_invitations/new")]
    UsersUsersRoleInvitationsNew,
    #[at("/users_users_role_invitations/new/table/:table_id")]
    UsersUsersRoleInvitationsNewWithTable { table_id: i32 },
    #[at("/users_users_role_invitations/new/user/:user_id")]
    UsersUsersRoleInvitationsNewWithUser { user_id: i32 },
    #[at("/users_users_role_requests")]
    UsersUsersRoleRequests,
    #[at("/users_users_role_requests/:table_id/:user_id")]
    UsersUsersRoleRequestsView { table_id: i32, user_id: i32 },
    #[at("/users_users_roles")]
    UsersUsersRoles,
    #[at("/users_users_roles/:table_id/:user_id")]
    UsersUsersRolesView { table_id: i32, user_id: i32 },
    #[at("/users_users_roles/new")]
    UsersUsersRolesNew,
    #[at("/users_users_roles/new/table/:table_id")]
    UsersUsersRolesNewWithTable { table_id: i32 },
    #[at("/users_users_roles/new/user/:user_id")]
    UsersUsersRolesNewWithUser { user_id: i32 },
    #[at("/")]
    Home,
    #[at("/collect")]
    Collect,
    #[at("/project-selection/:source_page")]
    ProjectSelection { source_page: String },
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl AppRoute {
    pub fn is_project_selection(&self) -> bool {
        matches!(self, AppRoute::ProjectSelection { .. })
    }
}

/// The switch to map each instance of the AppRoute to the corresponding page.
///
/// # Arguments
/// * `route` - The route to map.
pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Countries => {
            html! { <BasicList<Country> /> }
        }
        AppRoute::CountriesView { id } => {
            html! { <CountryPage id = {id} /> }
        }
        AppRoute::DerivedSamples => {
            html! { <BasicList<NestedDerivedSample> /> }
        }
        AppRoute::DerivedSamplesView {
            parent_sample_id,
            child_sample_id,
        } => {
            html! { <DerivedSamplePage parent_sample_id = {parent_sample_id} child_sample_id = {child_sample_id} /> }
        }
        AppRoute::DerivedSamplesNew => {
            html! { <CreateDerivedSampleForm /> }
        }
        AppRoute::DerivedSamplesNewWithParentSample { parent_sample_id } => {
            html! { <CreateDerivedSampleForm parent_sample_id={parent_sample_id} /> }
        }
        AppRoute::DerivedSamplesNewWithChildSample { child_sample_id } => {
            html! { <CreateDerivedSampleForm child_sample_id={child_sample_id} /> }
        }
        AppRoute::DerivedSamplesUpdate {
            parent_sample_id,
            child_sample_id,
        } => {
            html! { <UpdateDerivedSampleForm parent_sample_id={parent_sample_id} child_sample_id={child_sample_id} /> }
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
        AppRoute::OrganismTaxa => {
            html! { <BasicList<NestedOrganismTaxon> /> }
        }
        AppRoute::OrganismTaxaView {
            organism_id,
            taxon_id,
        } => {
            html! { <OrganismTaxonPage organism_id = {organism_id} taxon_id = {taxon_id} /> }
        }
        AppRoute::OrganismTaxaNew => {
            html! { <CreateOrganismTaxonForm /> }
        }
        AppRoute::OrganismTaxaNewWithOrganism { organism_id } => {
            html! { <CreateOrganismTaxonForm organism_id={organism_id} /> }
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
        AppRoute::ProjectsTeamsRoleInvitations => {
            html! { <BasicList<NestedProjectsTeamsRoleInvitation> /> }
        }
        AppRoute::ProjectsTeamsRoleInvitationsView { table_id, team_id } => {
            html! { <ProjectsTeamsRoleInvitationPage table_id = {table_id} team_id = {team_id} /> }
        }
        AppRoute::ProjectsTeamsRoleInvitationsNew => {
            html! { <CreateProjectsTeamsRoleInvitationForm /> }
        }
        AppRoute::ProjectsTeamsRoleInvitationsNewWithTable { table_id } => {
            html! { <CreateProjectsTeamsRoleInvitationForm table_id={table_id} /> }
        }
        AppRoute::ProjectsTeamsRoleInvitationsNewWithTeam { team_id } => {
            html! { <CreateProjectsTeamsRoleInvitationForm team_id={team_id} /> }
        }
        AppRoute::ProjectsTeamsRoleRequests => {
            html! { <BasicList<NestedProjectsTeamsRoleRequest> /> }
        }
        AppRoute::ProjectsTeamsRoleRequestsView { table_id, team_id } => {
            html! { <ProjectsTeamsRoleRequestPage table_id = {table_id} team_id = {team_id} /> }
        }
        AppRoute::ProjectsTeamsRoles => {
            html! { <BasicList<NestedProjectsTeamsRole> /> }
        }
        AppRoute::ProjectsTeamsRolesView { table_id, team_id } => {
            html! { <ProjectsTeamsRolePage table_id = {table_id} team_id = {team_id} /> }
        }
        AppRoute::ProjectsTeamsRolesNew => {
            html! { <CreateProjectsTeamsRoleForm /> }
        }
        AppRoute::ProjectsTeamsRolesNewWithTable { table_id } => {
            html! { <CreateProjectsTeamsRoleForm table_id={table_id} /> }
        }
        AppRoute::ProjectsTeamsRolesNewWithTeam { team_id } => {
            html! { <CreateProjectsTeamsRoleForm team_id={team_id} /> }
        }
        AppRoute::ProjectsUsersRoleInvitations => {
            html! { <BasicList<NestedProjectsUsersRoleInvitation> /> }
        }
        AppRoute::ProjectsUsersRoleInvitationsView { table_id, user_id } => {
            html! { <ProjectsUsersRoleInvitationPage table_id = {table_id} user_id = {user_id} /> }
        }
        AppRoute::ProjectsUsersRoleInvitationsNew => {
            html! { <CreateProjectsUsersRoleInvitationForm /> }
        }
        AppRoute::ProjectsUsersRoleInvitationsNewWithTable { table_id } => {
            html! { <CreateProjectsUsersRoleInvitationForm table_id={table_id} /> }
        }
        AppRoute::ProjectsUsersRoleInvitationsNewWithUser { user_id } => {
            html! { <CreateProjectsUsersRoleInvitationForm user_id={user_id} /> }
        }
        AppRoute::ProjectsUsersRoleRequests => {
            html! { <BasicList<NestedProjectsUsersRoleRequest> /> }
        }
        AppRoute::ProjectsUsersRoleRequestsView { table_id, user_id } => {
            html! { <ProjectsUsersRoleRequestPage table_id = {table_id} user_id = {user_id} /> }
        }
        AppRoute::ProjectsUsersRoles => {
            html! { <BasicList<NestedProjectsUsersRole> /> }
        }
        AppRoute::ProjectsUsersRolesView { table_id, user_id } => {
            html! { <ProjectsUsersRolePage table_id = {table_id} user_id = {user_id} /> }
        }
        AppRoute::ProjectsUsersRolesNew => {
            html! { <CreateProjectsUsersRoleForm /> }
        }
        AppRoute::ProjectsUsersRolesNewWithTable { table_id } => {
            html! { <CreateProjectsUsersRoleForm table_id={table_id} /> }
        }
        AppRoute::ProjectsUsersRolesNewWithUser { user_id } => {
            html! { <CreateProjectsUsersRoleForm user_id={user_id} /> }
        }
        AppRoute::Ranks => {
            html! { <BasicList<NestedRank> /> }
        }
        AppRoute::RanksView { id } => {
            html! { <RankPage id = {id} /> }
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
        AppRoute::SampleTaxa => {
            html! { <BasicList<NestedSampleTaxon> /> }
        }
        AppRoute::SampleTaxaView {
            sample_id,
            taxon_id,
        } => {
            html! { <SampleTaxonPage sample_id = {sample_id} taxon_id = {taxon_id} /> }
        }
        AppRoute::SampleTaxaNew => {
            html! { <CreateSampleTaxonForm /> }
        }
        AppRoute::SampleTaxaNewWithSample { sample_id } => {
            html! { <CreateSampleTaxonForm sample_id={sample_id} /> }
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
        AppRoute::Taxa => {
            html! { <BasicList<NestedTaxon> /> }
        }
        AppRoute::TaxaView { id } => {
            html! { <TaxonPage id = {id} /> }
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
        AppRoute::TeamsTeamsRoleInvitations => {
            html! { <BasicList<NestedTeamsTeamsRoleInvitation> /> }
        }
        AppRoute::TeamsTeamsRoleInvitationsView { table_id, team_id } => {
            html! { <TeamsTeamsRoleInvitationPage table_id = {table_id} team_id = {team_id} /> }
        }
        AppRoute::TeamsTeamsRoleInvitationsNew => {
            html! { <CreateTeamsTeamsRoleInvitationForm /> }
        }
        AppRoute::TeamsTeamsRoleInvitationsNewWithTable { table_id } => {
            html! { <CreateTeamsTeamsRoleInvitationForm table_id={table_id} /> }
        }
        AppRoute::TeamsTeamsRoleInvitationsNewWithTeam { team_id } => {
            html! { <CreateTeamsTeamsRoleInvitationForm team_id={team_id} /> }
        }
        AppRoute::TeamsUsersRoleInvitations => {
            html! { <BasicList<NestedTeamsUsersRoleInvitation> /> }
        }
        AppRoute::TeamsUsersRoleInvitationsView { table_id, user_id } => {
            html! { <TeamsUsersRoleInvitationPage table_id = {table_id} user_id = {user_id} /> }
        }
        AppRoute::TeamsUsersRoleInvitationsNew => {
            html! { <CreateTeamsUsersRoleInvitationForm /> }
        }
        AppRoute::TeamsUsersRoleInvitationsNewWithTable { table_id } => {
            html! { <CreateTeamsUsersRoleInvitationForm table_id={table_id} /> }
        }
        AppRoute::TeamsUsersRoleInvitationsNewWithUser { user_id } => {
            html! { <CreateTeamsUsersRoleInvitationForm user_id={user_id} /> }
        }
        AppRoute::TeamsUsersRoleRequests => {
            html! { <BasicList<NestedTeamsUsersRoleRequest> /> }
        }
        AppRoute::TeamsUsersRoleRequestsView { table_id, user_id } => {
            html! { <TeamsUsersRoleRequestPage table_id = {table_id} user_id = {user_id} /> }
        }
        AppRoute::TeamsUsersRoles => {
            html! { <BasicList<NestedTeamsUsersRole> /> }
        }
        AppRoute::TeamsUsersRolesView { table_id, user_id } => {
            html! { <TeamsUsersRolePage table_id = {table_id} user_id = {user_id} /> }
        }
        AppRoute::TeamsUsersRolesNew => {
            html! { <CreateTeamsUsersRoleForm /> }
        }
        AppRoute::TeamsUsersRolesNewWithTable { table_id } => {
            html! { <CreateTeamsUsersRoleForm table_id={table_id} /> }
        }
        AppRoute::TeamsUsersRolesNewWithUser { user_id } => {
            html! { <CreateTeamsUsersRoleForm user_id={user_id} /> }
        }
        AppRoute::Users => {
            html! { <BasicList<NestedUser> /> }
        }
        AppRoute::UsersView { id } => {
            html! { <UserPage id = {id} /> }
        }
        AppRoute::UsersUpdate { id } => {
            html! { <UpdateUserForm id={id} /> }
        }
        AppRoute::UsersUsersRoleInvitations => {
            html! { <BasicList<NestedUsersUsersRoleInvitation> /> }
        }
        AppRoute::UsersUsersRoleInvitationsView { table_id, user_id } => {
            html! { <UsersUsersRoleInvitationPage table_id = {table_id} user_id = {user_id} /> }
        }
        AppRoute::UsersUsersRoleInvitationsNew => {
            html! { <CreateUsersUsersRoleInvitationForm /> }
        }
        AppRoute::UsersUsersRoleInvitationsNewWithTable { table_id } => {
            html! { <CreateUsersUsersRoleInvitationForm table_id={table_id} /> }
        }
        AppRoute::UsersUsersRoleInvitationsNewWithUser { user_id } => {
            html! { <CreateUsersUsersRoleInvitationForm user_id={user_id} /> }
        }
        AppRoute::UsersUsersRoleRequests => {
            html! { <BasicList<NestedUsersUsersRoleRequest> /> }
        }
        AppRoute::UsersUsersRoleRequestsView { table_id, user_id } => {
            html! { <UsersUsersRoleRequestPage table_id = {table_id} user_id = {user_id} /> }
        }
        AppRoute::UsersUsersRoles => {
            html! { <BasicList<NestedUsersUsersRole> /> }
        }
        AppRoute::UsersUsersRolesView { table_id, user_id } => {
            html! { <UsersUsersRolePage table_id = {table_id} user_id = {user_id} /> }
        }
        AppRoute::UsersUsersRolesNew => {
            html! { <CreateUsersUsersRoleForm /> }
        }
        AppRoute::UsersUsersRolesNewWithTable { table_id } => {
            html! { <CreateUsersUsersRoleForm table_id={table_id} /> }
        }
        AppRoute::UsersUsersRolesNewWithUser { user_id } => {
            html! { <CreateUsersUsersRoleForm user_id={user_id} /> }
        }
        AppRoute::Home => {
            html! { <Home /> }
        }
        AppRoute::Collect => {
            html! { <Collect /> }
        }
        AppRoute::ProjectSelection { source_page } => {
            html! { <ProjectSelection source_page={AppRoute::try_from(source_page).unwrap_or(AppRoute::Home)} /> }
        }
        AppRoute::Login => {
            html! { <Login /> }
        }
        AppRoute::NotFound => {
            html! { <NotFound /> }
        }
    }
}

