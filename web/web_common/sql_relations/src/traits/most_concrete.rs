//! Submodule defining the `MostConcrete` trait which provides method relative
//! to dispatching the most-concrete table in an inheritance hierarchy.

use sql_traits::traits::{DatabaseLike, TableLike};

/// Trait describing a database characterized by table inheritance.
pub trait InheritableDatabaseLike: DatabaseLike {
    /// Returns the name of the column which indicates the most concrete
    /// table in an inheritance hierarchy.
    fn most_concrete_table_column_name(&self) -> &str;
}

/// Trait characterizing whether a table has the `most_concrete_table` column,
/// i.e. a column indicating the most concrete table in an inheritance
/// hierarchy.
pub trait MostConcreteTableLike:
    TableLike<Database = <Self as MostConcreteTableLike>::InheritableDatabase>
{
    /// Type of the database the table belongs to.
    type InheritableDatabase: InheritableDatabaseLike<Table = Self, Column = Self::Column>;

    /// Returns the column which indicates the most concrete table in an
    /// inheritance hierarchy, if any.
    ///
    /// # Arguments
    ///
    /// * `database` - The database the table belongs to.
    /// * `recursive` - Whether to search ancestor tables for a
    ///   `most_concrete_table` column.
    ///
    /// # Errors
    ///
    /// * If the column cannot be loaded from the database.
    fn most_concrete_column<'db>(
        &'db self,
        database: &'db Self::InheritableDatabase,
        recursive: bool,
    ) -> Option<&'db Self::Column>
    where
        Self: 'db,
    {
        if let Some(column) =
            self.column_by_name(database.most_concrete_table_column_name(), database)
        {
            return Some(column);
        };
        if recursive {
            for parent in self.extended_tables(database) {
                if let Some(col) = parent.most_concrete_column(database, true) {
                    return Some(col);
                }
            }
        }
        None
    }
}

impl<T> MostConcreteTableLike for T
where
    T: TableLike,
    T::Database: InheritableDatabaseLike<Table = T, Column = T::Column>,
{
    type InheritableDatabase = T::Database;
}
