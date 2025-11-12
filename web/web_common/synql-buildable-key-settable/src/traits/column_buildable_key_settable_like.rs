//! Submodule providing the `ColumnBuildableKeySettableLike` trait for SynQL
//! columns.

use sql_relations::traits::{HorizontalSameAsColumnLike, VerticalSameAsColumnLike};
use sql_traits::traits::{ColumnLike, ForeignKeyLike};
use synql_insertable_key_settable::traits::ColumnInsertableKeySettableLike;

/// Trait representing a SynQL column buildable key settable trait.
pub trait ColumnBuildableKeySettableLike: ColumnInsertableKeySettableLike {
    /// Returns whether the column has any check constraints which apply
    /// at the build time.
    fn has_buildable_key_settable_check_constraints(&self, database: &Self::DB) -> bool {
        self.vertical_same_as_foreign_keys(database)
            .chain(self.horizontal_same_as_foreign_keys(database))
            .any(|fk| {
                fk.host_columns(database)
                    .any(|col| col.has_insertable_key_settable_check_constraints(database))
            })
    }
}

impl<T: ColumnLike + ?Sized> ColumnBuildableKeySettableLike for T {}
