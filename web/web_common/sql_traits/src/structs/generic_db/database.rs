//! Implementation of the `DatabaseLike` trait for `GenericDB`.

use crate::{
    structs::GenericDB,
    traits::{DatabaseLike, TableLike},
};

impl<T> DatabaseLike for GenericDB<T>
where
    T: TableLike<Database = Self>,
{
    type Table = T;
    type Column = <T as TableLike>::Column;
    type ForeignKey = <T as TableLike>::ForeignKey;

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
}
