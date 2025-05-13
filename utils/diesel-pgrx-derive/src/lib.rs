#![doc = include_str!("../README.md")]

use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Type};

#[proc_macro_derive(DieselPGRX)]
/// Derives the `DieselPGRX` trait for the given struct or enum.
pub fn diesel_pgrx_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let ident_as_string = ident.to_string().to_lowercase();

    // We retrieve the list of attributes from the struct.
    let attribute_names: Vec<Ident> = if let syn::Data::Struct(data) = &data {
        data.fields.iter().map(|field| field.ident.clone().unwrap()).collect()
    } else {
        panic!("DieselPGRX can only be derived for structs, not enums");
    };

    let attribute_types: Vec<Type> = if let syn::Data::Struct(data) = &data {
        data.fields.iter().map(|field| field.ty.clone()).collect()
    } else {
        panic!("DieselPGRX can only be derived for structs, not enums");
    };

    let diesel_attribute_types: Vec<_> = attribute_types.iter().map(|ty| {
        match ty {
            syn::Type::Path(type_path) => {
                let segment = &type_path.path.segments.last().unwrap().ident;
                match segment.to_string().as_str() {
                    "String" => quote! { diesel_pgrx::diesel::sql_types::Text },
                    "i32" => quote! { diesel_pgrx::diesel::sql_types::Integer },
                    "f64" => quote! { diesel_pgrx::diesel::sql_types::Double },
                    _ => panic!("Unsupported type: {}", segment),
                }
            },
            _ => panic!("Unsupported type format"),
        }
    }).collect();

    quote! {
        impl diesel_pgrx::DieselPGRX for #ident {}

        pub mod diesel_impls {
            #[derive(
                Debug, Clone, Copy, Default, diesel_pgrx::diesel::query_builder::QueryId, diesel_pgrx::diesel::sql_types::SqlType,
            )]
            #[cfg_attr(
                feature = "postgres",
                diesel(postgres_type(name = #ident_as_string))
            )]
            #[cfg_attr(feature = "sqlite", diesel(sqlite_type(name = "TEXT")))]
            pub struct #ident;
        }

        #[cfg(feature = "postgres")]
        impl diesel_pgrx::diesel::serialize::ToSql<diesel_impls::#ident, diesel_pgrx::diesel::pg::Pg> for #ident
        {
            fn to_sql<'b>(&'b self, out: &mut diesel_pgrx::diesel::serialize::Output<'b, '_, diesel_pgrx::diesel::pg::Pg>) -> diesel_pgrx::diesel::serialize::Result {
                use std::io::Write;

                <
                    (#(#attribute_types),*) as diesel_pgrx::diesel::serialize::ToSql<
                        (#(#diesel_attribute_types),*),
                        diesel_pgrx::diesel::pg::Pg
                    >
                >::to_sql(
                    &(
                        #(
                            self.#attribute_names
                        ),*
                    ),
                    out,
                )?; 
                Ok(diesel::serialize::IsNull::No)
            }
        }

        #[cfg(feature = "postgres")]
        impl diesel_pgrx::diesel::deserialize::FromSql<diesel_impls::#ident, diesel_pgrx::diesel::pg::Pg> for #ident
        {
            fn from_sql(raw: diesel_pgrx::diesel::pg::PgValue<'_>) -> diesel_pgrx::diesel::deserialize::Result<Self> {
                use diesel_pgrx::diesel::deserialize::FromSql;
                use diesel_pgrx::diesel::pg::PgValue;

                let ( #(#attribute_names),* ) = <(#(#attribute_types),*) as diesel_pgrx::diesel::deserialize::FromSql<
                    (#(#diesel_attribute_types),*),
                    diesel_pgrx::diesel::pg::Pg
                >>::from_sql(raw)?;

                Ok(#ident {
                    #(
                        #attribute_names,
                    )*
                })
            }
        }
    }.into()
}
