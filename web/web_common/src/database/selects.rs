//! Submodule providing select queries, such as `Id` and `Search` queries.

use serde::{Deserialize, Serialize};

use super::PrimaryKey;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Select {
    Id {
        table_name: String,
        operation_name: Option<String>,
        primary_key: PrimaryKey,
    },
    All {
        table_name: String,
        limit: i64,
        offset: i64,
    },
    AllByUpdatedAt {
        table_name: String,
        limit: i64,
        offset: i64,
    },
    SearchTable {
        table_name: String,
        query: String,
        number_of_results: u32,
    },
}

impl Select {
    pub fn authorizations(&self) -> Vec<super::Authorization> {
        // TODO! SPECIFY IT BETTER!
        Vec::new()
    }

    /// Create a new `Id` query for a given `Table` and `PrimaryKey`.
    ///
    /// # Arguments
    /// * `table` - The table to select from.
    /// * `primary_key` - The primary key to search for.
    pub fn id(table: super::Table, primary_key: PrimaryKey) -> Self {
        Self::Id {
            table_name: table.into(),
            operation_name: None,
            primary_key,
        }
    }

    /// Create a new `Id` query for a given `Table`, `PrimaryKey`, and operation name.
    ///
    /// # Arguments
    /// * `table` - The table to select from.
    /// * `operation_name` - The name of the operation.
    /// * `primary_key` - The primary key to search for.
    pub fn id_with_operation_name(
        table: super::Table,
        operation_name: String,
        primary_key: PrimaryKey,
    ) -> Self {
        Self::Id {
            table_name: table.into(),
            operation_name: Some(operation_name),
            primary_key,
        }
    }

    /// Create a new `Select` query for a given `Table`, query, and number of results.
    ///
    /// # Arguments
    /// * `table` - The table to select from.
    /// * `query` - The query to search for.
    /// * `number_of_results` - The number of results to return.
    pub fn search(table: super::Table, query: String, number_of_results: u32) -> Self {
        Self::SearchTable {
            table_name: table.into(),
            query,
            number_of_results,
        }
    }

    /// Create a new `Select::All` query for a given `Table`.
    ///
    /// # Arguments
    /// * `table` - The table to select from.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    pub fn all(table: super::Table, limit: i64, offset: i64) -> Self {
        Self::All {
            table_name: table.into(),
            limit,
            offset,
        }
    }

    /// Create a new `Select::AllByUpdatedAt` query for a given `Table`.
    /// 
    /// # Arguments
    /// * `table` - The table to select from.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    pub fn all_by_updated_at(table: super::Table, limit: i64, offset: i64) -> Self {
        Self::AllByUpdatedAt {
            table_name: table.into(),
            limit,
            offset,
        }
    }
}

impl From<Select> for super::Operation {
    fn from(select: Select) -> Self {
        Self::Select(select)
    }
}
