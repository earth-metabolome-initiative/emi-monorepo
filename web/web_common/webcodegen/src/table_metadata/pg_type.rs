//! Submodule providing the `PgType` struct and associated methods.

use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, Queryable,
    QueryableByName, RunQueryDsl, Selectable, SelectableHelper,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;
use syn::{parse_str, Type};

use crate::errors::WebCodeGenError;

use super::{PgAttribute, PgEnum};

/// Constant listing types supporting `Copy`.
const COPY_TYPES: [&str; 6] = ["i32", "i16", "i64", "f32", "f64", "bool"];

/// Constant listing types supporting `Eq`.
const EQ_TYPES: [&str; 4] = ["i32", "i16", "i64", "bool"];

/// Constant listing types supporting `Hash`.
const HASH_TYPES: [&str; 4] = ["i32", "i16", "i64", "bool"];

#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Hash, Clone)]
#[diesel(table_name = crate::schema::pg_type)]
pub struct PgType {
    pub oid: u32,
    pub typname: String,
    pub typnamespace: u32,
    pub typowner: u32,
    pub typlen: i16,
    pub typbyval: bool,
    pub typtype: String,
    pub typcategory: String,
    pub typispreferred: bool,
    pub typisdefined: bool,
    pub typdelim: String,
    pub typrelid: u32,
    #[cfg(feature = "postgres_17")]
    pub typsubscript: u32,
    pub typelem: u32,
    pub typarray: u32,
    pub typinput: u32,
    pub typoutput: u32,
    pub typreceive: u32,
    pub typsend: u32,
    pub typmodin: u32,
    pub typmodout: u32,
    pub typanalyze: u32,
    pub typalign: String,
    pub typstorage: String,
    pub typnotnull: bool,
    pub typbasetype: u32,
    pub typtypmod: i32,
    pub typndims: i32,
    pub typcollation: u32,
    pub typdefaultbin: Option<Vec<u8>>,
    pub typdefault: Option<String>,
}

/// Returns the Syn rust type of the column.
pub fn rust_type_str<S: AsRef<str>>(type_name: S) -> &'static str {
    match type_name.as_ref() {
        "integer" => "i32",
        "character varying" => "String",
        "text" => "String",
        "name" => "String",
        "timestamp with time zone" => "chrono::NaiveDateTime",
        "timestamp without time zone" => "chrono::NaiveDateTime",
        "date" => "chrono::NaiveDate",
        "boolean" => "bool",
        "numeric" => "f64",
        "uuid" => "uuid::Uuid",
        "jsonb" => "serde_json::Value",
        "json" => "serde_json::Value",
        "bytea" => "Vec<u8>",
        "inet" => "std::net::IpAddr",
        "time without time zone" => "chrono::NaiveTime",
        "time with time zone" => "chrono::NaiveTime",
        "interval" => "chrono::Duration",
        "bit" => "Vec<u8>",
        "bit varying" => "Vec<u8>",
        "money" => "BigDecimal",
        "xml" => "String",
        "oid" => "u32",
        "smallint" => "i16",
        "bigint" => "i64",
        "real" => "f32",
        "double precision" => "f64",
        "float8" => "f64",
        "character" => "String",
        "char" => "String",
        "cidr" => "std::net::IpAddr",
        "macaddr" => "std::net::MacAddr",
        "macaddr8" => "std::net::MacAddr",
        "point" => "Point",
        "line" => "Line",
        "lseg" => "LineSegment",
        other => todo!("Unsupported data type: {}", other),
    }
}

