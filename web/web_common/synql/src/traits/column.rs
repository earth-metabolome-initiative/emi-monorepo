

#[cached(result = true, key = "String", convert = r#"{ format!("{column}") }"#)]
fn str_rust_data_type(column: &Column, conn: &mut PgConnection) -> Result<String, WebCodeGenError> {
    if let Ok(Some(geometry)) = column.geometry(conn) {
        return Ok(geometry.str_rust_type().to_owned());
    }
    if let Ok(Some(geography)) = column.geography(conn) {
        return Ok(geography.str_rust_type().to_owned());
    }
    match rust_type_str(column.data_type_str(conn)?, conn) {
        Ok(s) => Ok(s.to_string()),
        Err(error) => {
            if column.has_custom_type() {
                Ok(PgType::from_name(column.data_type_str(conn)?, conn)?.camelcased_name())
            } else {
                Err(error)
            }
        }
    }
}


    /// Returns the multi-column getter method ident for the provided columns.
    ///
    /// # Arguments
    ///
    /// * `columns` - A slice of `Column` references
    ///
    /// # Errors
    ///
    /// * If an error occurs while sanitizing the column names
    pub fn multi_column_getter_ident<C: AsRef<Column>>(
        columns: &[C],
    ) -> Result<Ident, crate::error::Error> {
        let getter_name = Self::multi_column_getter_name(columns)?;
        if RESERVED_RUST_WORDS.contains(&getter_name.as_str()) {
            Ok(Ident::new_raw(&getter_name, proc_macro2::Span::call_site()))
        } else {
            Ok(Ident::new(&getter_name, proc_macro2::Span::call_site()))
        }
    }


    /// Returns the multi-column getter method name for the provided columns.
    ///
    /// # Arguments
    ///
    /// * `columns` - A slice of `Column` references
    ///
    /// # Errors
    ///
    /// * If an error occurs while sanitizing the column names
    pub fn multi_column_getter_name<C: AsRef<Column>>(
        columns: &[C],
    ) -> Result<String, crate::error::Error> {
        Ok(columns
            .iter()
            .map(|c| c.as_ref().getter_name())
            .collect::<Result<Vec<String>, crate::error::Error>>()?
            .join("_and_"))
    }



    /// Returns the getter method ident for the column.
    ///
    /// # Errors
    ///
    /// * If an error occurs while sanitizing the column name
    ///
    /// # Returns
    ///
    /// A `Result` containing the getter method ident if the operation was
    /// successful,
    pub fn getter_ident(&self) -> Result<Ident, crate::error::Error> {
        let getter_name = self.getter_name()?;
        if RESERVED_RUST_WORDS.contains(&getter_name.as_str()) {
            Ok(Ident::new_raw(&getter_name, proc_macro2::Span::call_site()))
        } else {
            Ok(Ident::new(&getter_name, proc_macro2::Span::call_site()))
        }
    }


    /// Returns the getter method name for the column.
    ///
    /// # Errors
    ///
    /// * If an error occurs while sanitizing the column name
    ///
    /// # Returns
    ///
    /// A `Result` containing the getter method name if the operation was
    /// successful,
    pub fn getter_name(&self) -> Result<String, crate::error::Error> {
        let mut snake_case_name = self.snake_case_name()?;
        if let Some(stripped_snake_case_name) = snake_case_name.strip_suffix("_id") {
            snake_case_name = stripped_snake_case_name.to_owned();
        }

        Ok(snake_case_name)
    }



    /// Returns the rust `TokenStream` to create the default value of the column
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn rust_default_value(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, crate::error::Error> {
        let Some(column_default) = &self.column_default else {
            return Err(crate::error::Error::ColumnDoesNotHaveDefaultValue(Box::new(self.clone())));
        };
        let rust_str_data_type = self.str_rust_data_type(conn)?;
        let default = DefaultTypes::new(&rust_str_data_type, column_default)?;
        Ok(match (rust_str_data_type.as_str(), default) {
            (
                "::chrono::NaiveDateTime" | "chrono::NaiveDateTime",
                DefaultTypes::CurrentTimestamp,
            ) => {
                quote::quote! {
                    chrono::Local::now().naive_local()
                }
            }
            (
                "::rosetta_timestamp::TimestampUTC" | "rosetta_timestamp::TimestampUTC",
                DefaultTypes::CurrentTimestamp,
            ) => {
                quote::quote! {
                    rosetta_timestamp::TimestampUTC::default()
                }
            }
            ("i16", DefaultTypes::I16(value)) => {
                quote::quote! {
                    #value
                }
            }
            ("i32", DefaultTypes::I32(value)) => {
                quote::quote! {
                    #value
                }
            }
            ("i64", DefaultTypes::I64(value)) => {
                quote::quote! {
                    #value
                }
            }
            ("f32", DefaultTypes::F32(value)) => {
                quote::quote! {
                    #value
                }
            }
            ("f64", DefaultTypes::F64(value)) => {
                quote::quote! {
                    #value
                }
            }
            ("bool", DefaultTypes::Bool(value)) => {
                quote::quote! {
                    #value
                }
            }
            ("String", DefaultTypes::String(value)) => {
                quote::quote! {
                    #value.to_owned()
                }
            }
            ("::rosetta_uuid::Uuid" | "rosetta_uuid::Uuid", DefaultTypes::Uuid(value)) => value,
            (r#type, default) => {
                unimplemented!(
                    "Default value `{default:?}` for column \"{}\".\"{}\" of type `{}` is not implemented!",
                    self.table_name,
                    self.column_name,
                    r#type
                )
            }
        })
    }



    /// Returns whether the column name is a reserved diesel word.
    ///
    /// # Errors
    ///
    /// If an error occurs while sanitizing the column name
    pub fn requires_diesel_sanitization(&self) -> Result<bool, crate::error::Error> {
        Ok(RESERVED_DIESEL_WORDS.contains(&self.snake_case_name()?.as_str()))
    }

    /// Returns the sanitized snake case name of the table.
    ///
    /// # Errors
    ///
    /// If an error occurs while sanitizing the column name
    pub fn snake_case_name(&self) -> Result<String, crate::error::Error> {
        crate::utils::snake_case_name(&self.column_name)
    }

    /// Returns the sanitized snake case syn Ident of the table.
    ///
    /// If the column name is a reserved diesel word, the returned ident will be
    /// prefixed with `__`. If the column name is a reserved rust word, the
    /// returned ident will be the raw ident. Otherwise, the returned ident
    /// will be the sanitized snake case ident.
    ///
    /// # Returns
    ///
    /// A `Result` containing the sanitized snake case `Ident` if the operation
    /// was successful, or a `crate::error::Error` if an error occurred
    ///
    /// # Errors
    ///
    /// If an error occurs while sanitizing the column name
    pub fn snake_case_ident(&self) -> Result<Ident, crate::error::Error> {
        let snake_case_name = self.snake_case_name()?;
        if self.requires_diesel_sanitization()? {
            Ok(Ident::new(&format!("__{snake_case_name}"), proc_macro2::Span::call_site()))
        } else if RESERVED_RUST_WORDS.contains(&snake_case_name.as_str()) {
            Ok(Ident::new_raw(&snake_case_name, proc_macro2::Span::call_site()))
        } else {
            Ok(Ident::new(&snake_case_name, proc_macro2::Span::call_site()))
        }
    }

    /// Returns the sanitized camel case name of the table.
    ///
    /// # Errors
    ///
    /// * If an error occurs while sanitizing the column name
    pub fn camel_case_name(&self) -> Result<String, crate::error::Error> {
        crate::utils::camel_case_name(&self.column_name)
    }

    /// Returns the sanitized camel case syn Ident of the table.
    ///
    /// # Errors
    ///
    /// * If an error occurs while sanitizing the column name
    pub fn camel_case_ident(&self) -> Result<Ident, crate::error::Error> {
        let camel_case_name = self.camel_case_name()?;
        if RESERVED_RUST_WORDS.contains(&camel_case_name.as_str()) {
            Ok(Ident::new_raw(&camel_case_name, proc_macro2::Span::call_site()))
        } else {
            Ok(Ident::new(&camel_case_name, proc_macro2::Span::call_site()))
        }
    }

    /// Returns the uppercased acronym of the column name.
    ///
    /// # Errors
    ///
    /// * If an error occurs while generating the acronym
    pub fn acronym(&self) -> Result<String, crate::error::Error> {
        let camel_cased_name = self.snake_case_name()?;
        Ok(camel_cased_name
            .split('_')
            .filter_map(|s| s.chars().next())
            .map(|c| c.to_ascii_uppercase())
            .collect())
    }

    /// Returns the uppercased acronym syn Ident of the column name.
    ///
    /// # Errors
    ///
    /// * If an error occurs while generating the acronym
    pub fn acronym_ident(&self) -> Result<Ident, crate::error::Error> {
        let acronym = self.acronym()?;
        if RESERVED_RUST_WORDS.contains(&acronym.as_str()) {
            Ok(Ident::new_raw(&acronym, proc_macro2::Span::call_site()))
        } else {
            Ok(Ident::new(&acronym, proc_macro2::Span::call_site()))
        }
    }


/// Returns the primary key type for the table.
///
/// # Arguments
///
/// * `conn` - The database connection.
///
/// # Returns
///
/// The syn data type representing the primary key type.
///
/// # Errors
///
/// * If the table does not have primary keys.
/// * If the primary key columns cannot be loaded from the database.
pub fn primary_key_type(&self, conn: &mut PgConnection) -> Result<Type, WebCodeGenError> {
    let primary_key_columns = self.primary_key_columns(conn)?;

    if primary_key_columns.is_empty() {
        return Err(WebCodeGenError::NoPrimaryKeyColumn(Box::new(self.clone())));
    }

    // We construct the rust type or tuple of rust types that represent the primary
    // key.
    Ok(if primary_key_columns.len() == 1 {
        // If the primary key is a single column, we can just use the type of that
        // column.
        primary_key_columns[0].rust_data_type(conn)?
    } else {
        // If the primary key is a composite key, we need to construct a tuple of the
        // types.
        let mut primary_key_types = Vec::new();

        for column in primary_key_columns.as_ref() {
            let column_type = column.rust_data_type(conn)?;
            primary_key_types.push(column_type);
        }

        syn::parse_quote! { (#(#primary_key_types),*) }
    })
}

/// Returns the primary key attribute(s) for the table.
///
/// # Arguments
///
/// * `include_self` - Whether to include the table self.
/// * `conn` - The database connection.
///
/// # Returns
///
/// The syn [`TokenStream`](proc_macro2::TokenStream) representing the
/// primary key attribute(s).
///
/// # Errors
///
/// * If the table does not have primary keys.
/// * If the primary key columns cannot be loaded from the database.
pub fn primary_key_attributes(
    &self,
    include_self: bool,
    conn: &mut PgConnection,
) -> Result<TokenStream, WebCodeGenError> {
    let primary_key_columns = self.primary_key_columns(conn)?;
    // If the primary key is a composite key, we need to construct a tuple of the
    // types.
    let primary_key_names = primary_key_columns
        .iter()
        .map(|column| {
            let column_ident = column.snake_case_ident()?;
            let maybe_clone =
                if column.supports_copy(conn)? { None } else { Some(quote! { .clone() }) };
            Ok(if include_self {
                quote! { self.#column_ident #maybe_clone }
            } else {
                quote! { #column_ident #maybe_clone }
            })
        })
        .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

    let formatted = quote! {
        #(#primary_key_names),*
    };

    if primary_key_columns.len() == 1 { Ok(formatted) } else { Ok(quote! { ( #formatted ) }) }
}

/// Returns the primary key identifiers for the table.
///
/// # Arguments
///
/// * `conn` - The database connection.
///
/// # Returns
///
/// A vector of `Ident` representing the primary key identifiers.
///
/// # Errors
///
/// * If the primary key columns cannot be loaded from the database.
pub fn primary_key_idents(&self, conn: &mut PgConnection) -> Result<Vec<Ident>, WebCodeGenError> {
    self.primary_key_columns(conn)?
        .as_ref()
        .iter()
        .map(Column::snake_case_ident)
        .collect::<Result<Vec<Ident>, WebCodeGenError>>()
}

/// Returns the primary key decorator to be used for this table, if any.
///
/// # Arguments
///
/// * `conn` - The database connection.
///
/// # Returns
///
/// A `TokenStream` representing the primary key decorator.
///
/// # Errors
///
/// * If the primary key columns cannot be loaded from the database.
pub fn primary_key_decorator(
    &self,
    conn: &mut PgConnection,
) -> Result<TokenStream, WebCodeGenError> {
    // In some cases, the table will not have a primary key. In which case, we
    // cannot specify the primary key decorator on the struct.
    Ok(if self.has_primary_keys(conn)? {
        let primary_key_idents = self.primary_key_idents(conn)?;
        quote! {
            #[diesel(primary_key(#(#primary_key_idents),*))]
        }
    } else {
        TokenStream::new()
    })
}

/// Returns the path to the function.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a [`PgConnection`](diesel::PgConnection).
///
/// # Errors
///
/// * If the database connection is invalid.
/// * If the function is not contained in an extension.
pub fn path(&self, conn: &mut PgConnection) -> Result<syn::Path, diesel::result::Error> {
    let extension = self.extension(conn)?;
    let extension = extension.map(|ext| ext.ident());
    let ident = syn::Ident::new(&self.proname, proc_macro2::Span::call_site());
    let segments: Vec<syn::PathSegment> = match extension {
        Some(extension) => vec![extension.into(), ident.into()],
        None => vec![ident.into()],
    };
    Ok(syn::Path { leading_colon: None, segments: Punctuated::from_iter(segments) })
}

/// Returns whether the current function may be return a `Result`.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a `PgConnection`.
///
/// # Errors
///
/// * If the return type does not exist.
pub fn returns_result(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
    if self.proname.starts_with("must_be_")
        || self.proname.starts_with("must_not_be_")
        || self.proname.starts_with("must_contain_")
        || self.proname.starts_with("must_not_contain_")
    {
        Ok(self.return_type(conn)?.is_boolean(conn)?)
    } else {
        Ok(false)
    }
}
