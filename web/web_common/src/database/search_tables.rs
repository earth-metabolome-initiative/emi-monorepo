//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use crate::database::*;
pub trait Searchable<const EDIT: bool> {
    fn search_task(query: String, limit: u32) -> super::Select;
}
impl Searchable<false> for NestedBioOttRank {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::BioOttRanks,
              query,
              limit,
        )
    }
}
impl Searchable<false> for NestedBioOttTaxonItem {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::BioOttTaxonItems,
              query,
              limit,
        )
    }
}
impl Searchable<false> for NestedDocumentFormat {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::DocumentFormats,
              query,
              limit,
        )
    }
}
impl Searchable<false> for NestedOrganization {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Organizations,
              query,
              limit,
        )
    }
}
impl Searchable<false> for NestedProjectState {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::ProjectStates,
              query,
              limit,
        )
    }
}
impl Searchable<false> for NestedProject {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Projects,
              query,
              limit,
        )
    }
}
impl Searchable<true> for NestedProject {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search_editables(
             Table::Projects,
              query,
              limit,
        )
    }
}
impl Searchable<false> for NestedRole {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Roles,
              query,
              limit,
        )
    }
}
impl Searchable<false> for NestedSampleState {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::SampleStates,
              query,
              limit,
        )
    }
}
impl Searchable<true> for NestedSampledIndividual {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search_editables(
             Table::SampledIndividuals,
              query,
              limit,
        )
    }
}
impl Searchable<true> for NestedSample {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search_editables(
             Table::Samples,
              query,
              limit,
        )
    }
}
impl Searchable<true> for NestedSpectraCollection {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search_editables(
             Table::SpectraCollections,
              query,
              limit,
        )
    }
}
impl Searchable<false> for NestedTeamState {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::TeamStates,
              query,
              limit,
        )
    }
}
impl Searchable<false> for NestedTeam {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Teams,
              query,
              limit,
        )
    }
}
impl Searchable<true> for NestedTeam {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search_editables(
             Table::Teams,
              query,
              limit,
        )
    }
}
impl Searchable<false> for BioOttRank {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::BioOttRanks,
              query,
              limit,
        )
    }
}
impl Searchable<false> for BioOttTaxonItem {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::BioOttTaxonItems,
              query,
              limit,
        )
    }
}
impl Searchable<false> for Color {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Colors,
              query,
              limit,
        )
    }
}
impl Searchable<false> for Country {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Countries,
              query,
              limit,
        )
    }
}
impl Searchable<false> for DocumentFormat {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::DocumentFormats,
              query,
              limit,
        )
    }
}
impl Searchable<false> for FontAwesomeIcon {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::FontAwesomeIcons,
              query,
              limit,
        )
    }
}
impl Searchable<false> for Organization {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Organizations,
              query,
              limit,
        )
    }
}
impl Searchable<false> for ProjectState {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::ProjectStates,
              query,
              limit,
        )
    }
}
impl Searchable<false> for Project {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Projects,
              query,
              limit,
        )
    }
}
impl Searchable<true> for Project {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search_editables(
             Table::Projects,
              query,
              limit,
        )
    }
}
impl Searchable<false> for Role {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Roles,
              query,
              limit,
        )
    }
}
impl Searchable<false> for SampleState {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::SampleStates,
              query,
              limit,
        )
    }
}
impl Searchable<true> for SampledIndividual {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search_editables(
             Table::SampledIndividuals,
              query,
              limit,
        )
    }
}
impl Searchable<true> for Sample {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search_editables(
             Table::Samples,
              query,
              limit,
        )
    }
}
impl Searchable<true> for SpectraCollection {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search_editables(
             Table::SpectraCollections,
              query,
              limit,
        )
    }
}
impl Searchable<false> for TeamState {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::TeamStates,
              query,
              limit,
        )
    }
}
impl Searchable<false> for Team {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Teams,
              query,
              limit,
        )
    }
}
impl Searchable<true> for Team {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search_editables(
             Table::Teams,
              query,
              limit,
        )
    }
}
impl Searchable<false> for Unit {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Units,
              query,
              limit,
        )
    }
}
impl Searchable<false> for User {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Users,
              query,
              limit,
        )
    }
}
