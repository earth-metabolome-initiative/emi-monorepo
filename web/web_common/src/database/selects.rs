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
        filter: Option<Vec<u8>>,
        limit: i64,
        offset: i64,
    },
    CanView {
        table_name: String,
        primary_key: PrimaryKey,
    },
    CanUpdate {
        table_name: String,
        primary_key: PrimaryKey,
    },
    CanDelete {
        table_name: String,
        primary_key: PrimaryKey,
    },
    AllByUpdatedAt {
        table_name: String,
        filter: Option<Vec<u8>>,
        limit: i64,
        offset: i64,
    },
    SearchAll {
        query: String,
        limit: i64,
    },
    SearchTable {
        filter: Option<Vec<u8>>,
        table_name: String,
        query: String,
        limit: i64,
        offset: i64,
    },
    SearchEditableTable {
        filter: Option<Vec<u8>>,
        table_name: String,
        query: String,
        limit: i64,
        offset: i64,
    },
}

impl Select {
    /// Check if the query requires authentication.
    pub fn requires_authentication(&self) -> bool {
        match self {
            Self::Id { .. } => false,
            Self::All { .. } => false,
            Self::CanView { .. } => false,
            Self::CanUpdate { .. } => true,
            Self::CanDelete { .. } => true,
            Self::AllByUpdatedAt { .. } => false,
            Self::SearchTable { .. } => false,
            Self::SearchEditableTable { .. } => true,
            Self::SearchAll { .. } => false,
        }
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
    /// * `filter` - The filter to apply to the query.
    /// * `query` - The query to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    pub fn search<F: Serialize>(
        table: super::Table,
        filter: Option<&F>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> Self {
        Self::SearchTable {
            table_name: table.into(),
            filter: filter.map(|f| bincode::serialize(f).unwrap()),
            query,
            limit,
            offset,
        }
    }

    /// Create a new `Select` editable search query for a given `Table`, query, and number of results.
    ///
    /// # Arguments
    /// * `table` - The table to select from.
    /// * `filter` - The filter to apply to the query.
    /// * `query` - The query to search for.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    pub fn search_updatables<F: Serialize>(
        table: super::Table,
        filter: Option<&F>,
        query: String,
        limit: i64,
        offset: i64,
    ) -> Self {
        Self::SearchEditableTable {
            table_name: table.into(),
            filter: filter.map(|f| bincode::serialize(f).unwrap()),
            query,
            limit,
            offset,
        }
    }

    /// Create a new `Select` query for all tables.
    ///
    /// # Arguments
    /// * `query` - The query to search for.
    /// * `limit` - The maximum number of results to return.
    pub fn search_all(query: String, limit: i64) -> Self {
        Self::SearchAll { query, limit }
    }

    /// Create a new `Select::CanView` query for a given `Table` and `PrimaryKey`.
    ///
    /// # Arguments
    /// * `table` - The table to select from.
    /// * `primary_key` - The primary key to search for.
    pub fn can_view(table: super::Table, primary_key: PrimaryKey) -> Self {
        Self::CanView {
            table_name: table.into(),
            primary_key,
        }
    }

    /// Create a new `Select::CanUpdate` query for a given `Table` and `PrimaryKey`.
    ///
    /// # Arguments
    /// * `table` - The table to select from.
    /// * `primary_key` - The primary key to search for.
    pub fn can_update(table: super::Table, primary_key: PrimaryKey) -> Self {
        Self::CanUpdate {
            table_name: table.into(),
            primary_key,
        }
    }

    /// Create a new `Select::CanDelete` query for a given `Table` and `PrimaryKey`.
    ///
    /// # Arguments
    /// * `table` - The table to select from.
    /// * `primary_key` - The primary key to search for.
    pub fn can_admin(table: super::Table, primary_key: PrimaryKey) -> Self {
        Self::CanDelete {
            table_name: table.into(),
            primary_key,
        }
    }

    /// Create a new `Select::All` query for a given `Table`.
    ///
    /// # Arguments
    /// * `table` - The table to select from.
    /// * `filter` - The filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    pub fn all(table: super::Table, filter: Option<Vec<u8>>, limit: i64, offset: i64) -> Self {
        Self::All {
            table_name: table.into(),
            filter,
            limit,
            offset,
        }
    }

    /// Create a new `Select::AllByUpdatedAt` query for a given `Table`.
    ///
    /// # Arguments
    /// * `table` - The table to select from.
    /// * `filter` - The filter to apply to the query.
    /// * `limit` - The maximum number of results to return.
    /// * `offset` - The number of results to skip.
    pub fn all_by_updated_at(
        table: super::Table,
        filter: Option<Vec<u8>>,
        limit: i64,
        offset: i64,
    ) -> Self {
        Self::AllByUpdatedAt {
            table_name: table.into(),
            filter,
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