pub fn postgres_type_to_diesel(postgres_type: &str, nullable: bool) -> Type {
    let rust_type_str = match postgres_type {
        "integer" => "diesel::sql_types::Integer",
        "text" => "diesel::sql_types::Text",
        "timestamp without time zone" => "diesel::sql_types::Timestamp",
        "timestamp with time zone" => "diesel::sql_types::Timestamptz",
        "timestamptz" => "diesel::sql_types::Timestamptz",
        "timestamp" => "diesel::sql_types::Timestamp",
        "time" => "diesel::sql_types::Time",
        "uuid" => "diesel::sql_types::Uuid",
        "boolean" => "diesel::sql_types::Bool",
        "bool" => "diesel::sql_types::Bool",
        "real" => "diesel::sql_types::Float",
        "name" => "diesel::sql_types::Text",
        "double precision" => "diesel::sql_types::Double",
        "character varying" => "diesel::sql_types::Text",
        "char" => "diesel::sql_types::CChar",
        "bpchar" => "diesel::sql_types::Bpchar",
        "bytea" => "diesel::sql_types::Bytea",
        "json" => "diesel::sql_types::Json",
        "jsonb" => "diesel::sql_types::Jsonb",
        "macaddr" => "diesel::sql_types::Macaddr",
        "inet" => "diesel::sql_types::Inet",
        "oid" => "diesel::sql_types::Oid",
        "int2" => "diesel::sql_types::SmallInt",
        "int4" => "diesel::sql_types::Integer",
        "int8" => "diesel::sql_types::BigInt",
        "float4" => "diesel::sql_types::Float",
        "float8" => "diesel::sql_types::Double",
        "tsvector" => "diesel_full_text_search::TsVector",
        "tsquery" => "diesel_full_text_search::TsQuery",
        "money" => "diesel::pg::sql_types::Money",
        "smallint" => "diesel::sql_types::SmallInt",
        "bigint" => "diesel::sql_types::BigInt",
        "cstring" => "diesel::sql_types::Text",
        "interval" => "diesel::sql_types::Interval",
        "date" => "diesel::sql_types::Date",
        "geometry" | "geography" => "postgis_diesel::sql_types::Geometry",
        "point" => "postgis_diesel::sql_types::Geometry",
        "polygon" => "postgis_diesel::sql_types::Geometry",
        "geometry(Point,4326)" => "postgis_diesel::sql_types::Geometry",
        "line" => "postgis_diesel::sql_types::Geometry",
        _ => todo!("Unsupported data type: '{}'", postgres_type),
    };

    let rust_type_str = if nullable {
        format!("diesel::sql_types::Nullable<{}>", rust_type_str)
    } else {
        rust_type_str.to_string()
    };

    parse_str::<Type>(&rust_type_str)
        .expect(format!("Failed to parse rust type: '{}'", rust_type_str).as_str())
}

impl PgType {
    /// Returns the Syn rust type of the PgType.
    pub fn rust_type(&self, conn: &mut PgConnection) -> Result<Type, WebCodeGenError> {
        if self.is_composite() {
            let struct_name = Ident::new(&self.camelcased_name(), proc_macro2::Span::call_site());
            Ok(parse_str::<Type>(&struct_name.to_string()).unwrap())
        } else if self.is_user_defined(conn)? {
            let base_type = self.base_type(conn).unwrap();
            let base_type = if let Some(base_type) = base_type {
                base_type
            } else {
                return Err(WebCodeGenError::MissingBaseType(self.clone()));
            };
            base_type.rust_type(conn)
        } else {
            Ok(parse_str(rust_type_str(&self.typname)).unwrap())
        }
    }

    /// Returns the Syn postgres type of the PgType.
    pub fn diesel_type(
        &self,
        nullable: bool,
        conn: &mut PgConnection,
    ) -> Result<Type, WebCodeGenError> {
        if self.is_composite() {
            Ok(parse_str::<Type>(&format!("crate::Pg{}", &self.camelcased_name())).unwrap())
        } else if self.is_user_defined(conn)? {
            let base_type = self.base_type(conn)?;
            if let Some(base_type) = base_type {
                base_type.diesel_type(nullable, conn)
            } else {
                return Err(WebCodeGenError::MissingBaseType(self.clone()));
            }
        } else {
            Ok(postgres_type_to_diesel(&self.typname, nullable))
        }
    }

