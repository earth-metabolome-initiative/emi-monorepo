//! Generic database schema representations and utilities.

mod builder;
mod database;
mod sqlparser;

use std::{fmt::Debug, rc::Rc};

pub use builder::GenericDBBuilder;
pub use sqlparser::ParserDB;

use crate::traits::{
    CheckConstraintLike, ColumnLike, ForeignKeyLike, FunctionLike, TableLike, UniqueIndexLike,
};

/// A generic representation of a database schema.
pub struct GenericDB<T, C, U, F, Func, Ch>
where
    T: TableLike,
    C: ColumnLike,
    U: UniqueIndexLike,
    F: ForeignKeyLike,
    Func: FunctionLike,
    Ch: CheckConstraintLike,
{
    /// Catalog name of the database.
    catalog_name: String,
    /// Timezone of the database.
    timezone: Option<String>,
    /// List of tables in the database.
    tables: Vec<(Rc<T>, T::Meta)>,
    /// List of columns in the database.
    columns: Vec<(Rc<C>, C::Meta)>,
    /// List of unique indices in the database.
    unique_indices: Vec<(Rc<U>, U::Meta)>,
    /// List of foreign keys in the database.
    foreign_keys: Vec<(Rc<F>, F::Meta)>,
    /// List of functions created in the database.
    functions: Vec<(Rc<Func>, Func::Meta)>,
    /// Phantom data for check constraints.
    check_constraints: Vec<(Rc<Ch>, Ch::Meta)>,
}

impl<T, C, U, F, Func, Ch> Debug for GenericDB<T, C, U, F, Func, Ch>
where
    T: TableLike,
    C: ColumnLike,
    U: UniqueIndexLike,
    F: ForeignKeyLike,
    Func: FunctionLike,
    Ch: CheckConstraintLike,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenericDB")
            .field("catalog_name", &self.catalog_name)
            .field("tables", &self.tables.len())
            .field("columns", &self.columns.len())
            .field("unique_indices", &self.unique_indices.len())
            .field("foreign_keys", &self.foreign_keys.len())
            .field("functions", &self.functions.len())
            .finish()
    }
}

impl<T, C, U, F, Func, Ch> Clone for GenericDB<T, C, U, F, Func, Ch>
where
    T: TableLike,
    C: ColumnLike,
    U: UniqueIndexLike,
    F: ForeignKeyLike,
    Func: FunctionLike,
    Ch: CheckConstraintLike,
{
    fn clone(&self) -> Self {
        Self {
            catalog_name: self.catalog_name.clone(),
            timezone: self.timezone.clone(),
            tables: self.tables.clone(),
            columns: self.columns.clone(),
            unique_indices: self.unique_indices.clone(),
            foreign_keys: self.foreign_keys.clone(),
            functions: self.functions.clone(),
            check_constraints: self.check_constraints.clone(),
        }
    }
}

impl<T, C, U, F, Func, Ch> GenericDB<T, C, U, F, Func, Ch>
where
    T: TableLike,
    C: ColumnLike,
    U: UniqueIndexLike,
    F: ForeignKeyLike,
    Func: FunctionLike,
    Ch: CheckConstraintLike,
{
    /// Creates a new `GenericDBBuilder` instance.
    pub fn new() -> GenericDBBuilder<T, C, U, F, Func, Ch> {
        GenericDBBuilder::default()
    }

    /// Returns a reference to the metadata of the specified table.
    pub fn table_metadata(&self, table: &T) -> &T::Meta {
        self.tables
            .binary_search_by_key(
                &(
                    table.table_schema().map(std::string::ToString::to_string),
                    table.table_name().to_string(),
                ),
                |(t, _)| {
                    (
                        t.table_schema().map(std::string::ToString::to_string),
                        t.table_name().to_string(),
                    )
                },
            )
            .map(|index| &self.tables[index].1)
            .expect("Table not found in GenericDB")
    }

    /// Returns a reference to the metadata of the specified column.
    pub fn column_metadata(&self, column: &C) -> &C::Meta {
        self.columns
            .binary_search_by(|(c, _)| c.as_ref().cmp(column))
            .map(|index| &self.columns[index].1)
            .expect("Column not found in GenericDB")
    }

    /// Returns a reference to the metadata of the specified unique index.
    pub fn index_metadata(&self, index: &U) -> &U::Meta {
        self.unique_indices
            .binary_search_by(|(i, _)| i.as_ref().cmp(index))
            .map(|index| &self.unique_indices[index].1)
            .expect("Index not found in GenericDB")
    }

    /// Returns a reference to the metadata of the specified check constraint.
    pub fn check_constraint_metadata(&self, constraint: &Ch) -> &Ch::Meta {
        self.check_constraints
            .binary_search_by(|(c, _)| c.as_ref().cmp(constraint))
            .map(|index| &self.check_constraints[index].1)
            .expect("Check constraint not found in GenericDB")
    }

    /// Returns a reference to the metadata of the specified foreign key.
    pub fn foreign_key_metadata(&self, key: &F) -> &F::Meta {
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
    #[must_use]
    pub fn function(&self, name: &str) -> Option<&Func> {
        self.functions
            .binary_search_by(|(f, _)| f.name().cmp(name))
            .ok()
            .map(|index| self.functions[index].0.as_ref())
    }

    /// Returns a reference to the metadata of the specified function.
    ///
    /// # Arguments
    ///
    /// * `function` - The function to retrieve metadata for.
    pub fn function_metadata(&self, function: &Func) -> &Func::Meta {
        self.functions
            .binary_search_by(|(f, _)| f.name().cmp(function.name()))
            .map(|index| &self.functions[index].1)
            .expect("Function not found in GenericDB")
    }

    /// Returns a reference to the catalog name.
    #[inline]
    pub fn catalog_name(&self) -> &str {
        &self.catalog_name
    }
}
