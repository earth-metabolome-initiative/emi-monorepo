//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use crate::database::*;
pub trait Searchable {
    fn search_task(query: String, limit: u32) -> super::Select;
}
impl Searchable for NestedBioOttRank {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::BioOttRanks,
              query,
              limit,
        )
    }
}
impl Searchable for NestedBioOttTaxonItem {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::BioOttTaxonItems,
              query,
              limit,
        )
    }
}
impl Searchable for NestedItemCategory {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::ItemCategories,
              query,
              limit,
        )
    }
}
impl Searchable for NestedOrganization {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Organizations,
              query,
              limit,
        )
    }
}
impl Searchable for NestedProjectState {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::ProjectStates,
              query,
              limit,
        )
    }
}
impl Searchable for NestedProject {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Projects,
              query,
              limit,
        )
    }
}
impl Searchable for NestedSampleState {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::SampleStates,
              query,
              limit,
        )
    }
}
impl Searchable for NestedSamplingProcedure {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::SamplingProcedures,
              query,
              limit,
        )
    }
}
impl Searchable for NestedTeamState {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::TeamStates,
              query,
              limit,
        )
    }
}
impl Searchable for NestedTeam {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Teams,
              query,
              limit,
        )
    }
}
impl Searchable for NestedPublicUser {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::PublicUsers,
              query,
              limit,
        )
    }
}
impl Searchable for BioOttRank {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::BioOttRanks,
              query,
              limit,
        )
    }
}
impl Searchable for BioOttTaxonItem {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::BioOttTaxonItems,
              query,
              limit,
        )
    }
}
impl Searchable for Color {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Colors,
              query,
              limit,
        )
    }
}
impl Searchable for DocumentFormat {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::DocumentFormats,
              query,
              limit,
        )
    }
}
impl Searchable for FontAwesomeIcon {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::FontAwesomeIcons,
              query,
              limit,
        )
    }
}
impl Searchable for ItemCategory {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::ItemCategories,
              query,
              limit,
        )
    }
}
impl Searchable for Organization {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Organizations,
              query,
              limit,
        )
    }
}
impl Searchable for ProjectState {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::ProjectStates,
              query,
              limit,
        )
    }
}
impl Searchable for Project {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Projects,
              query,
              limit,
        )
    }
}
impl Searchable for SampleState {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::SampleStates,
              query,
              limit,
        )
    }
}
impl Searchable for SamplingProcedure {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::SamplingProcedures,
              query,
              limit,
        )
    }
}
impl Searchable for TeamState {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::TeamStates,
              query,
              limit,
        )
    }
}
impl Searchable for Team {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Teams,
              query,
              limit,
        )
    }
}
impl Searchable for Unit {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Units,
              query,
              limit,
        )
    }
}
impl Searchable for User {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Users,
              query,
              limit,
        )
    }
}
impl Searchable for PublicUser {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::PublicUsers,
              query,
              limit,
        )
    }
}
