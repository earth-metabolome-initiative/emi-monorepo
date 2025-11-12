//! Submodule providing the `TableBuildable` trait for SynQL table buildables.

use synql_builders_metadata::traits::TableBuildableMetadataLike;
use synql_core::structs::Workspace;

use crate::structs::TableBuildable;

/// Name of the module containing the buildable for a table.
pub const BUILDABLE_MODULE_NAME: &str = "buildable";

/// Trait representing a SynQL table buildable.
pub trait TableBuildableLike: TableBuildableMetadataLike {
    /// Returns the [`TableBuildable`](crate::structs::TableBuildable)
    /// representing the buildable for the table.
    ///
    /// # Arguments
    ///
    /// * `workspace` - The workspace where the table is defined.
    /// * `database` - The database connection to use to query the table
    ///   buildable.
    #[inline]
    fn buildable<'table>(
        &'table self,
        workspace: &'table Workspace,
        database: &'table Self::DB,
    ) -> TableBuildable<'table, Self> {
        TableBuildable::new(self, workspace, database)
    }
}

impl<T: TableBuildableMetadataLike> TableBuildableLike for T {}
