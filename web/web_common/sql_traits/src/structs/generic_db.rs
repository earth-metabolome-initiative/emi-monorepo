//! Generic database schema representations and utilities.

mod builder;
mod database;
mod sqlparser;

use std::rc::Rc;

pub use builder::GenericDBBuilder;
pub use sqlparser::ParserDB;

use crate::traits::{FunctionLike, Metadata, TableLike};

#[derive(Debug, Clone)]
/// A generic representation of a database schema.
pub struct GenericDB<T, F>
where
    T: TableLike,
    F: FunctionLike,
{
    /// Catalog name of the database.
    catalog_name: String,
    /// List of tables in the database.
    tables: Vec<(Rc<T>, T::Meta)>,
    /// List of columns in the database.
    columns: Vec<(Rc<T::Column>, <T::Column as Metadata>::Meta)>,
    /// List of unique indices in the database.
    unique_indices: Vec<(Rc<T::UniqueIndex>, <T::UniqueIndex as Metadata>::Meta)>,
    /// List of foreign keys in the database.
    foreign_keys: Vec<(Rc<T::ForeignKey>, <T::ForeignKey as Metadata>::Meta)>,
    /// List of functions crated in the database.
    functions: Vec<(F, <F as Metadata>::Meta)>,
}

impl<T, F> GenericDB<T, F>
where
    T: TableLike,
    F: FunctionLike,
{
    /// Creates a new `GenericDBBuilder` instance.
    pub fn new() -> GenericDBBuilder<T, F> {
        GenericDBBuilder::default()
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
            .binary_search_by(|(f, _)| f.name().cmp(name))
            .ok()
            .map(|index| &self.functions[index].0)
    }

    /// Returns a reference to the metadata of the specified function.
    ///
    /// # Arguments
    ///
    /// * `function` - The function to retrieve metadata for.
    pub fn function_metadata(&self, function: &F) -> &<F as Metadata>::Meta {
        self.functions
            .binary_search_by(|(f, _)| f.name().cmp(function.name()))
            .map(|index| &self.functions[index].1)
            .expect("Function not found in GenericDB")
    }

    /// Returns a reference to the catalog name.
    pub fn catalog_name(&self) -> &str {
        &self.catalog_name
    }
}
