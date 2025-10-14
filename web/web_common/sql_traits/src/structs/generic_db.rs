//! Generic database schema representations and utilities.

mod database;
mod sqlparser;

use std::rc::Rc;

pub use sqlparser::ParserDB;

use crate::traits::{FunctionLike, Metadata, TableLike};

#[derive(Debug, Clone)]
/// A generic representation of a database schema.
pub struct GenericDB<T, F>
where
    T: TableLike,
    F: FunctionLike,
{
    /// List of tables in the database.
    tables: Vec<(Rc<T>, T::Meta)>,
    /// List of columns in the database.
    columns: Vec<(Rc<T::Column>, <T::Column as Metadata>::Meta)>,
    /// List of unique indices in the database.
    unique_indices: Vec<(Rc<T::UniqueIndex>, <T::UniqueIndex as Metadata>::Meta)>,
    /// List of foreign keys in the database.
    foreign_keys: Vec<(Rc<T::ForeignKey>, <T::ForeignKey as Metadata>::Meta)>,
    /// List of functions crated in the database.
    functions: Vec<F>,
}

impl<T, F> GenericDB<T, F>
where
    T: TableLike,
    F: FunctionLike,
{
    /// Creates a new `GenericDB` instance.
    ///
    /// # Arguments
    ///
    /// * `tables` - A vector of tuples containing a reference-counted table and
    ///   its metadata.
    /// * `columns` - A vector of tuples containing a reference-counted column
    ///   and its metadata.
    /// * `unique_indices` - A vector of tuples containing a unique index and
    ///   its metadata.
    /// * `foreign_keys` - A vector of tuples containing a foreign key and its
    ///   metadata.
    /// * `functions` - A vector of functions created in the database.
    pub fn new(
        mut tables: Vec<(Rc<T>, T::Meta)>,
        mut columns: Vec<(Rc<T::Column>, <T::Column as Metadata>::Meta)>,
        mut unique_indices: Vec<(Rc<T::UniqueIndex>, <T::UniqueIndex as Metadata>::Meta)>,
        mut foreign_keys: Vec<(Rc<T::ForeignKey>, <T::ForeignKey as Metadata>::Meta)>,
        mut functions: Vec<F>,
    ) -> Self {
        tables.sort_unstable_by_key(|(table, _)| {
            (table.table_schema().map(|s| s.to_string()), table.table_name().to_string())
        });

        columns.sort_unstable_by(|(a, _), (b, _)| a.as_ref().cmp(b.as_ref()));
        unique_indices.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
        foreign_keys.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
        functions.sort_unstable_by(|a, b| a.name().cmp(b.name()));

        Self { tables, columns, unique_indices, foreign_keys, functions }
    }

    /// Returns a reference to the metadata of the specified table.
    pub fn table_metadata(&self, table: &T) -> &T::Meta {
        self.tables
            .binary_search_by_key(
                &(table.table_schema().map(|s| s.to_string()), table.table_name().to_string()),
                |(t, _)| (t.table_schema().map(|s| s.to_string()), t.table_name().to_string()),
            )
            .map(|index| &self.tables[index].1)
            .expect("Table not found in GenericDB")
    }

    /// Returns a reference to the metadata of the specified column.
    pub fn column_metadata(&self, column: &T::Column) -> &<T::Column as Metadata>::Meta {
        self.columns
            .binary_search_by(|(c, _)| c.as_ref().cmp(column))
            .map(|index| &self.columns[index].1)
            .expect("Column not found in GenericDB")
    }

    /// Returns a reference to the metadata of the specified unique index.
    pub fn index_metadata(&self, index: &T::UniqueIndex) -> &<T::UniqueIndex as Metadata>::Meta {
        self.unique_indices
            .binary_search_by(|(i, _)| i.as_ref().cmp(index))
            .map(|index| &self.unique_indices[index].1)
            .expect("Index not found in GenericDB")
    }

    /// Returns a reference to the metadata of the specified foreign key.
    pub fn foreign_key_metadata(&self, key: &T::ForeignKey) -> &<T::ForeignKey as Metadata>::Meta {
        self.foreign_keys
            .binary_search_by(|(k, _)| k.as_ref().cmp(key))
            .map(|index| &self.foreign_keys[index].1)
            .expect("Foreign key not found in GenericDB")
    }

    /// Returns a reference of the function by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function to retrieve.
    pub fn function(&self, name: &str) -> Option<&F> {
        self.functions
            .binary_search_by(|f| f.name().cmp(name))
            .ok()
            .map(|index| &self.functions[index])
    }
}
