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
            .field("timezone", &self.timezone)
            .field("tables", &self.tables.len())
            .field("columns", &self.columns.len())
            .field("unique_indices", &self.unique_indices.len())
            .field("foreign_keys", &self.foreign_keys.len())
            .field("functions", &self.functions.len())
            .field("check_constraints", &self.check_constraints.len())
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
    #[must_use]
    pub fn new(catalog_name: String) -> GenericDBBuilder<T, C, U, F, Func, Ch> {
        GenericDBBuilder::new(catalog_name)
    }

    /// Returns a reference to the metadata of the specified table, if it exists
    /// in the database.
    pub fn table_metadata(&self, table: &T) -> Option<&T::Meta> {
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
            .ok()
            .map(|index| &self.tables[index].1)
    }

    /// Returns a reference to the metadata of the specified column, if it
    /// exists in the database.
    pub fn column_metadata(&self, column: &C) -> Option<&C::Meta> {
        self.columns
            .binary_search_by(|(c, _)| c.as_ref().cmp(column))
            .ok()
            .map(|index| &self.columns[index].1)
    }

    /// Returns a reference to the metadata of the specified unique index, if it
    /// exists in the database.
    pub fn index_metadata(&self, index: &U) -> Option<&U::Meta> {
        self.unique_indices
            .binary_search_by(|(i, _)| i.as_ref().cmp(index))
            .ok()
            .map(|index| &self.unique_indices[index].1)
    }

    /// Returns a reference to the metadata of the specified check constraint,
    /// if it exists in the database.
    pub fn check_constraint_metadata(&self, constraint: &Ch) -> Option<&Ch::Meta> {
        self.check_constraints
            .binary_search_by(|(c, _)| c.as_ref().cmp(constraint))
            .ok()
            .map(|index| &self.check_constraints[index].1)
    }

    /// Returns a reference to the metadata of the specified foreign key, if it
    /// exists in the database.
    pub fn foreign_key_metadata(&self, key: &F) -> Option<&F::Meta> {
        self.foreign_keys
            .binary_search_by(|(k, _)| k.as_ref().cmp(key))
            .ok()
            .map(|index| &self.foreign_keys[index].1)
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

    /// Returns a reference to the metadata of the specified function, if it
    /// exists in the database.
    ///
    /// # Arguments
    ///
    /// * `function` - The function to retrieve metadata for.
    pub fn function_metadata(&self, function: &Func) -> Option<&Func::Meta> {
        self.functions
            .binary_search_by(|(f, _)| f.name().cmp(function.name()))
            .ok()
            .map(|index| &self.functions[index].1)
    }

    /// Returns a reference to the catalog name.
    #[must_use]
    #[inline]
    pub fn catalog_name(&self) -> &str {
        &self.catalog_name
    }
}
