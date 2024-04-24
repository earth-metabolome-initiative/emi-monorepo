//! This module contains the table names enumeration.
//!
//! This module is automatically generated. Do not write anything here.

use crate::database::*;
pub trait Searchable {
    fn search_task(query: String, limit: u32) -> super::Select;
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
impl Searchable for NestedSamplingProcedure {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::SamplingProcedures,
              query,
              limit,
        )
    }
}
impl Searchable for NestedTaxa {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Taxa,
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
impl Searchable for Classe {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Classes,
              query,
              limit,
        )
    }
}
impl Searchable for Kingdom {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Kingdoms,
              query,
              limit,
        )
    }
}
impl Searchable for OrganismDomain {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::OrganismDomains,
              query,
              limit,
        )
    }
}
impl Searchable for Phylum {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Phylums,
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
impl Searchable for Taxa {
    fn search_task(query: String, limit: u32) -> super::Select {
        super::Select::search(
             Table::Taxa,
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