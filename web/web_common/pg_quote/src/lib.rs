#![doc = include_str!("../README.md")]

mod impls;


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
    pub fn primary_key_idents(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Ident>, WebCodeGenError> {
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
    /// * `conn` - A mutable reference to a
    ///   [`PgConnection`](diesel::PgConnection).
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