    /// Returns the internal custom types of the PgType, if any.
    pub fn internal_custom_types(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<PgType>, WebCodeGenError> {
        let mut internal_custom_types = Vec::new();
        for attribute in self.attributes(conn)? {
            let pg_type = attribute.pg_type(conn)?;
            if pg_type.is_composite() || pg_type.is_enum(){
                internal_custom_types.extend(pg_type.internal_custom_types(conn)?);
                internal_custom_types.push(pg_type);
            }
        }

        Ok(internal_custom_types)
    }

    /// Returns the Type Base Type of the PgType.
    pub fn base_type(&self, conn: &mut PgConnection) -> Result<Option<PgType>, WebCodeGenError> {
        if self.typbasetype == 0 {
            Ok(None)
        } else {
            use crate::schema::pg_type;
            Ok(pg_type::dsl::pg_type
                .filter(pg_type::dsl::oid.eq(self.typbasetype))
                .first::<PgType>(conn)
                .ok())
        }
    }

    /// Returns whether the Postgres type is a user-defined type.
    pub fn is_user_defined(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        Ok(&self.typcategory == "U" && self.base_type(conn)?.is_some())
    }

    /// Returns whether the Postgres type is a composite type.
    pub fn is_composite(&self) -> bool {
        &self.typcategory == "C"
    }

    /// Returns whether the Postgres type is an enum type.
    pub fn is_enum(&self) -> bool {
        &self.typcategory == "E"
    }

    /// Returns whether the associated rust type supports `Copy`.
    pub fn supports_copy(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        if self.is_composite() {
            self.attributes(conn)?
                .into_iter()
                .fold(Ok(true), |acc, attribute| {
                    acc.and_then(|acc| attribute.supports_copy(conn).map(|b| acc && b))
                })
        } else if self.is_user_defined(conn)? {
            self.base_type(conn)?
                .ok_or(WebCodeGenError::MissingBaseType(self.clone()))
                .and_then(|base_type| base_type.supports_copy(conn))
        } else {
            Ok(COPY_TYPES.contains(&rust_type_str(&self.typname)))
        }
    }

    /// Returns whether the associated rust type supports `Hash`.
    pub fn supports_hash(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        if self.is_user_defined(conn)? || self.is_composite() {
            self.attributes(conn)?
                .into_iter()
                .fold(Ok(true), |acc, attribute| {
                    acc.and_then(|acc| attribute.supports_hash(conn).map(|b| acc && b))
                })
        } else {
            Ok(HASH_TYPES.contains(&rust_type_str(&self.typname)))
        }
    }

    /// Returns whether the associated rust type supports `Eq`.
    pub fn supports_eq(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        if self.is_user_defined(conn)? || self.is_composite() {
            self.attributes(conn)?
                .into_iter()
                .fold(Ok(true), |acc, attribute| {
                    acc.and_then(|acc| attribute.supports_eq(conn).map(|b| acc && b))
                })
        } else {
            Ok(EQ_TYPES.contains(&rust_type_str(&self.typname)))
        }
    }

    /// Returns the CamelCased name of the PgType.
    pub fn camelcased_name(&self) -> String {
        self.typname
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

    /// Returns the CamelCased name of the PgType for the Postgres binding.
    pub fn pg_binding_name(&self) -> String {
        format!("Pg{}", self.camelcased_name())
    }

    /// Returns the syn of the struct or enum associated to the PgType.
    pub fn to_syn(&self, conn: &mut PgConnection) -> Result<TokenStream, WebCodeGenError> {
        if self.is_composite() {
            let struct_name = Ident::new(&self.camelcased_name(), proc_macro2::Span::call_site());
            let mut fields = Vec::new();
            let mut diesel_types = Vec::new();
            let mut rust_types = Vec::new();
            let mut struct_attributes = Vec::new();
            let mut field_names = Vec::new();
            let attributes = self.attributes(conn)?;
            for attribute in attributes.iter() {
                let field_name = Ident::new(&attribute.attname, proc_macro2::Span::call_site());
                let field_pg_type = attribute.pg_type(conn)?;
                let field_type = field_pg_type.rust_type(conn)?;
                field_names.push(quote! {
                    #field_name
                });
                rust_types.push(quote! {
                    #field_type
                });
                let diesel_type = field_pg_type.diesel_type(attribute.attnotnull, conn)?;
                if field_pg_type.supports_copy(conn)? || attributes.len() == 1 {
                    struct_attributes.push(quote! {
                        self.#field_name
                    });
                } else {
                    struct_attributes.push(quote! {
                        self.#field_name.clone()
                    });
                }

                fields.push(quote! {
                    pub #field_name: #field_type
                });
                diesel_types.push(quote! {
                    #diesel_type
                });
            }

            let this_typname: &str = &self.typname;
            let postgres_struct_name =
                Ident::new(&self.pg_binding_name(), proc_macro2::Span::call_site());

            let mut derives = vec![
                Ident::new("Debug", proc_macro2::Span::call_site()),
                Ident::new("Clone", proc_macro2::Span::call_site()),
                Ident::new("PartialEq", proc_macro2::Span::call_site()),
            ];

            if self.supports_eq(conn)? {
                derives.push(Ident::new("Eq", proc_macro2::Span::call_site()));
            }

            if self.supports_hash(conn)? {
                derives.push(Ident::new("Hash", proc_macro2::Span::call_site()));
            }

            if self.supports_copy(conn)? {
                derives.push(Ident::new("Copy", proc_macro2::Span::call_site()));
            }

            let to_sql_operation = if diesel_types.len() > 1 {
                quote! {
                    diesel::serialize::WriteTuple::<(#(#diesel_types),*)>::write_tuple(
                        &(#(#struct_attributes),*),
                        &mut out.reborrow(),
                    )
                }
            } else {
                quote! {
                    diesel::serialize::ToSql::<#(#diesel_types)*, diesel::pg::Pg>::to_sql(
                        &#(#struct_attributes)*,
                        out,
                    )
                }
            };

            let from_sql_ops = if diesel_types.len() > 1 {
                quote! {
                    let (#(#field_names),*): (#(#rust_types),*) = diesel::deserialize::FromSql::<diesel::sql_types::Record<(#(#diesel_types),*)>, diesel::pg::Pg>::from_sql(bytes)?;
                    Ok(Self {
                        #(#field_names),*
                    })
                }
            } else {
                quote! {
                    let #(#field_names)*: #(#rust_types),* = diesel::deserialize::FromSql::<#(#diesel_types)*, diesel::pg::Pg>::from_sql(bytes)?;
                    Ok(Self {
                        #(#field_names),*
                    })
                }
            };

            Ok(quote! {
                #[cfg(feature = "diesel")]
                #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
                #[diesel(postgres_type(name = #this_typname))]
                pub struct #postgres_struct_name;

                #[derive(#(#derives),*)]
                #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                #[cfg_attr(feature = "diesel", derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression))]
                #[cfg_attr(feature = "diesel", diesel(sql_type = #postgres_struct_name))]
                pub struct #struct_name {
                    #(#fields),*
                }

                #[cfg(feature = "diesel")]
                impl diesel::serialize::ToSql<#postgres_struct_name, diesel::pg::Pg> for #struct_name {
                    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>) -> diesel::serialize::Result {
                        #to_sql_operation
                    }
                }

                #[cfg(feature = "diesel")]
                impl diesel::deserialize::FromSql<#postgres_struct_name, diesel::pg::Pg> for #struct_name {
                    fn from_sql(
                        bytes: <diesel::pg::Pg as diesel::backend::Backend>::RawValue<'_>,
                    ) -> diesel::deserialize::Result<Self> {
                        #from_sql_ops
                    }
                }
            })
        } else if self.is_enum() {
            let struct_name = Ident::new(&self.camelcased_name(), proc_macro2::Span::call_site());
            let variants = self.variants(conn)?;
            let mut variant_names = Vec::new();
            let mut in_variants = Vec::new();
            let mut out_variants = Vec::new();
            for variant in variants.iter() {
                let variant_name = Ident::new(&variant.enumlabel, proc_macro2::Span::call_site());
                variant_names.push(quote! {
                    #variant_name
                });
                let variant = variant.enumlabel.clone();
                in_variants.push(quote! {
                    #variant => Ok(#struct_name::#variant_name),
                });
                out_variants.push(quote! {
                    #struct_name::#variant_name => std::io::Write::write_all(out, #variant.as_bytes())?,         
                });
            }

            let this_typname: &str = &self.typname;
            let postgres_enum_name =
                Ident::new(&self.pg_binding_name(), proc_macro2::Span::call_site());

            Ok(quote! {
                #[cfg(feature = "diesel")]
                #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
                #[diesel(postgres_type(name = #this_typname))]
                pub struct #postgres_enum_name;

                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                #[cfg_attr(feature = "diesel", derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression))]
                #[cfg_attr(feature = "diesel", diesel(sql_type = #postgres_enum_name))]
                pub enum #struct_name {
                    #(#variant_names),*
                }

                #[cfg(feature = "diesel")]
                impl diesel::serialize::ToSql<#postgres_enum_name, diesel::pg::Pg> for #struct_name {
                    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>) -> diesel::serialize::Result {
                        match *self {
                            #(#out_variants)*
                        }
                        Ok(diesel::serialize::IsNull::No)
                    }
                }

