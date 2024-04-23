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
