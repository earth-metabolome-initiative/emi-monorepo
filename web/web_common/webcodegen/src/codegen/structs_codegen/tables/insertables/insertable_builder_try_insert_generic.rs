//! Submodule providing the generation of the `try_insert` method for the
//! insertable builder struct.

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{Table, errors::WebCodeGenError, traits::TableLike};

impl Table {
    /// Returns the implementation of the `TryInsertGeneric` trait for the
    /// insertable builder struct.
    ///
    /// # Arguments
    ///
    /// * `table` - The table for which the implementation is generated.
    /// * `conn` - The database connection to use for querying the database.
    ///
    /// # Errors
    ///
    /// If the provided connection to the database fails.
    pub(super) fn generate_insertable_builder_try_insert_generic_implementation(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let builder_ident = self.insertable_builder_ident()?;
        let row_path = self.import_struct_path()?;
        let attributes_enum = self.attributes_enum_ident()?;
        let table_generics = self.generics(conn)?;
        let primary_key_type = self.primary_key_type(conn)?;

        let left_generics = {
            let mut left_generics = table_generics.clone();
            left_generics.push(Ident::new("C", proc_macro2::Span::call_site()));
            left_generics
        };

        let maybe_generics = if table_generics.is_empty() {
            None
        } else {
            Some(quote! { < #(#table_generics),* > })
        };

        Ok(quote! {
            impl < #(#left_generics),* > web_common_traits::database::TryInsertGeneric<C> for #builder_ident #maybe_generics
                where
                    Self: web_common_traits::database::DispatchableInsertableVariant<
                        C,
                        Row=#row_path,
                        Error= web_common_traits::database::InsertError<#attributes_enum>,
                    > + web_common_traits::database::SetPrimaryKey<
                        PrimaryKey=#primary_key_type
                    > + common_traits::builder::IsCompleteBuilder,
            {
                type Error = web_common_traits::database::InsertError<#attributes_enum>;

                fn mint_primary_key(
                    self,
                    user_id: i32,
                    conn: &mut C,
                ) -> Result<Self::PrimaryKey, Self::Error>
                {
                    use diesel::Identifiable;
                    use web_common_traits::database::DispatchableInsertableVariant;
                    Ok(self.insert(user_id, conn)?.id())
                }
            }
        })
    }
}
