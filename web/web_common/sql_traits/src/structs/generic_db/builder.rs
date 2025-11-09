//! Builder for constructing a `GenericDB` instance.

use std::{fmt::Display, rc::Rc};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};

use crate::{
    structs::GenericDB,
    traits::{
        CheckConstraintLike, ColumnLike, ForeignKeyLike, FunctionLike, TableLike, UniqueIndexLike,
    },
};

/// Builder for constructing a `GenericDB` instance.
pub struct GenericDBBuilder<T, C, U, F, Func, Ch>
where
    T: TableLike,
    C: ColumnLike,
    U: UniqueIndexLike,
    F: ForeignKeyLike,
    Func: FunctionLike,
    Ch: CheckConstraintLike,
{
    /// Catalog name of the database.
    catalog_name: Option<String>,
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

impl<T, C, U, F, Func, Ch> Default for GenericDBBuilder<T, C, U, F, Func, Ch>
where
    T: TableLike,
    C: ColumnLike,
    U: UniqueIndexLike,
    F: ForeignKeyLike,
    Func: FunctionLike,
    Ch: CheckConstraintLike,
{
    fn default() -> Self {
        Self {
            catalog_name: None,
            timezone: None,
            tables: Vec::new(),
            columns: Vec::new(),
            unique_indices: Vec::new(),
            foreign_keys: Vec::new(),
            functions: Vec::new(),
            check_constraints: Vec::new(),
        }
    }
}

/// Attributes that can be set on the `GenericDBBuilder`.
#[derive(Debug)]
pub enum GenericDBAttribute {
    /// The catalog (database) name.
    CatalogName,
    /// The timezone of the database.
    Timezone,
}

impl Display for GenericDBAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenericDBAttribute::CatalogName => write!(f, "catalog_name"),
            GenericDBAttribute::Timezone => write!(f, "timezone"),
        }
    }
}

impl<T, C, U, F, Func, Ch> GenericDBBuilder<T, C, U, F, Func, Ch>
where
    T: TableLike,
    C: ColumnLike,
    U: UniqueIndexLike,
    F: ForeignKeyLike,
    Func: FunctionLike,
    Ch: CheckConstraintLike,
{
    /// Creates a new `GenericDBBuilder` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the timezone for the database.
    pub fn timezone(mut self, timezone: String) -> Self {
        self.timezone = Some(timezone);
        self
    }

    /// Sets the catalog name for the database.
    pub fn catalog_name(mut self, catalog_name: String) -> Self {
        self.catalog_name = Some(catalog_name);
        self
    }

    /// Adds a table with its metadata to the builder.
    pub fn add_table(mut self, table: Rc<T>, metadata: T::Meta) -> Self {
        assert!(
            self.tables.iter().all(|(existing_table, _)| existing_table.as_ref() != table.as_ref()),
            "Table '{}' already exists in the database '{}'",
            table.table_name(),
            self.catalog_name.as_deref().unwrap_or("<unnamed>")
        );

        self.tables.push((table, metadata));
        self
    }

    /// Adds multiple tables with their metadata to the builder.
    pub fn add_tables(mut self, tables: impl IntoIterator<Item = (Rc<T>, T::Meta)>) -> Self {
        self.tables.extend(tables);
        self
    }

    /// Adds a column with its metadata to the builder.
    pub fn add_column(mut self, column: Rc<C>, metadata: C::Meta) -> Self {
        self.columns.push((column, metadata));
        self
    }

    /// Adds multiple columns with their metadata to the builder.
    pub fn add_columns(mut self, columns: impl IntoIterator<Item = (Rc<C>, C::Meta)>) -> Self {
        self.columns.extend(columns);
        self
    }

    /// Adds a unique index with its metadata to the builder.
    pub fn add_unique_index(mut self, index: Rc<U>, metadata: U::Meta) -> Self {
        self.unique_indices.push((index, metadata));
        self
    }

    /// Adds multiple unique indices with their metadata to the builder.
    pub fn add_unique_indices(
        mut self,
        indices: impl IntoIterator<Item = (Rc<U>, U::Meta)>,
    ) -> Self {
        self.unique_indices.extend(indices);
        self
    }

    /// Adds a foreign key with its metadata to the builder.
    pub fn add_foreign_key(mut self, key: Rc<F>, metadata: F::Meta) -> Self {
        self.foreign_keys.push((key, metadata));
        self
    }

    /// Adds multiple foreign keys with their metadata to the builder.
    pub fn add_foreign_keys(mut self, keys: impl IntoIterator<Item = (Rc<F>, F::Meta)>) -> Self {
        self.foreign_keys.extend(keys);
        self
    }

    /// Adds a function with its metadata to the builder.
    pub fn add_function(mut self, function: Rc<Func>, metadata: Func::Meta) -> Self {
        self.functions.push((function, metadata));
        self
    }

    /// Adds multiple functions with their metadata to the builder.
    pub fn add_functions(
        mut self,
        functions: impl IntoIterator<Item = (Rc<Func>, Func::Meta)>,
    ) -> Self {
        self.functions.extend(functions);
        self
    }

    /// Returns a vector of function Rc references.
    pub fn function_rc_vec(&self) -> Vec<Rc<Func>> {
        self.functions.iter().map(|(func_rc, _)| func_rc.clone()).collect()
    }

    /// Adds a check constraint with its metadata to the builder.
    pub fn add_check_constraint(mut self, constraint: Rc<Ch>, metadata: Ch::Meta) -> Self {
        self.check_constraints.push((constraint, metadata));
        self
    }
}

impl<T, C, U, F, Func, Ch> Attributed for GenericDBBuilder<T, C, U, F, Func, Ch>
where
    T: TableLike,
    C: ColumnLike,
    U: UniqueIndexLike,
    F: ForeignKeyLike,
    Func: FunctionLike,
    Ch: CheckConstraintLike,
{
    type Attribute = GenericDBAttribute;
}

impl<T, C, U, F, Func, Ch> IsCompleteBuilder for GenericDBBuilder<T, C, U, F, Func, Ch>
where
    T: TableLike,
    C: ColumnLike,
    U: UniqueIndexLike,
    F: ForeignKeyLike,
    Func: FunctionLike,
    Ch: CheckConstraintLike,
{
    fn is_complete(&self) -> bool {
        self.catalog_name.is_some()
    }
}

impl<T, C, U, F, Func, Ch> Builder for GenericDBBuilder<T, C, U, F, Func, Ch>
where
    T: TableLike,
    C: ColumnLike,
    U: UniqueIndexLike,
    F: ForeignKeyLike,
    Func: FunctionLike,
    Ch: CheckConstraintLike,
{
    type Error = BuilderError<GenericDBAttribute>;
    type Object = GenericDB<T, C, U, F, Func, Ch>;

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
        self.check_constraints.sort_unstable_by(|(a, _), (b, _)| a.as_ref().cmp(b.as_ref()));

        Ok(GenericDB {
            catalog_name,
            timezone: self.timezone,
            tables: self.tables,
            columns: self.columns,
            unique_indices: self.unique_indices,
            foreign_keys: self.foreign_keys,
            functions: self.functions,
            check_constraints: self.check_constraints,
        })
    }
}
