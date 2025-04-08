//! Submodule providing the [`PgType`] struct and associated methods.

use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, Queryable,
    QueryableByName, RunQueryDsl, Selectable, SelectableHelper,
};
use proc_macro2::TokenStream;
use quote::quote;
use snake_case_sanitizer::Sanitizer as SnakeCaseSanizer;
use syn::{parse_str, Ident, Type};

use super::{table::RESERVED_RUST_WORDS, PgAttribute, PgSetting, PgEnum};
use crate::{
    codegen::{
        CODEGEN_DIESEL_MODULE, CODEGEN_DIRECTORY, CODEGEN_STRUCTS_MODULE, CODEGEN_TYPES_PATH,
    },
    errors::WebCodeGenError,
};

/// Constant listing types supporting `Copy`.
pub(crate) const COPY_TYPES: [&str; 7] = ["i32", "i16", "i64", "f32", "f64", "bool", "rosetta_uuid::Uuid"];

/// Constant listing types supporting `Eq`.
pub(crate) const EQ_TYPES: [&str; 5] = ["i32", "i16", "i64", "bool", "rosetta_uuid::Uuid"];

/// Constant listing types supporting `Hash`.
pub(crate) const HASH_TYPES: [&str; 5] = ["i32", "i16", "i64", "bool", "rosetta_uuid::Uuid"];

/// Represents a `PostgreSQL` type.
///
/// This struct contains metadata about a `PostgreSQL` type, including its name,
/// OID (Object Identifier), namespace, and other properties.
#[derive(
    Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone,
)]
#[diesel(table_name = crate::schema::pg_type)]
#[allow(clippy::struct_excessive_bools)]
pub struct PgType {
    /// The OID (Object Identifier) of the type.
    pub oid: u32,
    /// The name of the type.
    pub typname: String,
    /// The namespace (schema) of the type.
    pub typnamespace: u32,
    /// The owner of the type.
    pub typowner: u32,
    /// The size of the type in bytes.
    pub typlen: i16,
    /// Indicates if the type is passed by value.
    pub typbyval: bool,
    /// The type of the type.
    pub typtype: String,
    /// The category of the type.
    pub typcategory: String,
    /// Indicates if the type is preferred within its category.
    pub typispreferred: bool,
    /// Indicates if the type is defined.
    pub typisdefined: bool,
    /// The delimiter for array elements of this type.
    pub typdelim: String,
    /// The relation ID for a composite type.
    pub typrelid: u32,
    /// The element type of an array type.
    pub typelem: u32,
    /// The array type of a base type.
    pub typarray: u32,
    /// The input function for the type.
    pub typinput: u32,
    /// The output function for the type.
    pub typoutput: u32,
    /// The receive function for the type.
    pub typreceive: u32,
    /// The send function for the type.
    pub typsend: u32,
    /// The modifier input function for the type.
    pub typmodin: u32,
    /// The modifier output function for the type.
    pub typmodout: u32,
    /// The analyze function for the type.
    pub typanalyze: u32,
    /// The alignment requirement of the type.
    pub typalign: String,
    /// The storage strategy for the type.
    pub typstorage: String,
    /// Indicates if the type is not nullable.
    pub typnotnull: bool,
    /// The base type of a domain type.
    pub typbasetype: u32,
    /// The type modifier.
    pub typtypmod: i32,
    /// The number of dimensions for an array type.
    pub typndims: i32,
    /// The collation for the type.
    pub typcollation: u32,
    /// The default binary representation of the type.
    pub typdefaultbin: Option<Vec<u8>>,
    /// The default text representation of the type.
    pub typdefault: Option<String>,
}

