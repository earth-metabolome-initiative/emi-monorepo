//! Builder for constructing a `GenericDB` instance.

use std::rc::Rc;

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

impl<T, C, U, F, Func, Ch> GenericDBBuilder<T, C, U, F, Func, Ch>
where
    T: TableLike,
    C: ColumnLike,
    U: UniqueIndexLike,
    F: ForeignKeyLike,
    Func: FunctionLike,
    Ch: CheckConstraintLike,
{
    #[must_use]
    /// Creates a new `GenericDBBuilder` instance.
    pub fn new(catalog_name: String) -> Self {
        Self {
            catalog_name,
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

impl<T, C, U, F, Func, Ch> GenericDBBuilder<T, C, U, F, Func, Ch>
where
    T: TableLike,
    C: ColumnLike,
    U: UniqueIndexLike,
    F: ForeignKeyLike,
    Func: FunctionLike,
    Ch: CheckConstraintLike,
{
    /// Sets the timezone for the database.
    #[must_use]
    #[inline]
    pub fn timezone(mut self, timezone: String) -> Self {
        self.timezone = Some(timezone);
        self
    }

    /// Adds a table with its metadata to the builder.
    #[must_use]
    pub fn add_table(mut self, table: Rc<T>, metadata: T::Meta) -> Self {
        self.tables.push((table, metadata));
        self
    }

    /// Adds multiple tables with their metadata to the builder.
    #[must_use]
    pub fn add_tables(mut self, tables: impl IntoIterator<Item = (Rc<T>, T::Meta)>) -> Self {
        self.tables.extend(tables);
        self
    }

    /// Adds a column with its metadata to the builder.
    #[must_use]
    #[inline]
    pub fn add_column(mut self, column: Rc<C>, metadata: C::Meta) -> Self {
        self.columns.push((column, metadata));
        self
    }

    /// Adds multiple columns with their metadata to the builder.
    #[must_use]
    #[inline]
    pub fn add_columns(mut self, columns: impl IntoIterator<Item = (Rc<C>, C::Meta)>) -> Self {
        self.columns.extend(columns);
        self
    }

    /// Adds a unique index with its metadata to the builder.
    #[must_use]
    #[inline]
    pub fn add_unique_index(mut self, index: Rc<U>, metadata: U::Meta) -> Self {
        self.unique_indices.push((index, metadata));
        self
    }

    /// Adds multiple unique indices with their metadata to the builder.
    #[must_use]
    #[inline]
    pub fn add_unique_indices(
        mut self,
        indices: impl IntoIterator<Item = (Rc<U>, U::Meta)>,
    ) -> Self {
        self.unique_indices.extend(indices);
        self
    }

    /// Adds a foreign key with its metadata to the builder.
    #[must_use]
    #[inline]
    pub fn add_foreign_key(mut self, key: Rc<F>, metadata: F::Meta) -> Self {
        self.foreign_keys.push((key, metadata));
        self
    }

    /// Adds multiple foreign keys with their metadata to the builder.
    #[must_use]
    #[inline]
    pub fn add_foreign_keys(mut self, keys: impl IntoIterator<Item = (Rc<F>, F::Meta)>) -> Self {
        self.foreign_keys.extend(keys);
        self
    }

    /// Adds a function with its metadata to the builder.
    #[must_use]
    #[inline]
    pub fn add_function(mut self, function: Rc<Func>, metadata: Func::Meta) -> Self {
        self.functions.push((function, metadata));
        self
    }

    /// Adds multiple functions with their metadata to the builder.
    #[must_use]
    #[inline]
    pub fn add_functions(
        mut self,
        functions: impl IntoIterator<Item = (Rc<Func>, Func::Meta)>,
    ) -> Self {
        self.functions.extend(functions);
        self
    }

    /// Returns a vector of function Rc references.
    #[must_use]
    pub fn function_rc_vec(&self) -> Vec<Rc<Func>> {
        self.functions.iter().map(|(func_rc, _)| func_rc.clone()).collect()
    }

    /// Returns a slice of table Rc references with their metadata.
    #[must_use]
    pub fn tables(&self) -> &[(Rc<T>, T::Meta)] {
        &self.tables
    }

    /// Adds a check constraint with its metadata to the builder.
    #[must_use]
    pub fn add_check_constraint(mut self, constraint: Rc<Ch>, metadata: Ch::Meta) -> Self {
        self.check_constraints.push((constraint, metadata));
        self
    }
}

impl<T, C, U, F, Func, Ch> From<GenericDBBuilder<T, C, U, F, Func, Ch>>
    for GenericDB<T, C, U, F, Func, Ch>
where
    T: TableLike,
    C: ColumnLike,
    U: UniqueIndexLike,
    F: ForeignKeyLike,
    Func: FunctionLike,
    Ch: CheckConstraintLike,
{
    fn from(mut builder: GenericDBBuilder<T, C, U, F, Func, Ch>) -> Self {
        let catalog_name = builder.catalog_name;

        builder.tables.sort_unstable_by_key(|(table, _)| {
            (
                table.table_schema().map(std::string::ToString::to_string),
                table.table_name().to_string(),
            )
        });

        builder.columns.sort_unstable_by(|(a, _), (b, _)| a.as_ref().cmp(b.as_ref()));
        builder.unique_indices.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
        builder.foreign_keys.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
        builder.functions.sort_unstable_by(|(a, _), (b, _)| a.name().cmp(b.name()));
        builder.check_constraints.sort_unstable_by(|(a, _), (b, _)| a.as_ref().cmp(b.as_ref()));

        GenericDB {
            catalog_name,
            timezone: builder.timezone,
            tables: builder.tables,
            columns: builder.columns,
            unique_indices: builder.unique_indices,
            foreign_keys: builder.foreign_keys,
            functions: builder.functions,
            check_constraints: builder.check_constraints,
        }
    }
}
