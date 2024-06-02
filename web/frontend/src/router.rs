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

impl Viewable for OrganismBioOttTaxonItem {
    fn list_route() -> AppRoute {
        AppRoute::OrganismBioOttTaxonItems {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::OrganismBioOttTaxonItemsView {
            organism_id: self.organism_id,
            taxon_id: self.taxon_id,
        }
    }
}

impl Insertable for OrganismBioOttTaxonItem {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(organism_id) = filter.organism_id {
                return AppRoute::OrganismBioOttTaxonItemsNewWithOrganism { organism_id };
            }
        }
        AppRoute::OrganismBioOttTaxonItemsNew
    }
}

impl Viewable for NestedOrganismBioOttTaxonItem {
    fn list_route() -> AppRoute {
        AppRoute::OrganismBioOttTaxonItems {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::OrganismBioOttTaxonItemsView {
            organism_id: self.inner.organism_id,
            taxon_id: self.inner.taxon_id,
        }
    }
}

impl Insertable for NestedOrganismBioOttTaxonItem {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(organism_id) = filter.organism_id {
                return AppRoute::OrganismBioOttTaxonItemsNewWithOrganism { organism_id };
            }
        }
        AppRoute::OrganismBioOttTaxonItemsNew
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

impl Insertable for ProjectsTeamsRoleRequest {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::ProjectsTeamsRoleRequestsNewWithTable { table_id };
            }
            if let Some(team_id) = filter.team_id {
                return AppRoute::ProjectsTeamsRoleRequestsNewWithTeam { team_id };
            }
        }
        AppRoute::ProjectsTeamsRoleRequestsNew
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

impl Insertable for NestedProjectsTeamsRoleRequest {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::ProjectsTeamsRoleRequestsNewWithTable { table_id };
            }
            if let Some(team_id) = filter.team_id {
                return AppRoute::ProjectsTeamsRoleRequestsNewWithTeam { team_id };
            }
        }
        AppRoute::ProjectsTeamsRoleRequestsNew
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

impl Insertable for ProjectsUsersRoleRequest {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::ProjectsUsersRoleRequestsNewWithTable { table_id };
            }
            if let Some(user_id) = filter.user_id {
                return AppRoute::ProjectsUsersRoleRequestsNewWithUser { user_id };
            }
        }
        AppRoute::ProjectsUsersRoleRequestsNew
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

impl Insertable for NestedProjectsUsersRoleRequest {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::ProjectsUsersRoleRequestsNewWithTable { table_id };
            }
            if let Some(user_id) = filter.user_id {
                return AppRoute::ProjectsUsersRoleRequestsNewWithUser { user_id };
            }
        }
        AppRoute::ProjectsUsersRoleRequestsNew
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

impl Viewable for SampleBioOttTaxonItem {
    fn list_route() -> AppRoute {
        AppRoute::SampleBioOttTaxonItems {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::SampleBioOttTaxonItemsView {
            sample_id: self.sample_id,
            taxon_id: self.taxon_id,
        }
    }
}

impl Insertable for SampleBioOttTaxonItem {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(sample_id) = filter.sample_id {
                return AppRoute::SampleBioOttTaxonItemsNewWithSample { sample_id };
            }
        }
        AppRoute::SampleBioOttTaxonItemsNew
    }
}

impl Viewable for NestedSampleBioOttTaxonItem {
    fn list_route() -> AppRoute {
        AppRoute::SampleBioOttTaxonItems {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::SampleBioOttTaxonItemsView {
            sample_id: self.inner.sample_id,
            taxon_id: self.inner.taxon_id,
        }
    }
}

impl Insertable for NestedSampleBioOttTaxonItem {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(sample_id) = filter.sample_id {
                return AppRoute::SampleBioOttTaxonItemsNewWithSample { sample_id };
            }
        }
        AppRoute::SampleBioOttTaxonItemsNew
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

impl Viewable for Spectra {
    fn list_route() -> AppRoute {
        AppRoute::Spectra {}
    }
    fn view_route(&self) -> AppRoute {
        AppRoute::SpectraView { id: self.id }
    }
}

impl Insertable for Spectra {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(spectra_collection_id) = filter.spectra_collection_id {
                return AppRoute::SpectraNewWithSpectraCollection {
                    spectra_collection_id,
                };
            }
        }
        AppRoute::SpectraNew
    }
}

