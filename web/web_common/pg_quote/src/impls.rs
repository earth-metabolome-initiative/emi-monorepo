//! Submodule providing implementations of several traits to facilitate working
//! with `pg_diesel` models in the context of `syn`-based code generation.

mod from;
mod try_from;



    /// Returns the path of extension foreign keys to get from the provided
    /// `table` to the provided `column`.
    ///
    /// # Arguments
    ///
    /// * `table`: A reference to the table to start from.
    /// * `column`: A reference to the column to find the path to.
    /// * `conn`: A mutable reference to a `PostgreSQL` connection.
    pub(crate) fn extension_foreign_keys_path(
        &self,
        column: &Column,
        conn: &mut PgConnection,
    ) -> Result<Option<Vec<KeyColumnUsage>>, WebCodeGenError> {
        // If the provided table contains the provided column, then the path is empty.
        if self.has_column(column) {
            return Ok(Some(Vec::new()));
        }

        for ancestor_table in self.extension_tables(conn)?.iter() {
            if let Some(mut ancestor_foreign_keys_path) =
                ancestor_table.extension_foreign_keys_path(column, conn)?
            {
                // We identify which of the foreign keys is the one that bridges
                // from the current table to the ancestor table.
                let foreign_key = self
                    .foreign_keys(conn)
                    .unwrap_or_default()
                    .iter()
                    .find(|fk| {
                        fk.is_extension(conn).unwrap_or(false)
                            && fk.foreign_table(conn).ok().is_some_and(|fk_table| {
                                fk_table.as_ref() == ancestor_table.as_ref()
                            })
                    })
                    .cloned()
                    .unwrap();

                ancestor_foreign_keys_path.insert(0, foreign_key);
                return Ok(Some(ancestor_foreign_keys_path));
            }
        }

        Ok(None)
    }

    /// Returns the generics with the expected default values for the provided
    /// table.
    ///
    /// # Arguments
    ///
    /// * `table`: A reference to the table to generate generics for.
    ///
    /// # Errors
    ///
    /// Returns an error if the generics cannot be generated.
    fn generics_for_table_builder_type(
        &self,
        covered_successors: &mut HashSet<Table>,
        conn: &mut PgConnection,
    ) -> Result<Option<TokenStream>, WebCodeGenError> {
        let extension_tables = self.extension_tables(conn)?;

        // If the table is at the root of the extension graph, it has no
        // builder type generics.
        if extension_tables.is_empty() {
            return Ok(None);
        }

        let generics = extension_tables
            .iter()
            .map(|extended_table| {
                // If the extended table is already covered, its generic
                // parameters will be the primary key of the table, otherwise
                // we recursively determine it as the builder type with its own
                // generics.
                if covered_successors.insert(extended_table.as_ref().clone()) {
                    let extended_builder_generics =
                        extended_table.generics_for_table_builder_type(covered_successors, conn)?;
                    let extended_builder_type = extended_table.insertable_builder_ty()?;
                    Ok(quote::quote! { #extended_builder_type #extended_builder_generics })
                } else {
                    let primary_key_ty = extended_table.primary_key_type(conn)?;
                    Ok(quote::quote! { Option<#primary_key_ty> })
                }
            })
            .collect::<Result<Vec<_>, WebCodeGenError>>()?;

        Ok(Some(quote::quote! {
            <#(#generics),*>
        }))
    }

    /// Returns the generics with the expected default values for the provided
    /// table.
    ///
    /// # Arguments
    ///
    /// * `table`: A reference to the table to generate generics for.
    /// * `conn`: A mutable reference to a `PostgreSQL` connection.
    ///
    /// # Errors
    ///
    /// Returns an error if the generics cannot be generated.
    pub(crate) fn generics_for_table_builder_definition(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<TokenStream>, WebCodeGenError> {
        let extension_tables = self.extension_tables(conn)?;

        // If the table is at the root of the extension graph, it has no
        // builder type generics.
        if extension_tables.is_empty() {
            return Ok(None);
        }

        // We create a hashmap to map whether an ancestor extended by the current
        // table has already been handled by a predecessor or not.
        let mut covered_successors = HashSet::new();

        // Otherwise, for each extended table, we generate the set of generics
        // with the expected default values to define the builder type.
        let generics = extension_tables
            .iter()
            .map(|extended_table| {
                let extended_table_ident = extended_table.struct_ident()?;
                let extended_builder_type = extended_table.insertable_builder_ty()?;
                let extended_builder_generics = extended_table
                    .generics_for_table_builder_type(&mut covered_successors, conn)?;
                Ok(quote::quote! {
                    #extended_table_ident=#extended_builder_type #extended_builder_generics
                })
            })
            .collect::<Result<Vec<_>, WebCodeGenError>>()?;

        Ok(Some(quote::quote! {
            <#(#generics),*>
        }))
    }

    /// Returns the extension table idents for the table builder implementation.
    ///
    /// # Arguments
    ///
    /// * `table`: A reference to the table to generate generics for.
    ///
    /// # Errors
    ///
    /// Returns an error if the generics cannot be generated.
    pub(crate) fn generics(&self, conn: &mut PgConnection) -> Result<Vec<Ident>, WebCodeGenError> {
        self.extension_tables(conn)?
            .iter()
            .map(TableLike::struct_ident)
            .collect::<Result<Vec<_>, WebCodeGenError>>()
    }

    /// Returns the generics for the table builder implementation.
    ///
    /// # Arguments
    ///
    /// * `table`: A reference to the table to generate generics for.
    ///
    /// # Errors
    ///
    /// Returns an error if the generics cannot be generated.
    pub(crate) fn formatted_generics(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<TokenStream>, WebCodeGenError> {
        let generics = self.generics(conn)?;

        // If the table is at the root of the extension graph, it has no
        // builder type generics.
        if generics.is_empty() {
            return Ok(None);
        }

        Ok(Some(quote::quote! {
            <#(#generics),*>
        }))
    }