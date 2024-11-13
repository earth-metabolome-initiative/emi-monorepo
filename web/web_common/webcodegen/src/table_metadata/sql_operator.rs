// SELECT
//     oprname AS symbol,
//     oprleft::regtype AS left_operand_type,
//     oprright::regtype AS right_operand_type,
//     oprresult::regtype AS result_type,
//     oprcode AS name
// FROM
//     pg_operator

use diesel::PgConnection;
use prettyplease::unparse;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{File, Ident, Type};

use crate::table_metadata::sql_function::{postgres_type_to_diesel, UNSUPPORTED_DATA_TYPES};

pub struct SQLOperator {
    pub symbol: String,
    pub left_operand_type: Type,
    pub right_operand_type: Type,
    pub result_type: Type,
    pub name: String,
}

const DEPRECATED_OPERATORS: &[(&str, &str)] = &[
    ("point_above", ">^"),
    ("point_below", "<^"),
    ("ts_match_vq", "@@@"),
    ("ts_match_qv", "@@@")
];

impl SQLOperator {
    pub fn load_all(conn: &mut PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::pg_operator;
        use crate::schema::pg_proc;
        use crate::schema::pg_type;
        use diesel::prelude::*;

        let (left_pg_type, right_pg_type, return_pg_type) = diesel::alias!(
            pg_type as left_pg_type_alias,
            pg_type as right_pg_type_alias,
            pg_type as return_pg_type_alias
        );

        pg_operator::dsl::pg_operator
            .inner_join(
                left_pg_type
                    .on(pg_operator::dsl::oprleft.eq(left_pg_type.field(pg_type::dsl::oid))),
            )
            .inner_join(
                right_pg_type
                    .on(pg_operator::dsl::oprright.eq(right_pg_type.field(pg_type::dsl::oid))),
            )
            .inner_join(
                return_pg_type
                    .on(pg_operator::dsl::oprresult.eq(return_pg_type.field(pg_type::dsl::oid))),
            )
            .inner_join(pg_proc::table.on(pg_operator::dsl::oprcode.eq(pg_proc::dsl::oid)))
            .filter(
                pg_proc::dsl::proname
                    .not_ilike("%eq%")
                    .and(pg_proc::dsl::proname.not_ilike("%le%"))
                    .and(pg_proc::dsl::proname.not_ilike("%ge%"))
                    .and(pg_proc::dsl::proname.not_ilike("%lt%"))
                    .and(pg_proc::dsl::proname.not_ilike("%gt%"))
                    .and(pg_proc::dsl::proname.not_ilike("%ne%")),
            )
            .select((
                pg_operator::dsl::oprname,
                left_pg_type.field(pg_type::dsl::typname),
                right_pg_type.field(pg_type::dsl::typname),
                return_pg_type.field(pg_type::dsl::typname),
                pg_proc::dsl::proname,
            ))
            .load::<(String, String, String, String, String)>(conn)
            .map(|rows| {
                rows.into_iter()
                    .filter_map(
                        |(symbol, left_operand_type, right_operand_type, result_type, name)| {
                            if UNSUPPORTED_DATA_TYPES.contains(&left_operand_type.as_str())
                                || UNSUPPORTED_DATA_TYPES.contains(&right_operand_type.as_str())
                                || UNSUPPORTED_DATA_TYPES.contains(&result_type.as_str())
                            {
                                return None;
                            }
                            if DEPRECATED_OPERATORS.contains(&(
                                name.as_str(),
                                symbol.as_str(),
                            )) {
                                return None;
                            }

                            Some(Self {
                                symbol,
                                left_operand_type: postgres_type_to_diesel(
                                    left_operand_type.as_str(),
                                ),
                                right_operand_type: postgres_type_to_diesel(
                                    right_operand_type.as_str(),
                                ),
                                result_type: postgres_type_to_diesel(result_type.as_str()),
                                name,
                            })
                        },
                    )
                    .collect()
            })
    }

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

    pub fn to_syn(&self) -> TokenStream {
        let name = Ident::new(&self.struct_name(), proc_macro2::Span::call_site());
        let symbol = self.symbol.clone();
        let return_type = self.result_type.clone();
        let left_type = self.left_operand_type.clone();
        let right_type = self.right_operand_type.clone();

        let sanitized_name = Ident::new(
            self.name.replace(".", "_").as_str(),
            proc_macro2::Span::call_site(),
        );
        let trait_name = Ident::new(
            format!("Has{}", self.struct_name()).as_str(),
            proc_macro2::Span::call_site(),
        );

        quote! {
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
        }
    }

    pub fn write_all(
        conn: &mut PgConnection,
        output_path: &str,
    ) -> Result<(), diesel::result::Error> {
        let operators = Self::load_all(conn)?;

        // We convert the types to TokenStream
        let operators = operators.iter().map(|f| f.to_syn());

        // Create a new TokenStream
        let output = quote! {
            #( #operators )*
        };

        // Convert the generated TokenStream to a string
        let code_string = output.to_string();

        // Parse the generated code string into a syn::Item
        let syntax_tree: File = syn::parse_str(&code_string).unwrap();

        // Use prettyplease to format the syntax tree
        let formatted_code = unparse(&syntax_tree);

        // Write the formatted code to the output file
        std::fs::write(output_path, formatted_code).unwrap();

        Ok(())
    }
}
