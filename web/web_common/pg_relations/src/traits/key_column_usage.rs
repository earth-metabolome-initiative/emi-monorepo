
    /// Returns whether this key column usage defines an extension.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn is_extension(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        is_extension(self, conn)
    }



impl PartialBuilderKind {
    /// Returns whether the partial builder constraint is discretionary
    pub fn is_discretional(self) -> bool {
        matches!(self, PartialBuilderKind::Discretional)
    }

    /// Returns the formatted type of the partial builder constraint.
    ///
    /// # Arguments
    ///
    /// * `table` - The table associated with the partial builder constraint.
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database.
    pub(crate) fn formatted_type<T: AsRef<Table>>(
        self,
        table: &T,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, crate::error::Error> {
        let builder_type = table.as_ref().insertable_builder_ty()?;
        match self {
            PartialBuilderKind::Discretional => {
                let primary_key_type = table.as_ref().primary_key_type(conn)?;
                Ok(
                    quote! { web_common_traits::database::IdOrBuilder<#primary_key_type, #builder_type> },
                )
            }
            PartialBuilderKind::Mandatory => Ok(quote! { #builder_type }),
        }
    }
}


    /// Returns whether the key is a singleton foreign key, i.e. it is the only
    /// foreign key to refer to a particular foreign table within the context
    /// of its table of definition.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn is_singleton(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, crate::error::Error> {
        if self.is_local_primary_key(conn)? {
            return Ok(false);
        }
        if self.columns(conn)?.len() != 1 {
            return Ok(false);
        }
        let foreign_table = self.foreign_table(conn)?;
        let table = self.table(conn)?;
        Ok(table.foreign_keys(conn)?.iter().all(|fk| {
            fk == self || fk.foreign_table(conn).map(|t| t != foreign_table).unwrap_or(true)
        }))
    }

	pub fn is_extension(
    key_column_usage: &KeyColumnUsage,
    conn: &mut PgConnection,
) -> Result<bool, crate::error::Error> {
    Ok(key_column_usage.is_foreign_primary_key(conn)?
        && key_column_usage.is_local_primary_key(conn)?
        && !key_column_usage.is_self_referential(conn)?)
}