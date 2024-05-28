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
impl Searchable<false> for NestedLoginProvider {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::LoginProviders,
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
impl Searchable<false> for NestedSampledIndividual {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::SampledIndividuals,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<true> for NestedSampledIndividual {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search_updatables(
             Table::SampledIndividuals,
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
impl Searchable<false> for LoginProvider {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::LoginProviders,
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
impl Searchable<false> for SampledIndividual {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search(
             Table::SampledIndividuals,
              filter,
              query,
              limit,
              offset,
        )
    }
}
impl Searchable<true> for SampledIndividual {
    fn search_task(filter: Option<&Self::Filter>, query: String, limit: i64, offset: i64) -> super::Select {
        super::Select::search_updatables(
             Table::SampledIndividuals,
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