/// Returns the Syn rust type of the column.
///
/// # Arguments
///
/// * `type_name` - The name of the type.
///
/// # Returns
///
/// A string slice containing the rust type.
///
/// # Panics
///
/// Panics if the type is not supported.
pub fn rust_type_str<S: AsRef<str>>(type_name: S, conn: &mut PgConnection) -> Result<&'static str, WebCodeGenError> {
    Ok(match type_name.as_ref() {
        // Numeric types
        "integer" => "i32",
        "smallint" => "i16",
        "bigint" => "i64",
        "real" | "float4" => "f32",
        "double precision" | "float8" | "numeric" => "f64",
        "money" => "BigDecimal",
        "oid" => "u32",

        // Text types
        "character varying" | "text" | "name" | "xml" | "character" | "char" | "citext" => "String",

        // Boolean types
        "boolean" | "bool" => "bool",

        // Temporal types
        "timestamp without time zone" => "chrono::NaiveDateTime",
        "timestamp with time zone" => {
            let time_zone = PgSetting::time_zone(conn)?;
            match time_zone.setting.as_str() {
                "UTC" => "chrono::DateTime<chrono::Utc>",
                unknown_time_zone => unimplemented!("Time zone `{unknown_time_zone}` not supported"),
            }
        },
        "date" => "chrono::NaiveDate",
        "time without time zone" | "time with time zone" => "chrono::NaiveTime",
        "interval" => "chrono::Duration",

        // Binary types
        "bytea" | "bit" | "bit varying" => "Vec<u8>",

        // JSON types
        "json" | "jsonb" => "serde_json::Value",

        // Network address types
        "inet" | "cidr" => "std::net::IpAddr",
        "macaddr" | "macaddr8" => "std::net::MacAddr",

        // GIS types
        "point" => "postgis_diesel::types::Point",
        "geometry" => panic!("Geometry type not supported"),

        // UUID type
        "uuid" => "rosetta_uuid::Uuid",

        other => return Err(WebCodeGenError::UnknownPostgresRustType(other.to_owned())),
    })
}

/// Converts a `PostgreSQL` type to a `Diesel` type.
///
/// # Arguments
///
/// * `postgres_type` - A string slice that holds the `PostgreSQL` type name.
///
/// # Returns
///
/// A `Type` representing the corresponding Diesel type.
///
/// # Errors
///
/// * Returns an error if the type is not supported.
pub fn postgres_type_to_diesel_str(postgres_type: &str) -> Result<String, WebCodeGenError> {
    if let Some(postgres_type) = postgres_type.strip_suffix("[]") {
        return Ok(format!(
            "diesel::sql_types::Array<{}>",
            postgres_type_to_diesel_str(postgres_type)?
        ));
    }

    Ok(match postgres_type {
        // Numeric types
        "integer" | "i32" => "diesel::sql_types::Integer",
        "smallint" | "int2" => "diesel::sql_types::SmallInt",
        "bigint" | "int8" => "diesel::sql_types::BigInt",
        "real" | "float4" => "diesel::sql_types::Float",
        "double precision" | "float8" => "diesel::sql_types::Double",
        "money" => "diesel::pg::sql_types::Money",

        // Text types
        "text" | "character varying" | "name" | "cstring" => "diesel::sql_types::Text",
        "citext" => "diesel::sql_types::Citext",
        "char" | "character" => "diesel::sql_types::CChar",
        "bpchar" => "diesel::sql_types::Bpchar",

        // Boolean types
        "boolean" | "bool" => "diesel::sql_types::Bool",

        // Temporal types
        "timestamp without time zone" | "timestamp" => "diesel::sql_types::Timestamp",
        "timestamp with time zone" | "timestamptz" => "diesel::sql_types::Timestamptz",
        "time" => "diesel::sql_types::Time",
        "date" => "diesel::sql_types::Date",
        "interval" => "diesel::sql_types::Interval",

        // Binary types
        "bytea" => "diesel::sql_types::Binary",

        // JSON types
        "json" => "diesel::sql_types::Json",
        "jsonb" => "diesel::sql_types::Jsonb",

        // Network address types
        "macaddr" => "diesel::sql_types::Macaddr",
        "inet" => "diesel::sql_types::Inet",

        // Object Identifier types
        "oid" => "diesel::sql_types::Oid",

        // Full-text search types
        "tsvector" => "diesel_full_text_search::TsVector",
        "tsquery" => "diesel_full_text_search::TsQuery",

        // GIS types
        "geometry" | "point" | "polygon" | "geometry(Point,4326)" | "line" => {
            "postgis_diesel::sql_types::Geometry"
        }
        "geography" => "postgis_diesel::sql_types::Geography",

        // Other
        "uuid" => "rosetta_uuid::diesel_impls::Uuid",

        _ => {
            return Err(WebCodeGenError::UnknownDieselPostgresType(postgres_type.to_owned()));
        }
    }
    .to_owned())
}

/// Converts a `PostgreSQL` type to a `Diesel` type.
///
/// # Arguments
///
/// * `postgres_type` - A string slice that holds the `PostgreSQL` type name.
/// * `nullable` - A boolean indicating whether the type is nullable.
///
/// # Returns
///
/// A `Type` representing the corresponding Diesel type.
///
/// # Errors
///
/// * Returns an error if the type is not supported.
pub fn postgres_type_to_diesel(
    postgres_type: &str,
    nullable: bool,
) -> Result<Type, WebCodeGenError> {
    let rust_type_str = postgres_type_to_diesel_str(postgres_type)?;

    let rust_type_str = if nullable {
        format!("diesel::sql_types::Nullable<{rust_type_str}>")
    } else {
        rust_type_str.to_string()
    };

    Ok(parse_str::<Type>(&rust_type_str)?)
}

