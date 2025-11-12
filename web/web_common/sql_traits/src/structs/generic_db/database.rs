//! Implementation of the `DatabaseLike` trait for `GenericDB`.

use crate::{
    structs::GenericDB,
    traits::{
        CheckConstraintLike, ColumnLike, DatabaseLike, ForeignKeyLike, FunctionLike, TableLike,
        UniqueIndexLike,
    },
};

impl<T, C, U, F, Func, Ch> DatabaseLike for GenericDB<T, C, U, F, Func, Ch>
where
    T: TableLike<DB = Self>,
    C: ColumnLike<DB = Self>,
    U: UniqueIndexLike<DB = Self>,
    F: ForeignKeyLike<DB = Self>,
    Func: FunctionLike<DB = Self>,
    Ch: CheckConstraintLike<DB = Self>,
{
    type Table = T;
    type Column = C;
    type ForeignKey = F;
    type Function = Func;
    type UniqueIndex = U;
    type CheckConstraint = Ch;

    #[inline]
    fn catalog_name(&self) -> &str {
        &self.catalog_name
    }

    #[inline]
    fn number_of_tables(&self) -> usize {
        self.tables.len()
    }

    #[inline]
    fn timezone(&self) -> Option<&str> {
        self.timezone.as_deref()
    }

    fn table(&self, schema: Option<&str>, table_name: &str) -> Option<&Self::Table> {
        // The tables are sorted by schema and name, so we can use binary search.
        let key = (schema, table_name);
        self.tables
            .binary_search_by_key(&key, |(table, _)| (table.table_schema(), table.table_name()))
            .ok()
            .map(|index| self.tables[index].0.as_ref())
    }

    #[inline]
    fn tables(&self) -> impl Iterator<Item = &Self::Table> {
        self.tables.iter().map(|(table, _)| table.as_ref())
    }

    #[inline]
    fn functions(&self) -> impl Iterator<Item = &Self::Function> {
        self.functions.iter().map(|(func, _)| func.as_ref())
    }

    fn function(&self, name: &str) -> Option<&Self::Function> {
        self.functions
            .binary_search_by(|(f, _)| f.name().cmp(name))
            .ok()
            .map(|index| self.functions[index].0.as_ref())
    }
}
