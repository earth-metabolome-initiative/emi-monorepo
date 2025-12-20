//! Submodule defining and implementing the `CheckConstraintSyn` trait, which
//! provide methods to facilitate the rust code generation starting from a SQL
//! column representation, building on top of the
//! [`CheckConstraintLike`](sql_traits::traits::CheckConstraintLike) trait.

use proc_macro2::TokenStream;
use quote::quote;
use sql_traits::traits::{CheckConstraintLike, DatabaseLike};

mod sub_expressions;
mod translate_expression;
use translate_expression::TranslateExpression;

use crate::structs::Workspace;

/// Trait implemented by types that represent SQL check constraints and can be
/// used to generate Rust code for them.
pub trait CheckConstraintSynLike: CheckConstraintLike {
    /// Returns the tokenstream representing the check constraint in Rust code.
    ///
    /// # Arguments
    ///
    /// * `database` - The database connection to use to query additional
    ///  information about the check constraint if needed.
    /// * `workspace` - The workspace where the generated code will be placed.
    /// * `contextual_columns` - The columns that are in the context where the
    ///   check constraint is applied.
    fn to_syn<'db>(
        &'db self,
        database: &'db Self::DB,
        workspace: &Workspace,
        contextual_columns: &[&'db <Self::DB as DatabaseLike>::Column],
    ) -> TokenStream {
        let translator: TranslateExpression<'_, 'db, <Self as CheckConstraintLike>::DB> =
            TranslateExpression::new(self.borrow(), workspace, database, contextual_columns);

        let mut translated_expressions: Vec<TokenStream> = Vec::new();

        for sub_expression in sub_expressions::sub_expressions(self.expression(database)) {
            translated_expressions.push(translator.parse(sub_expression));
        }

        let relevant_optional_columns = self
            .columns(database)
            .filter(|column| !contextual_columns.contains(&column))
            .collect::<Vec<_>>();

        if relevant_optional_columns.is_empty() {
            translated_expressions.into()
        } else {
            let left = relevant_optional_columns.iter().map(|column| column.column_snake_ident());
            let right = relevant_optional_columns.iter().map(|column| {
                let ident = column.column_snake_ident();
                if column.supports_copy(database, workspace) {
                    quote! {
                        self.#ident
                    }
                } else {
                    quote! {
                        self.#ident.as_ref()
                    }
                }
            });

            TokenStream::new()
                .stream(quote! {
                    if let #(Some(#left)),* = #(#right),* {
                        #( #translated_expressions )*
                    }
                })
                .inherits(translated_expressions)
                .build()
                .unwrap()
        }
    }
}

impl<T> CheckConstraintSynLike for T where T: CheckConstraintLike {}