impl PgType {
    /// Returns the Syn rust type of the `PgType`.
    ///
    /// # Arguments
    ///
    /// * `optional` - A boolean indicating whether the type is optional.
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing the Syn rust type of the `PgType`, or an error if
    /// the type is not supported.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn rust_type(
        &self,
        optional: bool,
        conn: &mut PgConnection,
    ) -> Result<Type, WebCodeGenError> {
        match rust_type_str(&self.typname, conn) {
            Ok(rust_type) => Ok(parse_str::<Type>(rust_type)?),
            Err(error) => {
                if self.is_composite() || self.is_enum() {
                    let mut struct_name = format!(
                        "crate::{CODEGEN_DIRECTORY}::{CODEGEN_STRUCTS_MODULE}::{CODEGEN_TYPES_PATH}::{}::{}",
                        self.snake_case_name()?,
                        self.camelcased_name()
                    );
                    if optional {
                        struct_name = format!("Option<{struct_name}>");
                    }

                    Ok(parse_str::<Type>(&struct_name)?)
                } else if self.is_user_defined(conn)? {
                    let Some(base_type) = self.base_type(conn)? else {
                        return Err(WebCodeGenError::MissingBaseType(Box::new(self.clone())));
                    };
                    base_type.rust_type(optional, conn)
                } else {
                    Err(error)
                }
            }
        }
    }

    /// Returns the Syn postgres type of the `PgType`.
    ///
    /// # Arguments
    ///
    /// * `nullable` - A boolean indicating whether the type is nullable.
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing the Syn postgres type of the `PgType`, or an error
    /// if the type is not supported.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn diesel_type(
        &self,
        nullable: bool,
        conn: &mut PgConnection,
    ) -> Result<Type, WebCodeGenError> {
        let diesel_type = postgres_type_to_diesel(&self.typname, nullable);
        match diesel_type {
            Ok(diesel_type) => Ok(diesel_type),
            Err(error) => {
                if self.is_composite() || self.is_enum() {
                    let snake_case_name = self.snake_case_name()?;
                    let mut full_name = format!(
                        "crate::{CODEGEN_DIRECTORY}::{CODEGEN_DIESEL_MODULE}::{CODEGEN_TYPES_PATH}::{snake_case_name}::{}",
                        &self.pg_binding_name()
                    );
                    if nullable {
                        full_name = format!("diesel::sql_types::Nullable<{full_name}>");
                    }
                    Ok(parse_str::<Type>(&full_name)?)
                } else if self.is_user_defined(conn)? {
                    let base_type = self.base_type(conn)?;
                    if let Some(base_type) = base_type {
                        base_type.diesel_type(nullable, conn)
                    } else {
                        Err(WebCodeGenError::MissingBaseType(Box::new(self.clone())))
                    }
                } else {
                    Err(error)
                }
            }
        }
    }

    /// Returns the internal custom types of the `PgType`, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing the internal custom types of the `PgType`, or an
    /// error if the type is not supported.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn internal_custom_types(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<PgType>, WebCodeGenError> {
        let mut internal_custom_types = Vec::new();
        for attribute in self.attributes(conn)? {
            let pg_type = attribute.pg_type(conn)?;
            if pg_type.is_composite() || pg_type.is_enum() {
                internal_custom_types.extend(pg_type.internal_custom_types(conn)?);
                internal_custom_types.push(pg_type);
            }
        }

        Ok(internal_custom_types)
    }

    /// Returns the Type Base Type of the `PgType`.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing the Type Base Type of the `PgType`, or an error if
    /// the type is not supported.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn base_type(&self, conn: &mut PgConnection) -> Result<Option<PgType>, WebCodeGenError> {
        if self.typbasetype == 0 {
            Ok(None)
        } else {
            use crate::schema::pg_type;
            Ok(pg_type::table.filter(pg_type::oid.eq(self.typbasetype)).first::<PgType>(conn).ok())
        }
    }

    /// Returns the sanitized snake case name of the table.
    ///
    /// # Errors
    ///
    /// * If the snake case name cannot be generated.
    ///
    /// # Returns
    ///
    /// A string representing the sanitized snake case name of the table.
    pub fn snake_case_name(&self) -> Result<String, WebCodeGenError> {
        let sanitizer = SnakeCaseSanizer::default()
            .include_defaults()
            .remove_leading_underscores()
            .remove_trailing_underscores();
        Ok(sanitizer.to_snake_case(&self.typname)?)
    }

    /// Returns the sanitized snake case identifier of the table.
    ///
    /// # Errors
    ///
    /// * If the snake case identifier cannot be generated.
    pub fn snake_case_identifier(&self) -> Result<Ident, WebCodeGenError> {
        let snake_case_name = self.snake_case_name()?;
        if RESERVED_RUST_WORDS.contains(&snake_case_name.as_str()) {
            Ok(Ident::new_raw(&snake_case_name, proc_macro2::Span::call_site()))
        } else {
            Ok(Ident::new(&snake_case_name, proc_macro2::Span::call_site()))
        }
    }

    /// Returns whether the Postgres type is a user-defined type.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing a boolean indicating whether the Postgres type is a
    /// user-defined type, or an error if the type is not supported.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn is_user_defined(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        Ok(&self.typcategory == "U" && self.base_type(conn)?.is_some())
    }

    #[must_use]
    /// Returns whether the Postgres type is a composite type.
    pub fn is_composite(&self) -> bool {
        &self.typcategory == "C"
    }

    #[must_use]
    /// Returns whether the Postgres type is an enum type.
    pub fn is_enum(&self) -> bool {
        &self.typcategory == "E"
    }

    /// Returns whether the associated rust type supports `Copy`.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing a boolean indicating whether the associated rust
    /// type supports `Copy`, or an error if the type is not supported.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn supports_copy(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        if self.is_composite() {
            self.attributes(conn)?
                .into_iter()
                .try_fold(true, |acc, attribute| attribute.supports_copy(conn).map(|b| acc && b))
        } else if self.is_user_defined(conn)? {
            self.base_type(conn)?
                .ok_or(WebCodeGenError::MissingBaseType(Box::new(self.clone())))
                .and_then(|base_type| base_type.supports_copy(conn))
        } else {
            Ok(COPY_TYPES.contains(&rust_type_str(&self.typname, conn)?))
        }
    }

    /// Returns whether the associated rust type supports `Hash`.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing a boolean indicating whether the associated rust
    /// type supports `Hash`, or an error if the type is not supported.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn supports_hash(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        if self.is_user_defined(conn)? || self.is_composite() {
            self.attributes(conn)?
                .into_iter()
                .try_fold(true, |acc, attribute| attribute.supports_hash(conn).map(|b| acc && b))
        } else {
            Ok(HASH_TYPES.contains(&rust_type_str(&self.typname, conn)?))
        }
    }

    /// Returns whether the associated rust type supports `Eq`.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing a boolean indicating whether the associated rust
    /// type supports `Eq`, or an error if the type is not supported.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn supports_eq(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        if self.is_user_defined(conn)? || self.is_composite() {
            self.attributes(conn)?
                .into_iter()
                .try_fold(true, |acc, attribute| attribute.supports_eq(conn).map(|b| acc && b))
        } else {
            Ok(EQ_TYPES.contains(&rust_type_str(&self.typname, conn)?))
        }
    }

    #[must_use]
    /// Returns the `CamelCased` name of the `PgType`.
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

    #[must_use]
    /// Returns the `CamelCased` name of the [`PgType`] for the Postgres
    /// binding.
    pub fn pg_binding_name(&self) -> String {
        format!("Pg{}", self.camelcased_name())
    }

    #[must_use]
    /// Returns the `CamelCased` Ident of the [`PgType`] for the Diesel binding.
    pub fn pg_binding_ident(&self) -> Ident {
        Ident::new(&self.pg_binding_name(), proc_macro2::Span::call_site())
    }

    /// Returns the syn-based struct or enum associated to the `PgType`.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing the syn of the struct or enum associated to the
    /// `PgType`, or an error if the type is not supported.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    ///
    /// # Panics
    ///
    /// * If it is unknown how to implement the associated struct or enum.
    pub fn to_struct_or_enum(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let struct_name = Ident::new(&self.camelcased_name(), proc_macro2::Span::call_site());
        let postgres_struct_name = self.diesel_type(false, conn)?;
        if self.is_composite() {
            let mut fields = Vec::new();
            let attributes = self.attributes(conn)?;
            for attribute in &attributes {
                let field_name = Ident::new(&attribute.attname, proc_macro2::Span::call_site());
                let field_pg_type = attribute.pg_type(conn)?;
                let field_type = field_pg_type.rust_type(false, conn)?;

                fields.push(quote! {
                    pub #field_name: #field_type
                });
            }

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

            Ok(quote! {
                #[derive(#(#derives),*)]
                #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                #[derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression)]
                #[diesel(sql_type = #postgres_struct_name)]
                pub struct #struct_name {
                    #(#fields),*
                }
            })
        } else if self.is_enum() {
            let variants = self.variants(conn)?;
            let mut variant_names = Vec::new();
            for variant in &variants {
                let variant_name = Ident::new(&variant.enumlabel, proc_macro2::Span::call_site());
                variant_names.push(quote! {
                    #variant_name
                });
            }

            Ok(quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                #[derive(diesel::deserialize::FromSqlRow, diesel::expression::AsExpression)]
                #[diesel(sql_type = #postgres_struct_name)]
                pub enum #struct_name {
                    #(#variant_names),*
                }
            })
        } else {
            panic!("Unsupported type: {self:?}");
        }
    }

    #[must_use]
    /// Returns the syn of the struct or enum associated to the `PgType`.
    ///
    /// # Returns
    ///
    /// A Result containing the syn of the struct or enum associated to the
    /// `PgType`, or an error if the type is not supported.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    ///
    /// # Panics
    ///
    /// * If it is unknown what type macros are needed.
    pub fn to_diesel_macro(&self) -> TokenStream {
        let postgres_struct_name = self.pg_binding_ident();
        let this_typname: &str = &self.typname;
        if self.is_composite() || self.is_enum() {
            quote! {
                #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
                #[diesel(postgres_type(name = #this_typname))]
                pub struct #postgres_struct_name;
            }
        } else {
            panic!("Unsupported type: {self:?}");
        }
    }

    /// Returns the syn of the traits necessary for diesel to support the
    /// conversion between the Postgres type and the Rust type.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing the syn of the struct or enum associated to the
    /// `PgType`, or an error if the type is not supported.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    ///
    /// # Panics
    ///
    /// * If it is unknown what type implementations are needed.
    pub fn to_diesel_impls(&self, conn: &mut PgConnection) -> Result<TokenStream, WebCodeGenError> {
        let diesel_struct_path = self.diesel_type(false, conn)?;
        let rust_struct_path = self.rust_type(false, conn)?;
        if self.is_composite() {
            let mut diesel_types = Vec::new();
            let mut rust_types = Vec::new();
            let mut struct_attributes = Vec::new();
            let mut field_names = Vec::new();
            let attributes = self.attributes(conn)?;
            for attribute in &attributes {
                let field_name = Ident::new(&attribute.attname, proc_macro2::Span::call_site());
                let field_pg_type = attribute.pg_type(conn)?;
                let field_type = field_pg_type.rust_type(false, conn)?;
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

                diesel_types.push(quote! {
                    #diesel_type
                });
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
                #[cfg(feature = "postgres")]
                impl diesel::serialize::ToSql<#diesel_struct_path, diesel::pg::Pg> for #rust_struct_path {
                    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>) -> diesel::serialize::Result {
                        #to_sql_operation
                    }
                }

                #[cfg(feature = "postgres")]
                impl diesel::deserialize::FromSql<#diesel_struct_path, diesel::pg::Pg> for #rust_struct_path {
                    fn from_sql(
                        bytes: <diesel::pg::Pg as diesel::backend::Backend>::RawValue<'_>,
                    ) -> diesel::deserialize::Result<Self> {
                        #from_sql_ops
                    }
                }
            })
        } else if self.is_enum() {
            let variants = self.variants(conn)?;
            let mut in_variants = Vec::new();
            let mut out_variants = Vec::new();
            for variant in &variants {
                let variant_name = Ident::new(&variant.enumlabel, proc_macro2::Span::call_site());
                let variant = variant.enumlabel.clone();
                in_variants.push(quote! {
                    #variant => Ok(Self::#variant_name),
                });
                out_variants.push(quote! {
                    Self::#variant_name => std::io::Write::write_all(out, #variant.as_bytes())?,
                });
            }

            Ok(quote! {
                #[cfg(feature = "postgres")]
                impl diesel::serialize::ToSql<#diesel_struct_path, diesel::pg::Pg> for #rust_struct_path {
                    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>) -> diesel::serialize::Result {
                        match *self {
                            #(#out_variants)*
                        }
                        Ok(diesel::serialize::IsNull::No)
                    }
                }

                #[cfg(feature = "postgres")]
                impl diesel::deserialize::FromSql<#diesel_struct_path, diesel::pg::Pg> for #rust_struct_path {
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
            panic!("Unsupported type: {self:?}");
        }
    }

    /// Returns a new [`PgType`] struct from the given type name.
    ///
    /// # Arguments
    ///
    /// * `type_name` - The name of the type.
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing the [`PgType`] struct if the type exists, or an
    /// error if it does not.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn from_name(type_name: &str, conn: &mut PgConnection) -> Result<Self, WebCodeGenError> {
        use crate::schema::pg_type;
        Ok(pg_type::table.filter(pg_type::typname.eq(type_name)).first::<PgType>(conn)?)
    }

    #[must_use]
    /// Returns whether the [`PgType`] is a boolean.
    pub fn is_boolean(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        Ok(rust_type_str(&self.typname, conn)? == "bool")
    }

    #[must_use]
    /// Returns whether the [`PgType`] is a text type.
    pub fn is_text(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        Ok(rust_type_str(&self.typname, conn)? == "String")
    }

    #[must_use]
    /// Returns whether the [`PgType`] is a numeric type.
    pub fn is_numeric(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        let rust_type = rust_type_str(&self.typname, conn)?;
        Ok(rust_type == "i32"
            || rust_type == "i16"
            || rust_type == "i64"
            || rust_type == "f32"
            || rust_type == "f64")
    }

    #[must_use]
    /// Returns the tokenstream of the provided string casted to the required
    /// type.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to cast.
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing the tokenstream of the provided string casted to the
    /// required type, or an error if the type is not supported.
    pub fn cast(&self, value: &str, conn: &mut PgConnection) -> Result<TokenStream, WebCodeGenError> {
        match rust_type_str(&self.typname, conn)? {
            "i16" => {
                let value = value.parse::<i16>()?;
                Ok(quote! {
                    #value
                })
            }
            "i32" => {
                let value = value.parse::<i32>()?;
                Ok(quote! {
                    #value
                })
            }
            "i64" => {
                let value = value.parse::<i64>()?;
                Ok(quote! {
                    #value
                })
            }
            "f32" => {
                let value = value.parse::<f32>()?;
                Ok(quote! {
                    #value
                })
            }
            "f64" => {
                let value = value.parse::<f64>()?;
                Ok(quote! {
                    #value
                })
            }
            "bool" => {
                let value = value.parse::<bool>()?;
                Ok(quote! {
                    #value
                })
            }
            _ => {
                Err(WebCodeGenError::UnsupportedTypeCasting(
                    value.to_owned(),
                    Box::new(self.clone()),
                ))
            }
        }
    }

    /// Returns the [`PgType`] struct from the given OID.
    ///
    /// # Arguments
    ///
    /// * `oid` - The OID of the type.
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing the [`PgType`] struct if the type exists, or an
    /// error if it does not.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn from_oid(oid: u32, conn: &mut PgConnection) -> Result<Self, diesel::result::Error> {
        use crate::schema::pg_type;
        Ok(pg_type::table.filter(pg_type::oid.eq(oid)).first::<PgType>(conn)?)
    }

    /// Returns the attributes of the type, if it is a composite type.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing the attributes of the type if it is a composite
    /// type, or an error if it is not.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn attributes(&self, conn: &mut PgConnection) -> Result<Vec<PgAttribute>, WebCodeGenError> {
        use crate::schema::{pg_attribute, pg_type};

        Ok(pg_attribute::table
            .inner_join(pg_type::table.on(pg_attribute::attrelid.eq(pg_type::typrelid)))
            .filter(pg_type::typname.eq(&self.typname).and(pg_attribute::attisdropped.eq(false)))
            .select(PgAttribute::as_select())
            .load::<PgAttribute>(conn)?)
    }

    /// Returns the variants of the type, if it is an enum type.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing the variants of the type if it is an enum type, or
    /// an error if it is not.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn variants(&self, conn: &mut PgConnection) -> Result<Vec<PgEnum>, WebCodeGenError> {
        use crate::schema::pg_enum;

        Ok(pg_enum::table
            .filter(pg_enum::enumtypid.eq(self.oid))
            .order_by(pg_enum::enumsortorder)
            .select(PgEnum::as_select())
            .load::<PgEnum>(conn)?)
    }
}