                #[cfg(feature = "diesel")]
                impl diesel::deserialize::FromSql<#postgres_enum_name, diesel::pg::Pg> for #struct_name {
                    fn from_sql(
                        bytes: <diesel::pg::Pg as diesel::backend::Backend>::RawValue<'_>,
                    ) -> diesel::deserialize::Result<Self> {
                        let s: String = diesel::deserialize::FromSql::<diesel::sql_types::Text, diesel::pg::Pg>::from_sql(bytes)?;
                        match s.as_str() {
                            #(#in_variants)*
                            unknown => Err(format!("Unknown variant: {}", unknown).into()),
                        }
                    }
                }
            })

        } else {
            panic!("Unsupported type: {:?}", self);
        }
    }

    /// Returns a new PgType struct from the given type name.
    ///
    /// # Arguments
    ///
    /// * `type_name` - The name of the type.
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing the PgType struct if the type exists, or an error if it does not.
    ///
    pub fn from_name(type_name: &str, conn: &mut PgConnection) -> Result<Self, WebCodeGenError> {
        use crate::schema::pg_type;
        Ok(pg_type::dsl::pg_type
            .filter(pg_type::dsl::typname.eq(type_name))
            .first::<PgType>(conn)?)
    }

    /// Returns the attributes of the type, if it is a composite type.
    pub fn attributes(&self, conn: &mut PgConnection) -> Result<Vec<PgAttribute>, WebCodeGenError> {
        use crate::schema::pg_attribute;
        use crate::schema::pg_type;

        Ok(pg_attribute::dsl::pg_attribute
            .inner_join(
                pg_type::dsl::pg_type.on(pg_attribute::dsl::attrelid.eq(pg_type::dsl::typrelid)),
            )
            .filter(
                pg_type::dsl::typname
                    .eq(&self.typname)
                    .and(pg_attribute::dsl::attisdropped.eq(false)),
            )
            .select(PgAttribute::as_select())
            .load::<PgAttribute>(conn)?)
    }

    /// Returns the variants of the type, if it is an enum type.
    pub fn variants(&self, conn: &mut PgConnection) -> Result<Vec<PgEnum>, WebCodeGenError> {
        use crate::schema::pg_enum;

        Ok(pg_enum::dsl::pg_enum
            .filter(pg_enum::dsl::enumtypid.eq(self.oid))
            .order_by(pg_enum::dsl::enumsortorder)
            .select(PgEnum::as_select())
            .load::<PgEnum>(conn)?)
    }
}
