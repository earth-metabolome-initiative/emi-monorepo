//! Submodule defining the `SameAsIndex` trait which characterizes whether an
//! index can be used to define a same-as relationship.

use sql_traits::traits::{ForeignKeyLike, TableLike, UniqueIndexLike};

/// Trait characterizing whether an index can be used to define a same-as
/// relationship, i.e. it is a unique index over a single column.
pub trait SameAsIndexLike: UniqueIndexLike {
    /// Returns whether the index can be used to define a same-as relationship.
    fn is_same_as(&self, database: &Self::Database, table: &Self::Table) -> bool {
        // Next, we retrieve the columns associated with the index.
        let columns = self.columns(database, table);

        // We expect that all of the columns in the primary key of the table are also in
        // the index.
        let primary_key_columns = table.primary_key_columns(database).collect::<Vec<_>>();

        // If any of the primary key columns are not in the index, it cannot be a
        // same-as index
        if !primary_key_columns.iter().all(|pk_col| columns.contains(pk_col)) {
            return false;
        }

        // There needs to be a foreign key constraint which includes all of the
        // remaining columns in the index which refers to some other table's
        // primary key.
        let mut non_primary_key_columns =
            columns.iter().filter(|col| !primary_key_columns.contains(col)).collect::<Vec<_>>();

        if non_primary_key_columns.is_empty() {
            return true;
        }

        non_primary_key_columns.sort_unstable();

        // We search for a foreign key constraint that includes all of these columns,
        // and that refers to a primary key of another table. If we find any
        // foreign key which satisfies this condition, then we can conclude that
        // the index is a same-as index.

        for foreign_key in table.foreign_keys(database) {
            // If the foreign key does not refer to a foreign's table primary key, it cannot
            // be a same-as index.
            if !foreign_key.is_referenced_primary_key(database) {
                continue;
            }
            // If the foreign key does not include all of the non-primary key columns, it
            // cannot be a same-as index.
            let mut fk_columns = foreign_key.host_columns(database, table).collect::<Vec<_>>();
            fk_columns.sort_unstable();

            if non_primary_key_columns.iter().zip(fk_columns.iter()).all(|(a, b)| a == &b) {
                return true;
            }
        }

        false
    }
}

impl<T> SameAsIndexLike for T where T: UniqueIndexLike {}
