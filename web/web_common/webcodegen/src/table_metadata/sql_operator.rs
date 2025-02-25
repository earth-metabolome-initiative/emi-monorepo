use diesel::PgConnection;
use prettyplease::unparse;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{File, Ident, Type};

use crate::{
    errors::WebCodeGenError,
    table_metadata::{
        pg_type::{postgres_type_to_diesel, postgres_type_to_diesel_str},
        sql_function::UNSUPPORTED_DATA_TYPES,
    },
};

/// A struct representing a SQL operator
pub struct SQLOperator {
    /// The symbol of the operator
    pub symbol: String,
    /// The type of the left operand
    pub left_operand_type: String,
    /// The type of the right operand
    pub right_operand_type: String,
    /// The type of the result
    pub result_type: String,
    /// The name of the operator
    pub name: String,
}

const DEPRECATED_OPERATORS: &[(&str, &str)] =
    &[("point_above", ">^"), ("point_below", "<^"), ("ts_match_vq", "@@@"), ("ts_match_qv", "@@@")];

impl SQLOperator {
    /// Load all the SQL operators from the database
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Vec` of `SQLOperator` if the operation was
    /// successful, or a `WebCodeGenError` if an error occurred
    ///
    /// # Errors
    ///
    /// If an error occurs while loading the operators from the database
    pub fn load_all(conn: &mut PgConnection) -> Result<Vec<Self>, WebCodeGenError> {
        use diesel::prelude::*;

        use crate::schema::{pg_operator, pg_proc, pg_type};

        let (left_pg_type, right_pg_type, return_pg_type) = diesel::alias!(
            pg_type as left_pg_type_alias,
            pg_type as right_pg_type_alias,
            pg_type as return_pg_type_alias
        );

        pg_operator::table
            .inner_join(left_pg_type.on(pg_operator::oprleft.eq(left_pg_type.field(pg_type::oid))))
            .inner_join(
                right_pg_type.on(pg_operator::oprright.eq(right_pg_type.field(pg_type::oid))),
            )
            .inner_join(
                return_pg_type.on(pg_operator::oprresult.eq(return_pg_type.field(pg_type::oid))),
            )
            .inner_join(pg_proc::table.on(pg_operator::oprcode.eq(pg_proc::oid)))
            .filter(
                pg_proc::proname
                    .not_ilike("%eq%")
                    .and(pg_proc::proname.not_ilike("%le%"))
                    .and(pg_proc::proname.not_ilike("%ge%"))
                    .and(pg_proc::proname.not_ilike("%lt%"))
                    .and(pg_proc::proname.not_ilike("%gt%"))
                    .and(pg_proc::proname.not_ilike("%ne%")),
            )
            .select((
                pg_operator::oprname,
                left_pg_type.field(pg_type::typname),
                right_pg_type.field(pg_type::typname),
                return_pg_type.field(pg_type::typname),
                pg_proc::proname,
            ))
            .load::<(String, String, String, String, String)>(conn)
            .map_err(WebCodeGenError::from)
            .and_then(|rows| {
                rows.into_iter()
                    .filter(|(symbol, left_operand_type, right_operand_type, result_type, name)| {
                        if UNSUPPORTED_DATA_TYPES.contains(&left_operand_type.as_str())
                            || UNSUPPORTED_DATA_TYPES.contains(&right_operand_type.as_str())
                            || UNSUPPORTED_DATA_TYPES.contains(&result_type.as_str())
                        {
                            return false;
                        }
                        if DEPRECATED_OPERATORS.contains(&(name.as_str(), symbol.as_str())) {
                            return false;
                        }

                        true
                    })
                    .map(|(symbol, left_operand_type, right_operand_type, result_type, name)| {
                        Ok(Self {
                            symbol,
                            left_operand_type,
                            right_operand_type,
                            result_type,
                            name,
                        })
                    })
                    .collect()
            })
    }

    /// Returns the type of the left operand
    pub fn left_operand_type(&self) -> Result<Type, WebCodeGenError> {
        postgres_type_to_diesel(self.left_operand_type.as_str(), false)
    }

    /// Returns the type of the right operand
    pub fn right_operand_type(&self) -> Result<Type, WebCodeGenError> {
        postgres_type_to_diesel(self.right_operand_type.as_str(), false)
    }

    /// Returns the type of the result
    pub fn result_type(&self) -> Result<Type, WebCodeGenError> {
        postgres_type_to_diesel(self.result_type.as_str(), false)
    }

    #[must_use]
    /// Returns the struct name of the SQL operator
    pub fn struct_name(&self) -> String {
        self.name
            .split('_')
            .map(|s| {
                let mut chars = s.chars();
                match chars.next() {
                    None => String::new(),
                    Some(c) => c.to_uppercase().chain(chars).collect(),
                }
            })
            .collect()
    }

    /// Returns whether the operator includes `postgil_diesel` types
    pub fn includes_postgres_diesel_types(&self) -> Result<bool, WebCodeGenError> {
        Ok(postgres_type_to_diesel_str(&self.left_operand_type)?
            == "postgis_diesel::sql_types::Geometry"
            || postgres_type_to_diesel_str(&self.right_operand_type)?
                == "postgis_diesel::sql_types::Geometry"
            || postgres_type_to_diesel_str(&self.result_type)?
                == "postgis_diesel::sql_types::Geometry")
    }

    #[must_use]
    /// Convert the SQL operator to a `TokenStream`
    ///
    /// # Returns
    ///
    /// A `TokenStream` representing the SQL operator
    ///
    /// # Errors
    ///
    /// If the return, the left or the right operand types returned an error
    pub fn to_syn(&self) -> Result<TokenStream, WebCodeGenError> {
        let name = Ident::new(&self.struct_name(), proc_macro2::Span::call_site());
        let symbol = self.symbol.clone();
        let return_type = self.result_type()?;
        let left_type = self.left_operand_type()?;
        let right_type = self.right_operand_type()?;

        let sanitized_name =
            Ident::new(self.name.replace('.', "_").as_str(), proc_macro2::Span::call_site());
        let trait_name = Ident::new(
            format!("Has{}", self.struct_name()).as_str(),
            proc_macro2::Span::call_site(),
        );

        Ok(quote! {
            diesel::infix_operator!(
                #name,
                #symbol,
                #return_type,
                backend: diesel::pg::Pg
            );

            pub trait #trait_name: Sized + diesel::expression::Expression<SqlType=#left_type> {
                fn #sanitized_name<Rhs>(self, rhs: Rhs) -> #name<Self, Rhs::Expression>
                where
                    Rhs: diesel::expression::AsExpression<#right_type>,
                {
                    #name::new(self, rhs.as_expression())
                }
            }

            impl<T> #trait_name for T
            where
                T: Sized + diesel::expression::Expression<SqlType=#left_type>,
            {}
        })
    }

    /// Write all the SQL operators to a file
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    /// * `output_path` - The path to the file where the generated code will be
    ///   written
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` if the operation was successful, or a
    /// `WebCodeGenError` if an error occurred
    ///
    /// # Errors
    ///
    /// If an error occurs while loading the operators from the database, or
    /// while writing the generated code to the output file
    pub fn write_all(conn: &mut PgConnection, output_path: &str) -> Result<(), WebCodeGenError> {
        let operators = Self::load_all(conn)?;

        // Filter out all the operators that include postgis_diesel types
        let operators = operators
            .into_iter()
            .filter(|op| !op.includes_postgres_diesel_types().unwrap_or(false))
            .collect::<Vec<_>>();

        // We convert the types to TokenStream
        let operators = operators.iter().map(SQLOperator::to_syn).collect::<Result<Vec<_>, _>>()?;

        // Create a new TokenStream
        let output = quote! {
            #( #operators )*
        };

        // Convert the generated TokenStream to a string
        let code_string = output.to_string();

        // Parse the generated code string into a syn::Item
        let syntax_tree: File = syn::parse_str(&code_string)?;

        // Use prettyplease to format the syntax tree
        let formatted_code = unparse(&syntax_tree);

        // Write the formatted code to the output file
        std::fs::write(output_path, formatted_code)?;

        Ok(())
    }
}
