//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use crate::database::*;
use std::rc::Rc;
use serde::{Serialize, Deserialize};

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
impl Searchable<false> for NestedUnit {
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
impl Searchable<false> for NestedUser {
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
impl Searchable<true> for NestedUser {
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
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum SearchableStruct {
    NestedBioOttRank(Rc<NestedBioOttRank>),
    NestedBioOttTaxonItem(Rc<NestedBioOttTaxonItem>),
    NestedDerivedSample(Rc<NestedDerivedSample>),
    NestedDocumentFormat(Rc<NestedDocumentFormat>),
    NestedNameplateCategory(Rc<NestedNameplateCategory>),
    NestedNameplate(Rc<NestedNameplate>),
    NestedObservationSubject(Rc<NestedObservationSubject>),
    NestedObservation(Rc<NestedObservation>),
    NestedOrganismBioOttTaxonItem(Rc<NestedOrganismBioOttTaxonItem>),
    NestedOrganism(Rc<NestedOrganism>),
    NestedOrganization(Rc<NestedOrganization>),
    NestedProjectState(Rc<NestedProjectState>),
    NestedProject(Rc<NestedProject>),
    NestedProjectsTeamsRoleInvitation(Rc<NestedProjectsTeamsRoleInvitation>),
    NestedProjectsTeamsRoleRequest(Rc<NestedProjectsTeamsRoleRequest>),
    NestedProjectsTeamsRole(Rc<NestedProjectsTeamsRole>),
    NestedProjectsUsersRoleInvitation(Rc<NestedProjectsUsersRoleInvitation>),
    NestedProjectsUsersRoleRequest(Rc<NestedProjectsUsersRoleRequest>),
    NestedProjectsUsersRole(Rc<NestedProjectsUsersRole>),
    NestedRole(Rc<NestedRole>),
    NestedSampleBioOttTaxonItem(Rc<NestedSampleBioOttTaxonItem>),
    NestedSampleContainerCategory(Rc<NestedSampleContainerCategory>),
    NestedSampleContainer(Rc<NestedSampleContainer>),
    NestedSampleState(Rc<NestedSampleState>),
    NestedSample(Rc<NestedSample>),
    NestedSpectraCollection(Rc<NestedSpectraCollection>),
    NestedTeamState(Rc<NestedTeamState>),
    NestedTeam(Rc<NestedTeam>),
    NestedTeamsTeamsRoleInvitation(Rc<NestedTeamsTeamsRoleInvitation>),
    NestedTeamsUsersRoleInvitation(Rc<NestedTeamsUsersRoleInvitation>),
    NestedTeamsUsersRoleRequest(Rc<NestedTeamsUsersRoleRequest>),
    NestedTeamsUsersRole(Rc<NestedTeamsUsersRole>),
    NestedUnit(Rc<NestedUnit>),
    NestedUser(Rc<NestedUser>),
    NestedUsersUsersRoleInvitation(Rc<NestedUsersUsersRoleInvitation>),
    NestedUsersUsersRoleRequest(Rc<NestedUsersUsersRoleRequest>),
    NestedUsersUsersRole(Rc<NestedUsersUsersRole>),
    Color(Rc<Color>),
    Country(Rc<Country>),
    FontAwesomeIcon(Rc<FontAwesomeIcon>),
}

impl Filtrable for SearchableStruct {
    type Filter = EmptyFilter;
}
impl Searchable<false> for SearchableStruct {
    fn search_task(_filter: Option<&Self::Filter>, query: String, limit: i64, _offset: i64) -> super::Select {
        super::Select::search_all(
            query,
            limit,
        )
    }
}
impl From<NestedBioOttRank> for SearchableStruct {
    fn from(value: NestedBioOttRank) -> Self {
        SearchableStruct::NestedBioOttRank(Rc::from(value))
    }
}
impl From<NestedBioOttTaxonItem> for SearchableStruct {
    fn from(value: NestedBioOttTaxonItem) -> Self {
        SearchableStruct::NestedBioOttTaxonItem(Rc::from(value))
    }
}
impl From<NestedDerivedSample> for SearchableStruct {
    fn from(value: NestedDerivedSample) -> Self {
        SearchableStruct::NestedDerivedSample(Rc::from(value))
    }
}
impl From<NestedDocumentFormat> for SearchableStruct {
    fn from(value: NestedDocumentFormat) -> Self {
        SearchableStruct::NestedDocumentFormat(Rc::from(value))
    }
}
impl From<NestedNameplateCategory> for SearchableStruct {
    fn from(value: NestedNameplateCategory) -> Self {
        SearchableStruct::NestedNameplateCategory(Rc::from(value))
    }
}
impl From<NestedNameplate> for SearchableStruct {
    fn from(value: NestedNameplate) -> Self {
        SearchableStruct::NestedNameplate(Rc::from(value))
    }
}
impl From<NestedObservationSubject> for SearchableStruct {
    fn from(value: NestedObservationSubject) -> Self {
        SearchableStruct::NestedObservationSubject(Rc::from(value))
    }
}
impl From<NestedObservation> for SearchableStruct {
    fn from(value: NestedObservation) -> Self {
        SearchableStruct::NestedObservation(Rc::from(value))
    }
}
impl From<NestedOrganismBioOttTaxonItem> for SearchableStruct {
    fn from(value: NestedOrganismBioOttTaxonItem) -> Self {
        SearchableStruct::NestedOrganismBioOttTaxonItem(Rc::from(value))
    }
}
impl From<NestedOrganism> for SearchableStruct {
    fn from(value: NestedOrganism) -> Self {
        SearchableStruct::NestedOrganism(Rc::from(value))
    }
}
impl From<NestedOrganization> for SearchableStruct {
    fn from(value: NestedOrganization) -> Self {
        SearchableStruct::NestedOrganization(Rc::from(value))
    }
}
impl From<NestedProjectState> for SearchableStruct {
    fn from(value: NestedProjectState) -> Self {
        SearchableStruct::NestedProjectState(Rc::from(value))
    }
}
impl From<NestedProject> for SearchableStruct {
    fn from(value: NestedProject) -> Self {
        SearchableStruct::NestedProject(Rc::from(value))
    }
}
impl From<NestedProjectsTeamsRoleInvitation> for SearchableStruct {
    fn from(value: NestedProjectsTeamsRoleInvitation) -> Self {
        SearchableStruct::NestedProjectsTeamsRoleInvitation(Rc::from(value))
    }
}
impl From<NestedProjectsTeamsRoleRequest> for SearchableStruct {
    fn from(value: NestedProjectsTeamsRoleRequest) -> Self {
        SearchableStruct::NestedProjectsTeamsRoleRequest(Rc::from(value))
    }
}
impl From<NestedProjectsTeamsRole> for SearchableStruct {
    fn from(value: NestedProjectsTeamsRole) -> Self {
        SearchableStruct::NestedProjectsTeamsRole(Rc::from(value))
    }
}
impl From<NestedProjectsUsersRoleInvitation> for SearchableStruct {
    fn from(value: NestedProjectsUsersRoleInvitation) -> Self {
        SearchableStruct::NestedProjectsUsersRoleInvitation(Rc::from(value))
    }
}
impl From<NestedProjectsUsersRoleRequest> for SearchableStruct {
    fn from(value: NestedProjectsUsersRoleRequest) -> Self {
        SearchableStruct::NestedProjectsUsersRoleRequest(Rc::from(value))
    }
}
impl From<NestedProjectsUsersRole> for SearchableStruct {
    fn from(value: NestedProjectsUsersRole) -> Self {
        SearchableStruct::NestedProjectsUsersRole(Rc::from(value))
    }
}
impl From<NestedRole> for SearchableStruct {
    fn from(value: NestedRole) -> Self {
        SearchableStruct::NestedRole(Rc::from(value))
    }
}
impl From<NestedSampleBioOttTaxonItem> for SearchableStruct {
    fn from(value: NestedSampleBioOttTaxonItem) -> Self {
        SearchableStruct::NestedSampleBioOttTaxonItem(Rc::from(value))
    }
}
impl From<NestedSampleContainerCategory> for SearchableStruct {
    fn from(value: NestedSampleContainerCategory) -> Self {
        SearchableStruct::NestedSampleContainerCategory(Rc::from(value))
    }
}
impl From<NestedSampleContainer> for SearchableStruct {
    fn from(value: NestedSampleContainer) -> Self {
        SearchableStruct::NestedSampleContainer(Rc::from(value))
    }
}
impl From<NestedSampleState> for SearchableStruct {
    fn from(value: NestedSampleState) -> Self {
        SearchableStruct::NestedSampleState(Rc::from(value))
    }
}
impl From<NestedSample> for SearchableStruct {
    fn from(value: NestedSample) -> Self {
        SearchableStruct::NestedSample(Rc::from(value))
    }
}
impl From<NestedSpectraCollection> for SearchableStruct {
    fn from(value: NestedSpectraCollection) -> Self {
        SearchableStruct::NestedSpectraCollection(Rc::from(value))
    }
}
impl From<NestedTeamState> for SearchableStruct {
    fn from(value: NestedTeamState) -> Self {
        SearchableStruct::NestedTeamState(Rc::from(value))
    }
}
impl From<NestedTeam> for SearchableStruct {
    fn from(value: NestedTeam) -> Self {
        SearchableStruct::NestedTeam(Rc::from(value))
    }
}
impl From<NestedTeamsTeamsRoleInvitation> for SearchableStruct {
    fn from(value: NestedTeamsTeamsRoleInvitation) -> Self {
        SearchableStruct::NestedTeamsTeamsRoleInvitation(Rc::from(value))
    }
}
impl From<NestedTeamsUsersRoleInvitation> for SearchableStruct {
    fn from(value: NestedTeamsUsersRoleInvitation) -> Self {
        SearchableStruct::NestedTeamsUsersRoleInvitation(Rc::from(value))
    }
}
impl From<NestedTeamsUsersRoleRequest> for SearchableStruct {
    fn from(value: NestedTeamsUsersRoleRequest) -> Self {
        SearchableStruct::NestedTeamsUsersRoleRequest(Rc::from(value))
    }
}
impl From<NestedTeamsUsersRole> for SearchableStruct {
    fn from(value: NestedTeamsUsersRole) -> Self {
        SearchableStruct::NestedTeamsUsersRole(Rc::from(value))
    }
}
impl From<NestedUnit> for SearchableStruct {
    fn from(value: NestedUnit) -> Self {
        SearchableStruct::NestedUnit(Rc::from(value))
    }
}
impl From<NestedUser> for SearchableStruct {
    fn from(value: NestedUser) -> Self {
        SearchableStruct::NestedUser(Rc::from(value))
    }
}
impl From<NestedUsersUsersRoleInvitation> for SearchableStruct {
    fn from(value: NestedUsersUsersRoleInvitation) -> Self {
        SearchableStruct::NestedUsersUsersRoleInvitation(Rc::from(value))
    }
}
impl From<NestedUsersUsersRoleRequest> for SearchableStruct {
    fn from(value: NestedUsersUsersRoleRequest) -> Self {
        SearchableStruct::NestedUsersUsersRoleRequest(Rc::from(value))
    }
}
impl From<NestedUsersUsersRole> for SearchableStruct {
    fn from(value: NestedUsersUsersRole) -> Self {
        SearchableStruct::NestedUsersUsersRole(Rc::from(value))
    }
}
impl From<Color> for SearchableStruct {
    fn from(value: Color) -> Self {
        SearchableStruct::Color(Rc::from(value))
    }
}
impl From<Country> for SearchableStruct {
    fn from(value: Country) -> Self {
        SearchableStruct::Country(Rc::from(value))
    }
}
impl From<FontAwesomeIcon> for SearchableStruct {
    fn from(value: FontAwesomeIcon) -> Self {
        SearchableStruct::FontAwesomeIcon(Rc::from(value))
    }
}
impl Describable for SearchableStruct {
    fn description(&self) -> Option<&str> {
        match self {
            SearchableStruct::NestedBioOttRank(value) => value.description(),
            SearchableStruct::NestedBioOttTaxonItem(value) => value.description(),
            SearchableStruct::NestedDerivedSample(value) => value.description(),
            SearchableStruct::NestedDocumentFormat(value) => value.description(),
            SearchableStruct::NestedNameplateCategory(value) => value.description(),
            SearchableStruct::NestedNameplate(value) => value.description(),
            SearchableStruct::NestedObservationSubject(value) => value.description(),
            SearchableStruct::NestedObservation(value) => value.description(),
            SearchableStruct::NestedOrganismBioOttTaxonItem(value) => value.description(),
            SearchableStruct::NestedOrganism(value) => value.description(),
            SearchableStruct::NestedOrganization(value) => value.description(),
            SearchableStruct::NestedProjectState(value) => value.description(),
            SearchableStruct::NestedProject(value) => value.description(),
            SearchableStruct::NestedProjectsTeamsRoleInvitation(value) => value.description(),
            SearchableStruct::NestedProjectsTeamsRoleRequest(value) => value.description(),
            SearchableStruct::NestedProjectsTeamsRole(value) => value.description(),
            SearchableStruct::NestedProjectsUsersRoleInvitation(value) => value.description(),
            SearchableStruct::NestedProjectsUsersRoleRequest(value) => value.description(),
            SearchableStruct::NestedProjectsUsersRole(value) => value.description(),
            SearchableStruct::NestedRole(value) => value.description(),
            SearchableStruct::NestedSampleBioOttTaxonItem(value) => value.description(),
            SearchableStruct::NestedSampleContainerCategory(value) => value.description(),
            SearchableStruct::NestedSampleContainer(value) => value.description(),
            SearchableStruct::NestedSampleState(value) => value.description(),
            SearchableStruct::NestedSample(value) => value.description(),
            SearchableStruct::NestedSpectraCollection(value) => value.description(),
            SearchableStruct::NestedTeamState(value) => value.description(),
            SearchableStruct::NestedTeam(value) => value.description(),
            SearchableStruct::NestedTeamsTeamsRoleInvitation(value) => value.description(),
            SearchableStruct::NestedTeamsUsersRoleInvitation(value) => value.description(),
            SearchableStruct::NestedTeamsUsersRoleRequest(value) => value.description(),
            SearchableStruct::NestedTeamsUsersRole(value) => value.description(),
            SearchableStruct::NestedUnit(value) => value.description(),
            SearchableStruct::NestedUser(value) => value.description(),
            SearchableStruct::NestedUsersUsersRoleInvitation(value) => value.description(),
            SearchableStruct::NestedUsersUsersRoleRequest(value) => value.description(),
            SearchableStruct::NestedUsersUsersRole(value) => value.description(),
            SearchableStruct::Color(value) => value.description(),
            SearchableStruct::Country(value) => value.description(),
            SearchableStruct::FontAwesomeIcon(value) => value.description(),
        }
    }
}
impl Colorable for SearchableStruct {
    fn color(&self) -> Option<&str> {
        match self {
            SearchableStruct::NestedBioOttRank(value) => value.color(),
            SearchableStruct::NestedBioOttTaxonItem(value) => value.color(),
            SearchableStruct::NestedDerivedSample(value) => value.color(),
            SearchableStruct::NestedDocumentFormat(value) => value.color(),
            SearchableStruct::NestedNameplateCategory(value) => value.color(),
            SearchableStruct::NestedNameplate(value) => value.color(),
            SearchableStruct::NestedObservationSubject(value) => value.color(),
            SearchableStruct::NestedObservation(value) => value.color(),
            SearchableStruct::NestedOrganismBioOttTaxonItem(value) => value.color(),
            SearchableStruct::NestedOrganism(value) => value.color(),
            SearchableStruct::NestedOrganization(value) => value.color(),
            SearchableStruct::NestedProjectState(value) => value.color(),
            SearchableStruct::NestedProject(value) => value.color(),
            SearchableStruct::NestedProjectsTeamsRoleInvitation(value) => value.color(),
            SearchableStruct::NestedProjectsTeamsRoleRequest(value) => value.color(),
            SearchableStruct::NestedProjectsTeamsRole(value) => value.color(),
            SearchableStruct::NestedProjectsUsersRoleInvitation(value) => value.color(),
            SearchableStruct::NestedProjectsUsersRoleRequest(value) => value.color(),
            SearchableStruct::NestedProjectsUsersRole(value) => value.color(),
            SearchableStruct::NestedRole(value) => value.color(),
            SearchableStruct::NestedSampleBioOttTaxonItem(value) => value.color(),
            SearchableStruct::NestedSampleContainerCategory(value) => value.color(),
            SearchableStruct::NestedSampleContainer(value) => value.color(),
            SearchableStruct::NestedSampleState(value) => value.color(),
            SearchableStruct::NestedSample(value) => value.color(),
            SearchableStruct::NestedSpectraCollection(value) => value.color(),
            SearchableStruct::NestedTeamState(value) => value.color(),
            SearchableStruct::NestedTeam(value) => value.color(),
            SearchableStruct::NestedTeamsTeamsRoleInvitation(value) => value.color(),
            SearchableStruct::NestedTeamsUsersRoleInvitation(value) => value.color(),
            SearchableStruct::NestedTeamsUsersRoleRequest(value) => value.color(),
            SearchableStruct::NestedTeamsUsersRole(value) => value.color(),
            SearchableStruct::NestedUnit(value) => value.color(),
            SearchableStruct::NestedUser(value) => value.color(),
            SearchableStruct::NestedUsersUsersRoleInvitation(value) => value.color(),
            SearchableStruct::NestedUsersUsersRoleRequest(value) => value.color(),
            SearchableStruct::NestedUsersUsersRole(value) => value.color(),
            SearchableStruct::Color(value) => value.color(),
            SearchableStruct::Country(value) => value.color(),
            SearchableStruct::FontAwesomeIcon(value) => value.color(),
        }
    }
}
