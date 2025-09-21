
    /// Returns the identifier for this key column usage getter.
    ///
    /// # Implementation details
    ///
    /// The name of the constraint is defined as follows:
    ///
    /// * If the constraint refers to several columns, the name is the name of
    ///   the constraint as defined in the database.
    /// * If the constraint refers to a single column, but there exist some
    ///   other single-column constraint which also refers to the same column,
    ///   the name is the name of the constraint as defined in the database.
    /// * Otherwise, the name is the getter identifier of the column.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn constraint_ident(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Ident, WebCodeGenError> {
        let columns = self.columns(conn)?;
        if columns.len() == 1 {
            // We check whether there exist some other constraint which also refers
            // to the same column in a single-column constraint.
            let column = &columns[0];
            let constraints = column.foreign_keys(conn)?;
            let mut has_other_constraints = false;
            for constraint in constraints.as_ref() {
                if constraint == self {
                    continue;
                }
                if constraint.columns(conn)?.len() == 1 {
                    has_other_constraints = true;
                    break;
                }
            }
            if !has_other_constraints {
                // If there are no other constraints, we use the getter identifier
                return column.getter_ident();
            }
        }

        let mut snake_case_name = self.constraint_name.to_lowercase();
        while snake_case_name.contains("__") {
            snake_case_name = snake_case_name.replace("__", "_");
        }

        Ok(Ident::new(&snake_case_name, proc_macro2::Span::call_site()))
    }

    /// Returns the where statement for this key column usage
    ///
    /// # Arguments
    ///
    /// * `include_self` - A boolean indicating whether to include the `self`
    ///   reference in the where statement.
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn where_statement(
        &self,
        foreign_table: bool,
        include_self: bool,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        // Analogously, we check before executing the query whether the current column
        // is None. If so, we return None as well.
        let mut where_statement = TokenStream::new();

        let columns = self.columns(conn)?;
        let foreign_columns = self.foreign_columns(conn)?;
        let foreign_key_table = self.foreign_table(conn)?;
        let current_table = self.table(conn)?;
        let foreign_table_path = foreign_key_table.import_diesel_path()?;
        let current_table_path = current_table.import_diesel_path()?;

        assert!(!columns.is_empty(), "The key column usage must have at least one column {self:?}",);

        assert!(
            !foreign_columns.is_empty(),
            "The foreign key must have at least one column {self:?}",
        );

        assert_eq!(
            columns.len(),
            foreign_columns.len(),
            "The number of columns in the key column usage must match the number of foreign columns",
        );

        assert!(
            foreign_columns.iter().all(|c| c.table_name == foreign_key_table.table_name),
            "Error while processing table `{}.{}`'s FK `{}`: All foreign columns must belong to the same table `{}` as the foreign key, but got {foreign_columns:?}",
            self.table_schema,
            self.table_name,
            self.constraint_name,
            foreign_key_table.table_name
        );

        for (column, foreign_column) in columns.iter().zip(foreign_columns.iter()) {
            let current_column_ident: Ident = column.snake_case_ident()?;
            let foreign_column_ident: Ident = foreign_column.snake_case_ident()?;

            let column_attribute = if column.is_nullable() || !include_self {
                quote::quote! { #current_column_ident }
            } else {
                quote::quote! { &self.#current_column_ident }
            };

            let single_where_statement = if foreign_table {
                quote::quote! {
                    #foreign_table_path::dsl::#foreign_column_ident.eq(#column_attribute)
                }
            } else {
                quote::quote! {
                    #current_table_path::dsl::#current_column_ident.eq(#column_attribute)
                }
            };

            if where_statement.is_empty() {
                where_statement = single_where_statement;
            } else {
                where_statement = quote::quote! {
                    #where_statement.and(#single_where_statement)
                };
            }
        }

        Ok(where_statement)
    }