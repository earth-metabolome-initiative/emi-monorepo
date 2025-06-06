//! Submodule implementing the `Ancestor` trait for a table.

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;

use crate::errors::WebCodeGenError;

impl crate::Table {
    /// Generates the implementation of the `Ancestor` trait for the table.
    pub(super) fn ancestor_traits_impl(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<TokenStream>, WebCodeGenError> {
        let mut homogeneous_parent_columns = self.homogeneous_parent_columns(conn)?;

        if homogeneous_parent_columns.is_empty() {
            // If there are no homogeneous parent columns, we cannot implement the Ancestor
            // trait.
            return Ok(None);
        }

        if homogeneous_parent_columns.len() > 1 {
            unimplemented!(
                "Ancestor trait cannot be implemented for table `{}.{}` with multiple homogeneous parent columns",
                self.table_schema,
                self.table_name
            );
        }

        // If any of the homogeneous parent columns are multiple, we do not know, at
        // this time, how to implement the Ancestor trait.
        if homogeneous_parent_columns.iter().any(|col| col.len() > 1) {
            unimplemented!(
                "Ancestor trait cannot be implemented for table `{}.{}` as it contains a multiple homogeneous parent column",
                self.table_schema,
                self.table_name
            );
        }

        let struct_ident = self.struct_ident()?;

        let homogeneous_parent_column = homogeneous_parent_columns.pop().unwrap().pop().unwrap();
        let parent_id_ident = homogeneous_parent_column.snake_case_ident()?;
        let parent_id = homogeneous_parent_column.column_name;
        let primary_key_column = self.primary_key_columns(conn)?.pop().unwrap();
        let primary_key_type = primary_key_column.diesel_type(conn)?;
        let primary_key_name = primary_key_column.column_name;

        Ok(Some(quote! {
            impl<C> web_common_traits::prelude::Ancestor<C> for #struct_ident
            where
                Self: web_common_traits::prelude::TableName + Sized,
                C: diesel::connection::LoadConnection,
                <C as diesel::Connection>::Backend:
                    diesel::backend::DieselReserveSpecialization + diesel::sql_types::HasSqlType<#primary_key_type> + 'static,
                web_common_traits::prelude::AncestorExists: diesel::deserialize::FromSqlRow< diesel::sql_types::Untyped, <C as diesel::Connection>::Backend>,
                for<'a> &'a Self: diesel::Identifiable,
                for<'a> <&'a Self as diesel::Identifiable>::Id:
                    diesel::serialize::ToSql<#primary_key_type, C::Backend>,
            {
                const PARENT_ID: &'static str = #parent_id;
                const ID: &'static str = #primary_key_name;
                type SqlType = #primary_key_type;
            }

            impl web_common_traits::prelude::Descendant<#struct_ident> for #struct_ident {
                fn parent_id(&self) -> Option<<&Self as diesel::Identifiable>::Id> {
                    self.#parent_id_ident.as_ref()
                }
            }
        }))
    }
}