impl Updatable for Spectra {
    fn update_route(&self) -> AppRoute {
        AppRoute::SpectraUpdate { id: self.id }
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

impl Insertable for NestedSpectra {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(spectra_collection_id) = filter.spectra_collection_id {
                return AppRoute::SpectraNewWithSpectraCollection {
                    spectra_collection_id,
                };
            }
        }
        AppRoute::SpectraNew
    }
}

impl Updatable for NestedSpectra {
    fn update_route(&self) -> AppRoute {
        AppRoute::SpectraUpdate { id: self.inner.id }
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

impl Insertable for TeamsUsersRoleRequest {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::TeamsUsersRoleRequestsNewWithTable { table_id };
            }
            if let Some(user_id) = filter.user_id {
                return AppRoute::TeamsUsersRoleRequestsNewWithUser { user_id };
            }
        }
        AppRoute::TeamsUsersRoleRequestsNew
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

impl Insertable for NestedTeamsUsersRoleRequest {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::TeamsUsersRoleRequestsNewWithTable { table_id };
            }
            if let Some(user_id) = filter.user_id {
                return AppRoute::TeamsUsersRoleRequestsNewWithUser { user_id };
            }
        }
        AppRoute::TeamsUsersRoleRequestsNew
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

impl Insertable for UsersUsersRoleRequest {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::UsersUsersRoleRequestsNewWithTable { table_id };
            }
            if let Some(user_id) = filter.user_id {
                return AppRoute::UsersUsersRoleRequestsNewWithUser { user_id };
            }
        }
        AppRoute::UsersUsersRoleRequestsNew
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

