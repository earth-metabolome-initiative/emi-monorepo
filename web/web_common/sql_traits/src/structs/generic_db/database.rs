//! Implementation of the `DatabaseLike` trait for `GenericDB`.

use crate::{
    structs::GenericDB,
    traits::{DatabaseLike, FunctionLike, TableLike},
};

impl<T, F> DatabaseLike for GenericDB<T, F>
where
    T: TableLike<Database = Self>,
    F: FunctionLike,
{
    type Table = T;
    type Column = <T as TableLike>::Column;
    type ForeignKey = <T as TableLike>::ForeignKey;
    type Function = F;

    fn catalog_name(&self) -> &str {
        &self.catalog_name
    }

    fn number_of_tables(&self) -> usize {
        self.tables.len()
    }

    fn table(&self, schema: Option<&str>, table_name: &str) -> &Self::Table {
        // The tables are sorted by schema and name, so we can use binary search.
        let key = (schema, table_name);
        match self
            .tables
            .binary_search_by_key(&key, |(table, _)| (table.table_schema(), table.table_name()))
        {
            Ok(index) => &self.tables[index].0,
            Err(_) => panic!("Table not found: {:?}.{:?}", schema, table_name),
        }
    }

    fn tables(&self) -> impl Iterator<Item = &Self::Table> {
        self.tables.iter().map(|(table, _)| table.as_ref())
    }

    fn functions(&self) -> impl Iterator<Item = &Self::Function> {
        self.functions.iter().map(|(func, _)| func)
    }

    fn function(&self, name: &str) -> Option<&Self::Function> {
        self.functions
            .binary_search_by(|(f, _)| f.name().cmp(name))
            .ok()
            .map(|index| &self.functions[index].0)
    }
}
