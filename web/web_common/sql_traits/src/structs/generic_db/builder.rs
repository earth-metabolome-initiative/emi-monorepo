//! Builder for constructing a `GenericDB` instance.

use std::{fmt::Display, rc::Rc};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::{
    structs::GenericDB,
    traits::{FunctionLike, Metadata, TableLike},
};

/// Builder for constructing a `GenericDB` instance.
pub struct GenericDBBuilder<T, F>
where
    T: TableLike,
    F: FunctionLike,
{
    /// Catalog name of the database.
    catalog_name: Option<String>,
    /// List of tables in the database.
    tables: Vec<(Rc<T>, T::Meta)>,
    /// List of columns in the database.
    columns: Vec<(Rc<T::Column>, <T::Column as Metadata>::Meta)>,
    /// List of unique indices in the database.
    unique_indices: Vec<(Rc<T::UniqueIndex>, <T::UniqueIndex as Metadata>::Meta)>,
    /// List of foreign keys in the database.
    foreign_keys: Vec<(Rc<T::ForeignKey>, <T::ForeignKey as Metadata>::Meta)>,
    /// List of functions created in the database.
    functions: Vec<(F, <F as Metadata>::Meta)>,
}

impl<T, F> Default for GenericDBBuilder<T, F>
where
    T: TableLike,
    F: FunctionLike,
{
    fn default() -> Self {
        Self {
            catalog_name: None,
            tables: Vec::new(),
            columns: Vec::new(),
            unique_indices: Vec::new(),
            foreign_keys: Vec::new(),
            functions: Vec::new(),
        }
    }
}

/// Attributes that can be set on the `GenericDBBuilder`.
#[derive(Debug)]
pub enum GenericDBAttribute {
    /// The catalog (database) name.
    CatalogName,
}

impl Display for GenericDBAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenericDBAttribute::CatalogName => write!(f, "catalog_name"),
        }
    }
}

impl<T, F> GenericDBBuilder<T, F>
where
    T: TableLike,
    F: FunctionLike,
{
    /// Creates a new `GenericDBBuilder` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the catalog name for the database.
    pub fn catalog_name(mut self, catalog_name: String) -> Self {
        self.catalog_name = Some(catalog_name);
        self
    }

    /// Adds a table with its metadata to the builder.
    pub fn add_table(mut self, table: Rc<T>, metadata: T::Meta) -> Self {
        self.tables.push((table, metadata));
        self
    }

    /// Adds multiple tables with their metadata to the builder.
    pub fn add_tables(mut self, tables: impl IntoIterator<Item = (Rc<T>, T::Meta)>) -> Self {
        self.tables.extend(tables);
        self
    }

    /// Adds a column with its metadata to the builder.
    pub fn add_column(
        mut self,
        column: Rc<T::Column>,
        metadata: <T::Column as Metadata>::Meta,
    ) -> Self {
        self.columns.push((column, metadata));
        self
    }

    /// Adds multiple columns with their metadata to the builder.
    pub fn add_columns(
        mut self,
        columns: impl IntoIterator<Item = (Rc<T::Column>, <T::Column as Metadata>::Meta)>,
    ) -> Self {
        self.columns.extend(columns);
        self
    }

    /// Adds a unique index with its metadata to the builder.
    pub fn add_unique_index(
        mut self,
        index: Rc<T::UniqueIndex>,
        metadata: <T::UniqueIndex as Metadata>::Meta,
    ) -> Self {
        self.unique_indices.push((index, metadata));
        self
    }

    /// Adds multiple unique indices with their metadata to the builder.
    pub fn add_unique_indices(
        mut self,
        indices: impl IntoIterator<Item = (Rc<T::UniqueIndex>, <T::UniqueIndex as Metadata>::Meta)>,
    ) -> Self {
        self.unique_indices.extend(indices);
        self
    }

    /// Adds a foreign key with its metadata to the builder.
    pub fn add_foreign_key(
        mut self,
        key: Rc<T::ForeignKey>,
        metadata: <T::ForeignKey as Metadata>::Meta,
    ) -> Self {
        self.foreign_keys.push((key, metadata));
        self
    }

    /// Adds multiple foreign keys with their metadata to the builder.
    pub fn add_foreign_keys(
        mut self,
        keys: impl IntoIterator<Item = (Rc<T::ForeignKey>, <T::ForeignKey as Metadata>::Meta)>,
    ) -> Self {
        self.foreign_keys.extend(keys);
        self
    }

    /// Adds a function with its metadata to the builder.
    pub fn add_function(mut self, function: F, metadata: <F as Metadata>::Meta) -> Self {
        self.functions.push((function, metadata));
        self
    }

    /// Adds multiple functions with their metadata to the builder.
    pub fn add_functions(
        mut self,
        functions: impl IntoIterator<Item = (F, <F as Metadata>::Meta)>,
    ) -> Self {
        self.functions.extend(functions);
        self
    }
}

impl<T, F> Attributed for GenericDBBuilder<T, F>
where
    T: TableLike,
    F: FunctionLike,
{
    type Attribute = GenericDBAttribute;
}

impl<T, F> IsCompleteBuilder for GenericDBBuilder<T, F>
where
    T: TableLike,
    F: FunctionLike,
{
    fn is_complete(&self) -> bool {
        self.catalog_name.is_some()
    }
}

impl<T, F> Builder for GenericDBBuilder<T, F>
where
    T: TableLike,
    F: FunctionLike,
{
    type Error = BuilderError<GenericDBAttribute>;
    type Object = GenericDB<T, F>;

    fn build(mut self) -> Result<Self::Object, Self::Error> {
        let catalog_name = self
            .catalog_name
            .ok_or(BuilderError::IncompleteBuild(GenericDBAttribute::CatalogName))?;

        self.tables.sort_unstable_by_key(|(table, _)| {
            (table.table_schema().map(|s| s.to_string()), table.table_name().to_string())
        });

        self.columns.sort_unstable_by(|(a, _), (b, _)| a.as_ref().cmp(b.as_ref()));
        self.unique_indices.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
        self.foreign_keys.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
        self.functions.sort_unstable_by(|(a, _), (b, _)| a.name().cmp(b.name()));

        Ok(GenericDB {
            catalog_name,
            tables: self.tables,
            columns: self.columns,
            unique_indices: self.unique_indices,
            foreign_keys: self.foreign_keys,
            functions: self.functions,
        })
    }
}
