//! Submodule providing the `ColumnInsertableKeySettableLike` trait for SynQL
//! columns.

use sql_traits::traits::{CheckConstraintLike, ColumnLike, DatabaseLike};

/// Trait representing a SynQL column insertable key settable trait.
pub trait ColumnInsertableKeySettableLike: ColumnLike {
    /// Returns an iterator over the check constraints which apply
    /// at the insertion time for the column.
    fn insertable_key_settable_check_constraints<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::CheckConstraint> + 'db {
        self.check_constraints(database).filter(move |check_constraint| {
            !check_constraint.is_mutual_nullability_constraint(database)
                && !check_constraint.columns(database).any(|column| column.is_generated())
        })
    }

    /// Returns whether the column has any check constraints which apply
    /// at the insertion time.
    fn has_insertable_key_settable_check_constraints(&self, database: &Self::DB) -> bool {
        self.insertable_key_settable_check_constraints(database).next().is_some()
    }
}

impl<T: ColumnLike + ?Sized> ColumnInsertableKeySettableLike for T {}
