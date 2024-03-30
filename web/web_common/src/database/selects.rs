//! Submodule providing select queries, such as `Id` and `Search` queries.

use serde::{Deserialize, Serialize};
use super::SearcheableTable;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Select {
    Id(super::Table, uuid::Uuid),
    SearchTable(SearcheableTable, String),
}

impl Select {
    pub fn authorizations(&self) -> Vec<super::Authorization> {
        todo!("Implement authorizations for select queries");
    }
}