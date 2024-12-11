use crate::table_metadata::pg_type::postgres_type_to_diesel;
use diesel::pg::PgConnection;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, TextExpressionMethods};
use prettyplease::unparse;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{File, Ident, Type};
use crate::errors::WebCodeGenError;

pub const UNSUPPORTED_DATA_TYPES: &[&str] = &[
    "internal",
    "anyelement",
    "valid_detail",
    "box",
    "box2d",
    "box3d",
    "box2df",
    "box3df",
    "spheroid",
    "circle",
    "path",
    "lseg",
    "gidx",
    "name",
    "anycompatible",
    "anycompatiblearray",
    "-",
    "tid",
    "xid",
    "xid8",
    "cid",
    "anyarray",
    "anynonarray",
    "oidvector",
    "aclitem",
    "macaddr8",
    "aclitem[]",
    "bit",
    "bit varying",
    "pg_lsn",
    "anyenum",
    "anyrange",
    "record",
    "anymultirange",
    "jsonpath",
    "timetz",
    "varbit",
    "numeric",
    "_text",
];


pub struct SQLFunction {
    name: String,
    return_type: Option<Type>,
    arguments: Vec<(String, Type)>,
}

impl SQLFunction {
    pub fn load_all(conn: &mut PgConnection) -> Result<Vec<SQLFunction>, WebCodeGenError> {
        use crate::schema::pg_namespace;
        use crate::schema::pg_proc;
        use crate::sql_functions::{pg_get_function_arguments, pg_get_function_result};

        let data: Vec<(_, _, _)> = pg_proc::table
            .inner_join(pg_namespace::table.on(pg_proc::pronamespace.eq(pg_namespace::oid)))
            .filter(pg_get_function_result(pg_proc::oid).ne("trigger"))
            .filter(pg_namespace::dsl::nspname.ne("pg_catalog"))
            .filter(pg_namespace::dsl::nspname.ne("information_schema"))
            .filter(pg_proc::dsl::proname.not_like("diesel_%"))
            .filter(pg_proc::dsl::proname.not_like("uuid_%"))
            .filter(pg_proc::dsl::proname.not_like("set_%"))
            .filter(pg_proc::dsl::proname.not_like("show_%"))
            .filter(pg_proc::dsl::proname.not_like("gtrgm_%"))
            .filter(pg_proc::dsl::proname.not_like("gin_%"))
            .select((
                pg_proc::dsl::proname,
                pg_get_function_result(pg_proc::oid),
                pg_get_function_arguments(pg_proc::oid),
            ))
            .load::<(String, String, String)>(conn)?;

        let mut sql_functions: Vec<SQLFunction> = Vec::new();
        let mut overloading_functions: Vec<SQLFunction> = Vec::new();

        for (function_name, return_type, arguments) in data {
            let mut sql_function = SQLFunction {
                name: function_name.clone(),
                return_type: None,
                arguments: Vec::new(),
            };
            let arguments: Vec<&str> = if !arguments.is_empty() {
                arguments.split(", ").collect()
            } else {
                Vec::new()
            };

            if overloading_functions
                .iter()
                .any(|f| f.name == function_name)
            {
                continue;
            }

            if let Some(pos) = sql_functions.iter().position(|f| f.name == function_name) {
                sql_functions.remove(pos);
                overloading_functions.push(sql_function);
                continue;
            }

            if return_type.contains("OUT")
                || return_type.contains("SETOF")
                || return_type.contains("TABLE")
                || arguments.iter().any(|a| a.contains("SETOF"))
            {
                continue;
            }

            if function_name.starts_with('_') {
                continue;
            }

            let mut found_unsupported_data_type = false;

            for (i, argument) in arguments.iter().enumerate() {
                let argument = if let Some(pos) = argument.find(" DEFAULT ") {
                    &argument[..pos]
                } else {
                    argument
                };

                let argument_type = if let Some(pos) = argument.find(" ") {
                    &argument[pos + 1..]
                } else {
                    argument
                };

                if UNSUPPORTED_DATA_TYPES.contains(&argument_type) {
                    found_unsupported_data_type = true;
                    break;
                }

                let argument_name = if let Some(pos) = argument.find(" ") {
                    argument[..pos].replace("\"", "")
                } else {
                    format!("arg{}", i)
                };

                sql_function.arguments.push((
                    argument_name,
                    postgres_type_to_diesel(&argument_type, false),
                ));
            }

            if found_unsupported_data_type || UNSUPPORTED_DATA_TYPES.contains(&return_type.as_str())
            {
                continue;
            }

            if !return_type.is_empty() && return_type != "void" {
                sql_function.return_type = Some(postgres_type_to_diesel(&return_type, false));
            }
            sql_functions.push(sql_function);
        }

        Ok(sql_functions)
    }

    pub fn to_syn(&self) -> TokenStream {
        let function_name = Ident::new(&self.name, proc_macro2::Span::call_site());
        let arguments = self.arguments.iter().map(|(name, ty)| {
            let name = Ident::new(name, proc_macro2::Span::call_site());
            quote! { #name: #ty }
        });

        match &self.return_type {
            Some(return_type) => {
                quote! {
                    diesel::define_sql_function! {
                        fn #function_name(#(#arguments),*) -> #return_type;
                    }
                }
            }
            None => {
                quote! {
                    diesel::define_sql_function! {
                        fn #function_name(#(#arguments),*);
                    }
                }
            }
        }
    }

    pub fn write_all(conn: &mut PgConnection, output_path: &str) -> Result<(), WebCodeGenError> {
        let functions = Self::load_all(conn)?;

        // We convert the functions to TokenStream
        let functions = functions.iter().map(|f| f.to_syn());

        // Create a new TokenStream
        let output = quote! {
            #( #functions )*
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
