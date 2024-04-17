//! Submodule providing select queries, such as `Id` and `Search` queries.

use super::SearcheableTable;
use serde::{Deserialize, Serialize};
use crate::api::ws::messages::FrontendMessage;
use crate::database::Operation;
use crate::database::Task;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Select {
    Id(super::Table, uuid::Uuid),
    SearchTable(SearcheableTable, String),
}

impl Select {
    pub fn authorizations(&self) -> Vec<super::Authorization> {
        // TODO! SPECIFY IT BETTER!
        Vec::new()
    }

    pub fn search(table: SearcheableTable, query: String) -> Self {
        Self::SearchTable(table, query)
    }
}

impl From<Select> for super::Operation {
    fn from(select: Select) -> Self {
        Self::Select(select)
    }
}

impl From<Select> for FrontendMessage {
    fn from(select: Select) -> Self {
        let operation: Operation = select.into();
        let task: Task = operation.into();
        task.into()
    }
}