impl Insertable for NestedUsersUsersRoleRequest {
    fn new_route(filter: Option<&Self::Filter>) -> AppRoute {
        if let Some(filter) = filter {
            if let Some(table_id) = filter.table_id {
                return AppRoute::UsersUsersRoleRequestsNewWithTable { table_id };
            }
            if let Some(user_id) = filter.user_id {
                return AppRoute::UsersUsersRoleRequestsNewWithUser { user_id };
            }
        }
        AppRoute::UsersUsersRoleRequestsNew
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
    #[at("/organism_bio_ott_taxon_items")]
    OrganismBioOttTaxonItems,
    #[at("/organism_bio_ott_taxon_items/:organism_id/:taxon_id")]
    OrganismBioOttTaxonItemsView {
        organism_id: uuid::Uuid,
        taxon_id: i32,
    },
    #[at("/organism_bio_ott_taxon_items/new")]
    OrganismBioOttTaxonItemsNew,
    #[at("/organism_bio_ott_taxon_items/new/organism/:organism_id")]
    OrganismBioOttTaxonItemsNewWithOrganism { organism_id: uuid::Uuid },
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
    #[at("/projects_teams_role_requests/new")]
    ProjectsTeamsRoleRequestsNew,
    #[at("/projects_teams_role_requests/new/table/:table_id")]
    ProjectsTeamsRoleRequestsNewWithTable { table_id: i32 },
    #[at("/projects_teams_role_requests/new/team/:team_id")]
    ProjectsTeamsRoleRequestsNewWithTeam { team_id: i32 },
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
    #[at("/projects_users_role_requests/new")]
    ProjectsUsersRoleRequestsNew,
    #[at("/projects_users_role_requests/new/table/:table_id")]
    ProjectsUsersRoleRequestsNewWithTable { table_id: i32 },
    #[at("/projects_users_role_requests/new/user/:user_id")]
    ProjectsUsersRoleRequestsNewWithUser { user_id: i32 },
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
    #[at("/sample_bio_ott_taxon_items")]
    SampleBioOttTaxonItems,
    #[at("/sample_bio_ott_taxon_items/:sample_id/:taxon_id")]
    SampleBioOttTaxonItemsView {
        sample_id: uuid::Uuid,
        taxon_id: i32,
    },
    #[at("/sample_bio_ott_taxon_items/new")]
    SampleBioOttTaxonItemsNew,
    #[at("/sample_bio_ott_taxon_items/new/sample/:sample_id")]
    SampleBioOttTaxonItemsNewWithSample { sample_id: uuid::Uuid },
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
    #[at("/teams_users_role_requests/new")]
    TeamsUsersRoleRequestsNew,
    #[at("/teams_users_role_requests/new/table/:table_id")]
    TeamsUsersRoleRequestsNewWithTable { table_id: i32 },
    #[at("/teams_users_role_requests/new/user/:user_id")]
    TeamsUsersRoleRequestsNewWithUser { user_id: i32 },
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
    #[at("/users_users_role_requests/new")]
    UsersUsersRoleRequestsNew,
    #[at("/users_users_role_requests/new/table/:table_id")]
    UsersUsersRoleRequestsNewWithTable { table_id: i32 },
    #[at("/users_users_role_requests/new/user/:user_id")]
    UsersUsersRoleRequestsNewWithUser { user_id: i32 },
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
        AppRoute::OrganismBioOttTaxonItems => {
            html! { <BasicList<NestedOrganismBioOttTaxonItem> /> }
        }
        AppRoute::OrganismBioOttTaxonItemsView {
            organism_id,
            taxon_id,
        } => {
            html! { <OrganismBioOttTaxonItemPage organism_id = {organism_id} taxon_id = {taxon_id} /> }
        }
        AppRoute::OrganismBioOttTaxonItemsNew => {
            html! { <CreateOrganismBioOttTaxonItemForm /> }
        }
        AppRoute::OrganismBioOttTaxonItemsNewWithOrganism { organism_id } => {
            html! { <CreateOrganismBioOttTaxonItemForm organism_id={organism_id} /> }
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
        AppRoute::ProjectsTeamsRoleRequestsNew => {
            html! { <CreateProjectsTeamsRoleRequestForm /> }
        }
        AppRoute::ProjectsTeamsRoleRequestsNewWithTable { table_id } => {
            html! { <CreateProjectsTeamsRoleRequestForm table_id={table_id} /> }
        }
        AppRoute::ProjectsTeamsRoleRequestsNewWithTeam { team_id } => {
            html! { <CreateProjectsTeamsRoleRequestForm team_id={team_id} /> }
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
        AppRoute::ProjectsUsersRoleRequestsNew => {
            html! { <CreateProjectsUsersRoleRequestForm /> }
        }
        AppRoute::ProjectsUsersRoleRequestsNewWithTable { table_id } => {
            html! { <CreateProjectsUsersRoleRequestForm table_id={table_id} /> }
        }
        AppRoute::ProjectsUsersRoleRequestsNewWithUser { user_id } => {
            html! { <CreateProjectsUsersRoleRequestForm user_id={user_id} /> }
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
        AppRoute::SampleBioOttTaxonItems => {
            html! { <BasicList<NestedSampleBioOttTaxonItem> /> }
        }
        AppRoute::SampleBioOttTaxonItemsView {
            sample_id,
            taxon_id,
        } => {
            html! { <SampleBioOttTaxonItemPage sample_id = {sample_id} taxon_id = {taxon_id} /> }
        }
        AppRoute::SampleBioOttTaxonItemsNew => {
            html! { <CreateSampleBioOttTaxonItemForm /> }
        }
        AppRoute::SampleBioOttTaxonItemsNewWithSample { sample_id } => {
            html! { <CreateSampleBioOttTaxonItemForm sample_id={sample_id} /> }
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
        AppRoute::TeamsUsersRoleRequestsNew => {
            html! { <CreateTeamsUsersRoleRequestForm /> }
        }
        AppRoute::TeamsUsersRoleRequestsNewWithTable { table_id } => {
            html! { <CreateTeamsUsersRoleRequestForm table_id={table_id} /> }
        }
        AppRoute::TeamsUsersRoleRequestsNewWithUser { user_id } => {
            html! { <CreateTeamsUsersRoleRequestForm user_id={user_id} /> }
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
            html! { <BasicList<User> /> }
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
        AppRoute::UsersUsersRoleRequestsNew => {
            html! { <CreateUsersUsersRoleRequestForm /> }
        }
        AppRoute::UsersUsersRoleRequestsNewWithTable { table_id } => {
            html! { <CreateUsersUsersRoleRequestForm table_id={table_id} /> }
        }
        AppRoute::UsersUsersRoleRequestsNewWithUser { user_id } => {
            html! { <CreateUsersUsersRoleRequestForm user_id={user_id} /> }
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
        AppRoute::Login => {
            html! { <Login /> }
        }
        AppRoute::NotFound => {
            html! { <NotFound /> }
        }
    }
}
