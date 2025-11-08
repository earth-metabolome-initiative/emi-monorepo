//! Submodule defining the `TableInsertable` struct.

use quote::quote;
use sql_traits::traits::TableLike;
use synql_core::{
    prelude::Builder,
    structs::{Decorator, Derive, InternalToken},
};

use crate::traits::TableInsertableLike;

mod into_crate;
mod into_data;
mod into_module;
mod value_settable;

#[derive(Debug)]
/// Struct representing a SynQL table insertable.
pub struct TableInsertable<'data, 'table, T: TableLike + ?Sized> {
    table: &'table T,
    workspace: &'table synql_core::structs::Workspace<'data>,
    database: &'table T::DB,
}

impl<'data, 'table, T: TableLike + ?Sized> Clone for TableInsertable<'data, 'table, T> {
    fn clone(&self) -> Self {
        Self { table: self.table, workspace: self.workspace, database: self.database }
    }
}

impl<'data, 'table, T: TableLike + ?Sized> Copy for TableInsertable<'data, 'table, T> {}

impl<'data, 'table, T: TableInsertableLike + ?Sized> TableInsertable<'data, 'table, T> {
    pub(crate) fn new(
        table: &'table T,
        workspace: &'table synql_core::structs::Workspace<'data>,
        database: &'table T::DB,
    ) -> Self {
        Self { table, workspace, database }
    }

    /// Returns the derives for the insertable struct.
    fn diesel_derives(&self) -> Derive {
        let traits = vec![
            self.workspace.external_trait("Insertable").expect("Failed to get Insertable trait"),
        ];

        Derive::new()
            .add_traits(traits)
            .expect("Failed to add Insertable trait to derive")
            .build()
            .expect("Failed to build Insertable derive")
    }

    /// Returns the table decorator for the insertable struct.
    fn table_decorator(&self) -> Decorator {
        let table_schema = self
            .table
            .schema_module(self.workspace)
            .expect("Failed to get the table schema module");
        let snake_case_ident = self.table.table_snake_ident();

        Decorator::new()
            .token(
                InternalToken::new()
                    .private()
                    .stream(quote! {
                        diesel(table_name = #table_schema::#snake_case_ident)
                    })
                    .internal_module(table_schema)
                    .build()
                    .unwrap(),
            )
            .unwrap()
            .build()
            .unwrap()
    }
}
