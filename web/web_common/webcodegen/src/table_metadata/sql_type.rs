use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;
use diesel::sql_types::Oid;
use diesel::{Queryable, Selectable};
use prettyplease::unparse;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_str, File, Ident, Type};

#[derive(Debug)]
pub struct SQLType {
    name: String,
}

impl SQLType {
    pub fn load_all(conn: &mut PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::pg_type;
        use diesel::prelude::*;
        pg_type::dsl::pg_type
            .filter(
                pg_type::dsl::typname
                    .not_ilike("pg_%")
                    .and(pg_type::dsl::typname.not_ilike(r"\_%"))
                    .and(pg_type::dsl::typname.not_like("any%")),
            )
            .select(pg_type::dsl::typname)
            .load::<String>(conn)
            .map(|names| names.into_iter().map(|name| Self { name }).collect())
    }

    pub fn rust_diesel_name(&self) -> String {
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
        let name = Ident::new(&self.name, proc_macro2::Span::call_site());
        let rust_name = Ident::new(&self.rust_diesel_name(), proc_macro2::Span::call_site());
        quote! {
            #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
            #[diesel(postgres_type(name = "#name"))]
            pub struct #rust_name;
        }
    }

    pub fn write_all(conn: &mut PgConnection, output_path: &str) -> Result<(), DieselError> {
        let types = Self::load_all(conn)?;

        // We convert the types to TokenStream
        let types = types.iter().map(|f| f.to_syn());

        // Create a new TokenStream
        let output = quote! {
            #( #types )*
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
