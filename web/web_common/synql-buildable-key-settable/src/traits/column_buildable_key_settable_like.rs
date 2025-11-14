//! Submodule providing the `ColumnBuildableKeySettableLike` trait for SynQL
//! columns.

use sql_relations::traits::{VerticalSameAsColumnLike, VerticalSameAsForeignKeyLike};
use sql_traits::traits::ColumnLike;
use synql_insertable_key_settable::traits::ColumnInsertableKeySettableLike;

/// Trait representing a SynQL column buildable key settable trait.
pub trait ColumnBuildableKeySettableLike: ColumnInsertableKeySettableLike {
    /// Returns whether the column has any check constraints which apply
    /// at the build time.
    fn has_buildable_key_settable_check_constraints(&self, database: &Self::DB) -> bool {
        self.has_insertable_key_settable_check_constraints(database)
            || self.vertical_same_as_foreign_keys(database).any(|fk| {
                let (host, referenced) = fk.vertical_same_as_column_pair(database).unwrap();
                host.has_insertable_key_settable_check_constraints(database)
                    || referenced.has_buildable_key_settable_check_constraints(database)
            })
    }
}

impl<T: ColumnLike + ?Sized> ColumnBuildableKeySettableLike for T {}
