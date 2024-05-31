//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use crate::database::*;

pub trait Searchable<const EDIT: bool>: Filtrable {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select;
}
impl Searchable<false> for NestedBioOttRank {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::BioOttRanks,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedBioOttTaxonItem {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::BioOttTaxonItems,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedDerivedSample {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::DerivedSamples,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<true> for NestedDerivedSample {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search_updatables(
             Table::DerivedSamples,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedDocumentFormat {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::DocumentFormats,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedNameplateCategory {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::NameplateCategories,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedNameplate {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::Nameplates,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<true> for NestedNameplate {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search_updatables(
             Table::Nameplates,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedObservationSubject {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::ObservationSubjects,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedObservation {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::Observations,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<true> for NestedObservation {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search_updatables(
             Table::Observations,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedOrganismBioOttTaxonItem {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::OrganismBioOttTaxonItems,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedOrganism {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::Organisms,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<true> for NestedOrganism {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search_updatables(
             Table::Organisms,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedOrganization {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::Organizations,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedProjectState {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::ProjectStates,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedProject {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::Projects,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<true> for NestedProject {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search_updatables(
             Table::Projects,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedProjectsTeamsRoleInvitation {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::ProjectsTeamsRoleInvitations,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedProjectsTeamsRoleRequest {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::ProjectsTeamsRoleRequests,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedProjectsTeamsRole {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::ProjectsTeamsRoles,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedProjectsUsersRoleInvitation {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::ProjectsUsersRoleInvitations,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedProjectsUsersRoleRequest {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::ProjectsUsersRoleRequests,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedProjectsUsersRole {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::ProjectsUsersRoles,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedRole {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::Roles,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedSampleBioOttTaxonItem {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::SampleBioOttTaxonItems,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedSampleContainerCategory {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::SampleContainerCategories,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedSampleContainer {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::SampleContainers,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<true> for NestedSampleContainer {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search_updatables(
             Table::SampleContainers,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedSampleState {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::SampleStates,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedSample {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::Samples,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<true> for NestedSample {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search_updatables(
             Table::Samples,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedSpectraCollection {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::SpectraCollections,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<true> for NestedSpectraCollection {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search_updatables(
             Table::SpectraCollections,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedTeamState {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::TeamStates,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedTeam {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::Teams,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<true> for NestedTeam {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search_updatables(
             Table::Teams,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedTeamsTeamsRoleInvitation {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::TeamsTeamsRoleInvitations,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedTeamsUsersRoleInvitation {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::TeamsUsersRoleInvitations,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedTeamsUsersRoleRequest {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::TeamsUsersRoleRequests,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedTeamsUsersRole {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::TeamsUsersRoles,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedUsersUsersRoleInvitation {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::UsersUsersRoleInvitations,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedUsersUsersRoleRequest {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::UsersUsersRoleRequests,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NestedUsersUsersRole {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::UsersUsersRoles,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for BioOttRank {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::BioOttRanks,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for BioOttTaxonItem {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::BioOttTaxonItems,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for Color {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::Colors,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for Country {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::Countries,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for DerivedSample {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::DerivedSamples,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<true> for DerivedSample {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search_updatables(
             Table::DerivedSamples,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for DocumentFormat {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::DocumentFormats,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for FontAwesomeIcon {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::FontAwesomeIcons,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for NameplateCategory {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::NameplateCategories,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for Nameplate {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::Nameplates,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<true> for Nameplate {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search_updatables(
             Table::Nameplates,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for ObservationSubject {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::ObservationSubjects,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for Observation {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::Observations,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<true> for Observation {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search_updatables(
             Table::Observations,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for OrganismBioOttTaxonItem {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::OrganismBioOttTaxonItems,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for Organism {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::Organisms,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<true> for Organism {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search_updatables(
             Table::Organisms,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for Organization {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::Organizations,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for ProjectState {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::ProjectStates,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for Project {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::Projects,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<true> for Project {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search_updatables(
             Table::Projects,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for ProjectsTeamsRoleInvitation {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::ProjectsTeamsRoleInvitations,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for ProjectsTeamsRoleRequest {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::ProjectsTeamsRoleRequests,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for ProjectsTeamsRole {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::ProjectsTeamsRoles,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for ProjectsUsersRoleInvitation {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::ProjectsUsersRoleInvitations,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for ProjectsUsersRoleRequest {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::ProjectsUsersRoleRequests,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for ProjectsUsersRole {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::ProjectsUsersRoles,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for Role {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::Roles,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for SampleBioOttTaxonItem {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::SampleBioOttTaxonItems,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for SampleContainerCategory {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::SampleContainerCategories,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for SampleContainer {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::SampleContainers,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<true> for SampleContainer {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search_updatables(
             Table::SampleContainers,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for SampleState {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::SampleStates,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for Sample {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::Samples,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<true> for Sample {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search_updatables(
             Table::Samples,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for SpectraCollection {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::SpectraCollections,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<true> for SpectraCollection {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search_updatables(
             Table::SpectraCollections,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for TeamState {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::TeamStates,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for Team {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::Teams,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<true> for Team {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search_updatables(
             Table::Teams,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for TeamsTeamsRoleInvitation {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::TeamsTeamsRoleInvitations,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for TeamsUsersRoleInvitation {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::TeamsUsersRoleInvitations,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for TeamsUsersRoleRequest {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::TeamsUsersRoleRequests,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for TeamsUsersRole {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::TeamsUsersRoles,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for Unit {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::Units,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for User {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::Users,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<true> for User {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search_updatables(
             Table::Users,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for UsersUsersRoleInvitation {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::UsersUsersRoleInvitations,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for UsersUsersRoleRequest {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::UsersUsersRoleRequests,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<false> for UsersUsersRole {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::UsersUsersRoles,
              filter,
              query,
              limit,
              offset,
        )
    }
}
