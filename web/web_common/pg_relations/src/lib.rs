#![doc = include_str!("../README.md")]

pub mod traits;
mod impls;
pub(crate) mod functions;

/// Returns the `KeyColumnUsage` linking to extension tables, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the table cannot be loaded from the database.
    pub fn extension_foreign_keys(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<KeyColumnUsage>, WebCodeGenError> {
        Ok(self
            .foreign_keys(conn)?
            .as_ref()
            .iter()
            .filter(|foreign_key| foreign_key.is_extension(conn).unwrap_or(false))
            .cloned()
            .collect())
    }

    /// Returns the immediate tables this table extends, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the table cannot be loaded from the database.
    ///
    /// # Implementation details
    ///
    /// A table is considered an extension table if it has a primary key
    /// composed of a single column which is also a foreign key to another
    /// table. This does apply to composite primary keys.
    pub fn extension_tables(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Arc<Vec<Arc<Table>>>, WebCodeGenError> {
        extension_tables(self, conn)
    }

    /// Returns whether the current table extends any other table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the database connection is invalid.
    pub fn is_extension(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        Ok(!self.extension_tables(conn)?.is_empty())
    }

    /// Returns the associated same-as foreign keys of the table, if any.
    ///
    /// # Arguments
    ///
    /// * `include_local_primary_key` - Whether to include the local primary key
    ///   in the constraint.
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the table cannot be loaded from the database.
    pub fn associated_same_as_foreign_keys(
        &self,
        include_local_primary_key: bool,
        conn: &mut PgConnection,
    ) -> Result<Vec<KeyColumnUsage>, WebCodeGenError> {
        let mut associated_foreign_key = Vec::new();
        for foreign_key in self.foreign_keys(conn)?.as_ref() {
            if foreign_key
                .is_associated_same_as_constraint(include_local_primary_key, conn)?
                .is_some()
            {
                associated_foreign_key.push(foreign_key.clone());
            }
        }

        Ok(associated_foreign_key)
    }

    /// Returns the ancestral same-as foreign keys of the table, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the table cannot be loaded from the database.
    pub fn ancestral_same_as_foreign_keys(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<KeyColumnUsage>, WebCodeGenError> {
        let mut ancestral_foreign_key = Vec::new();
        for foreign_key in self.foreign_keys(conn)?.as_ref() {
            if foreign_key.is_ancestral_same_as_constraint(conn)?.is_some() {
                ancestral_foreign_key.push(foreign_key.clone());
            }
        }

        ancestral_foreign_key.sort_unstable();
        ancestral_foreign_key.dedup();

        Ok(ancestral_foreign_key)
    }

    /// Returns the associated tables this table references via foreign keys, if
    /// any.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Implementative details
    ///
    /// A table referenced by another table is considered associated if any
    /// columns of the latter table referencing the former table are not
    /// part of the primary key of the latter table, and still require a
    /// partial builder.
    ///
    /// # Errors
    ///
    /// * If the table cannot be loaded from the database.
    pub(crate) fn associated_tables(
        &self,
        include_local_primary_key: bool,
        conn: &mut PgConnection,
    ) -> Result<Vec<Arc<Table>>, WebCodeGenError> {
        let mut associated_tables = Vec::new();
        for foreign_key in self.associated_same_as_foreign_keys(include_local_primary_key, conn)? {
            let foreign_table = foreign_key.foreign_table(conn)?;
            associated_tables.push(foreign_table);
        }

        associated_tables.sort_unstable();
        associated_tables.dedup();

        Ok(associated_tables)
    }

    /// Returns the tables this table extends, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the table cannot be loaded from the database.
    ///
    /// # Implementation details
    ///
    /// A table is considered an extension table if it has a primary key
    /// composed of a single column which is also a foreign key to another
    /// table. This does apply to composite primary keys.
    pub fn ancestral_extension_tables(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Arc<Vec<Table>>, WebCodeGenError> {
        ancestral_extension_tables(self, conn)
    }

    /// Returns whether the table is extending the provided table.
    ///
    /// # Arguments
    ///
    /// * `ancestor` - The potential ancestor table.
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the table cannot be loaded from the database.
    pub fn is_extending(
        &self,
        ancestor: &Table,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        self.ancestral_extension_tables(conn).map(|tables| tables.contains(ancestor))
    }

    /// Returns whether the table shares any ancestor with the provided table.
    ///
    /// # Arguments
    ///
    /// * `other` - The other table.
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the table cannot be loaded from the database.
    pub fn share_ancestors(
        &self,
        other: &Table,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        let self_ancestors = self.ancestral_extension_tables(conn)?;
        let other_ancestors = other.ancestral_extension_tables(conn)?;

        for ancestor in self_ancestors.as_ref() {
            if other_ancestors.contains(ancestor) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Returns the columns and foreign keys of the table which require partial
    /// builders.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the columns cannot be loaded from the database.
    pub fn partial_builder_columns(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<(Column, PartialBuilderKind, KeyColumnUsage, KeyColumnUsage)>, WebCodeGenError>
    {
        Ok(self
            .columns(conn)?
            .as_ref()
            .iter()
            .filter_map(|column| {
                let (partial_builder_kind, potential_same_as_constraint, foreign_key) =
                    column.requires_partial_builder(conn).ok().flatten()?;
                Some((
                    column.clone(),
                    partial_builder_kind,
                    potential_same_as_constraint,
                    foreign_key,
                ))
            })
            .collect())
    }

    /// Returns the same as indices for the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of indices.
    ///
    /// # Errors
    ///
    /// * If the indices cannot be loaded from the database.
    pub fn same_as_indices(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<PgIndex>, WebCodeGenError> {
        same_as_indices(self, conn)
    }

    /// Returns the same as foreign keys for the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the foreign keys cannot be loaded from the database.
    pub fn same_as_foreign_keys(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<(KeyColumnUsage, PgIndex)>, WebCodeGenError> {
        let mut same_as_foreign_keys = Vec::new();
        for foreign_key in self.foreign_keys(conn)?.as_ref() {
            if let Some(index) = foreign_key.is_same_as_constraint(conn)? {
                same_as_foreign_keys.push((foreign_key.clone(), index));
            }
        }
        Ok(same_as_foreign_keys)
    }

    /// Returns whether the table has an `created_by` column.
    ///
    /// # Arguments
    ///
    /// * `include_ancestors` - Whether to include ancestor tables in the
    ///   search.
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection is invalid.
    pub fn has_created_by_column(
        &self,
        include_ancestors: bool,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        for column in self.columns(conn)?.as_ref() {
            if column.is_created_by(conn)? {
                return Ok(true);
            }
        }

        if include_ancestors {
            for table in self.extension_tables(conn)?.as_ref() {
                if table.has_created_by_column(true, conn)? {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    /// Returns the most concrete table column, if any.
    ///
    /// # Arguments
    ///
    /// * `include_ancestors` - Whether to include ancestor tables in the
    ///   search.
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection is invalid.
    pub fn most_concrete_table_column(
        &self,
        include_ancestors: bool,
        conn: &mut PgConnection,
    ) -> Result<Option<Column>, WebCodeGenError> {
        for column in self.columns(conn)?.as_ref() {
            if column.is_most_concrete_table(conn)? {
                return Ok(Some(column.clone()));
            }
        }

        if include_ancestors {
            let extension_tables = self.extension_tables(conn)?;
            for table in extension_tables.as_ref() {
                if let Some(column) = table.most_concrete_table_column(true, conn)? {
                    return Ok(Some(column));
                }
            }

            // All tables in the extension hierarchy must have the
            // most concrete table column in their root.
            if !extension_tables.is_empty() {
                unreachable!(
                    "The current SQL schema is invalid: the extension hierarchy of table {self} does not have a `most concrete table` column.",
                )
            }
        }

        Ok(None)
    }

    /// Returns whether the table has a `most concrete table` column.
    ///
    /// # Arguments
    ///
    /// * `include_ancestors` - Whether to include ancestor tables in the
    ///   search.
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection is invalid.
    pub fn has_most_concrete_table_column(
        &self,
        include_ancestors: bool,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        Ok(self.most_concrete_table_column(include_ancestors, conn)?.is_some())
    }

    /// Returns the `updated_by` column of the table, if any.
    ///
    /// # Arguments
    ///
    /// * `include_ancestors` - Whether to include ancestor tables in the
    ///   search.
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection is invalid.
    pub(crate) fn updated_by_column(
        &self,
        include_ancestors: bool,
        conn: &mut PgConnection,
    ) -> Result<Option<Column>, WebCodeGenError> {
        for column in self.columns(conn)?.as_ref() {
            if column.is_updated_by(conn)? {
                return Ok(Some(column.clone()));
            }
        }
        if include_ancestors {
            for table in self.extension_tables(conn)?.as_ref() {
                if let Some(column) = table.updated_by_column(true, conn)? {
                    return Ok(Some(column));
                }
            }
        }
        Ok(None)
    }

    /// Returns whether the table has an `updated_by` column.
    ///
    /// # Arguments
    ///
    /// * `include_ancestors` - Whether to include ancestor tables in the
    ///   search.
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection is invalid.
    pub(crate) fn has_updated_by_column(
        &self,
        include_ancestors: bool,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        Ok(self.updated_by_column(include_ancestors, conn)?.is_some())
    }

    /// Returns the parent keys of the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// The set of foreign key columns that have `ON DELETE CASCADE` constraint.
    ///
    /// # Errors
    ///
    /// * If the foreign keys cannot be loaded from the database.
    pub fn parent_keys(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<KeyColumnUsage>, WebCodeGenError> {
        let mut parent_keys = Vec::new();
        for foreign_key in self.foreign_keys(conn)?.as_ref() {
            if !foreign_key.has_on_delete_cascade(conn)?
                && foreign_key.is_foreign_primary_key(conn)?
                && !foreign_key.is_self_referential(conn)?
                && foreign_key.is_ancestral_same_as_constraint(conn)?.is_none()
            {
                parent_keys.push(foreign_key.clone());
            }
        }
        Ok(parent_keys)
    }

    /// Returns the parent tables of the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of tables that are parents to the current table.
    ///
    /// # Errors
    ///
    /// * If the parent tables cannot be loaded from the database.
    pub fn parent_tables(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Arc<Table>>, WebCodeGenError> {
        let mut parent_tables = Vec::new();
        for foreign_key in self.parent_keys(conn)? {
            parent_tables.push(foreign_key.foreign_table(conn)?);
        }
        parent_tables.sort_unstable();
        parent_tables.dedup();
        Ok(parent_tables)
    }

    /// Returns the foreign key columns which point to the current table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the foreign keys cannot be loaded from the database.
    pub fn homogeneous_parent_columns(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Arc<Vec<Column>>>, WebCodeGenError> {
        let mut homogeneous_parent_columns = Vec::new();
        for foreign_key in self.foreign_keys(conn)?.as_ref() {
            let foreign_table = foreign_key.foreign_table(conn)?;
            if foreign_table.as_ref() == self && foreign_key.is_foreign_primary_key(conn)? {
                homogeneous_parent_columns.push(foreign_key.columns(conn)?);
            }
        }

        Ok(homogeneous_parent_columns)
    }

    /// Returns whether the table has parent tables.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the table has parent tables.
    ///
    /// # Errors
    ///
    /// * If the foreign keys cannot be loaded from the database.
    pub fn has_parents(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        Ok(!self.parent_keys(conn)?.is_empty())
    }

    /// Returns whether the table has singleton foreign keys.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the foreign keys cannot be loaded from the database.
    pub fn has_singleton_foreign_keys(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        Ok(self.foreign_keys(conn)?.iter().any(|fk| fk.is_singleton(conn).unwrap_or(false)))
    }

    /// Returns the table singleton foreign keys.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the foreign keys cannot be loaded from the database.
    pub fn singleton_foreign_keys(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<KeyColumnUsage>, WebCodeGenError> {
        Ok(self
            .foreign_keys(conn)?
            .iter()
            .filter(|fk| fk.is_singleton(conn).unwrap_or(false))
            .cloned()
            .collect())
    }


#[pg_cached::auto_cached]
pub(super) fn ancestral_extension_tables(
    table: &Table,
    conn: &mut PgConnection,
) -> Result<Arc<Vec<Table>>, crate::error::Error> {
    let mut tables = Vec::new();
    for extended_table in table.extension_tables(conn)?.as_ref() {
        tables.extend(extended_table.ancestral_extension_tables(conn)?.as_ref().iter().cloned());
        tables.push(extended_table.as_ref().clone());
    }
    tables.sort_unstable();
    tables.dedup();
    Ok(Arc::new(tables))
}



#[pg_cached::auto_cached]
pub(super) fn extension_tables(
    table: &Table,
    conn: &mut PgConnection,
) -> Result<Arc<Vec<Arc<Table>>>, crate::error::Error> {
    Ok(Arc::new(
        table
            .extension_foreign_keys(conn)?
            .into_iter()
            .filter_map(|foreign_key| foreign_key.foreign_table(conn).ok())
            .collect(),
    ))
}


    /// Returns whether this key column usage is a `same-as` constraint
    ///
    /// A `same-as` constraint is a composite foreign key that refers to a
    /// UNIQUE constraint, where the foreign key's foreign columns are the same
    /// as the primary key of the foreign table, and the foreign column
    /// corresponding to the current column is part of the primary key of the
    /// foreign table.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_same_as_constraint(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<PgIndex>, WebCodeGenError> {
        let Some(foreign_unique_constraint) = self.is_foreign_unique_key(conn)? else {
            return Ok(None);
        };

        Ok(if foreign_unique_constraint.is_same_as(conn)? {
            // If the foreign unique constraint is a same-as constraint, we return it
            Some(foreign_unique_constraint)
        } else {
            // Otherwise, we return None
            None
        })
    }

    /// Returns whether this key column usage is an ancestral same-as
    /// constraint.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_ancestral_same_as_constraint(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<PgIndex>, WebCodeGenError> {
        if !self.includes_local_primary_key(conn)? {
            return Ok(None);
        }

        let foreign_table = self.foreign_table(conn)?;
        let table = self.table(conn)?;

        if !table.is_extending(&foreign_table, conn)? {
            return Ok(None);
        }

        self.is_same_as_constraint(conn)
    }

    /// Returns whether this key column usage is an associated same-as
    /// constraint.
    ///
    /// # Arguments
    ///
    /// * `include_local_primary_key` - Whether to include the local primary key
    ///   in the constraint
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_associated_same_as_constraint(
        &self,
        include_local_primary_key: bool,
        conn: &mut PgConnection,
    ) -> Result<Option<PgIndex>, WebCodeGenError> {
        if !include_local_primary_key && self.includes_local_primary_key(conn)? {
            return Ok(None);
        }
        if !self
            .columns(conn)?
            .iter()
            .any(|c| c.requires_partial_builder(conn).ok().flatten().is_some())
        {
            return Ok(None);
        }

        let foreign_table = self.foreign_table(conn)?;

        let table = self.table(conn)?;

        if table.is_extending(&foreign_table, conn)? {
            return Ok(None);
        }

        self.is_same_as_constraint(conn)
    }

    /// Returns whether this constraint may be either a partial builder
    /// of a potential partial builder constraint.
    fn preliminary_partial_builder_check(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<Vec<KeyColumnUsage>>, WebCodeGenError> {
        // A partial builder constraint must be a foreign primary key.
        // and if the foreign key is self-referential, we do not consider it
        // a partial builder constraint.
        if self.is_self_referential(conn)? || !self.is_foreign_primary_key(conn)? {
            return Ok(None);
        }

        let foreign_table = self.foreign_table(conn)?;
        let host_table = self.table(conn)?;
        // If the source table is an extension of the foreign table,
        // we do not consider it a partial builder constraint.
        if host_table.is_extending(&foreign_table, conn)?
            || host_table.share_ancestors(&foreign_table, conn)?
        {
            return Ok(None);
        }

        // At this point, we need to identify foreign keys in the
        // foreign table which point to ancestors of the current table.
        let mut keys_to_local_ancestors = Vec::new();
        let primary_key_columns = host_table.primary_key_columns(conn)?;
        for foreign_key in foreign_table.foreign_keys(conn)?.as_ref() {
            let foreign_columns = foreign_key.foreign_columns(conn)?;

            if foreign_columns.len() != primary_key_columns.len() {
                continue;
            }

            if foreign_columns.iter().zip(primary_key_columns.iter()).all(
                |(foreign_column, primary_key_column)| {
                    primary_key_column.is_ancestrally_same_as(foreign_column, conn).unwrap_or(false)
                },
            ) {
                keys_to_local_ancestors.push(foreign_key.clone());
            }
        }

        if keys_to_local_ancestors.is_empty() {
            return Ok(None);
        }

        // While it is conceivable to define partial builders on the foreign keys
        // and not on the columns themselves, at this time we are proceeding solely
        // with a column-based approach. Hence, we only support single-column foreign
        // keys.
        if foreign_table.has_composite_primary_key(conn)? {
            unimplemented!(
                "Partial builders from table {host_table} to table {foreign_table} on composite foreign keys are not supported yet"
            );
        }

        Ok(Some(keys_to_local_ancestors))
    }

    /// Returns whether this key column usage is a potential partial builder
    /// same-as constraint.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Implementation details
    ///
    /// A potential partial builder constraint is a constraint that
    /// differs from a partial builder constraint in that the host
    /// table does not have the same-as constraint which closes the triangular
    /// relationship, and therefore the associated table may require the primary
    /// key of an ancestor of the host table to be built, but it also may not.
    /// Such distintion is modelled by requiring these structs to have as type
    /// parameter `IdOrBuilder<PK, B>`, where `PK` is the primary key type of
    /// the associated table, and `B` is the builder type of the associated
    /// table. This way, the user of the API can choose to provide either
    /// the primary key or the builder type when creating a new instance of
    /// the associated table.
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn is_partial_builder_constraint(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<(PartialBuilderKind, KeyColumnUsage)>, WebCodeGenError> {
        // First, we perform some preliminary checks to rule out
        // constraints that cannot possibly be partial builders.
        let Some(keys_to_local_ancestors) = self.preliminary_partial_builder_check(conn)? else {
            return Ok(None);
        };

        // As described in the method documentation, what distinguishes
        // a potential partial builder constraint from a partial builder
        // constraint is the absence of same-as constraints in the host table
        // pointing to foreign keys in the foreign table which point to
        // ancestors of the host table.

        // We determine the local columns of the constraints pointing to
        // ancestors of the host table.
        let columns_to_local_ancestors = keys_to_local_ancestors
            .iter()
            .map(|key| key.columns(conn))
            .collect::<Result<Vec<Arc<Vec<Column>>>, WebCodeGenError>>()?;

        let table = self.table(conn)?;
        let local_columns = self.columns(conn)?;
        let foreign_columns = self.foreign_columns(conn)?;

        // For each foreign key in the host table, we check whether it contains
        // references to the specific ID contained in the local & foreign columns
        // of the current constraint, which implicitly also checks whether the
        // foreign key points to the same foreign table as the current constraint.
        // Next, we check whether the foreign key's foreign columns contain
        // any of the columns pointing to ancestors of the host table described
        // in constraints to ancestors of the host table which we determined above.
        for foreign_key in table.foreign_keys(conn)?.as_ref() {
            // We retrieve the local columns of the foreign key we are checking.
            let fk_local_columns = foreign_key.columns(conn)?;
            // If all of the columns involved in the current constraint are
            // present in the local columns of the foreign key, we proceed
            // to check the foreign columns.
            if !local_columns.iter().all(|c| fk_local_columns.contains(c)) {
                continue;
            }

            let fk_foreign_columns = foreign_key.foreign_columns(conn)?;
            // All of the foreign columns of the current constraint must
            // be present in the foreign columns of the foreign key.
            if !foreign_columns.iter().all(|c| fk_foreign_columns.contains(c)) {
                continue;
            }

            // Now that we have established that the foreign key involves
            // all of the local & foreign columns of the current constraint,
            // we need to find at least one case where the foreign key's
            // foreign columns contain all of the columns in a `columns_to_local_ancestors`.
            for columns_to_local_ancestor in &columns_to_local_ancestors {
                if columns_to_local_ancestor.iter().all(|c| fk_foreign_columns.contains(c)) {
                    return Ok(Some((PartialBuilderKind::Mandatory, foreign_key.clone())));
                }
            }
        }

        // In the case of a discretional partial builder constraint, we also need
        // to check that there exist only one `keys_to_local_ancestors`, otherwise
        // it would not be clear which of these columns to set.
        assert_eq!(
            keys_to_local_ancestors.len(),
            1,
            "Discretional partial builder constraints must have exactly one key to a local ancestor."
        );

        Ok(Some((PartialBuilderKind::Discretional, keys_to_local_ancestors[0].clone())))
    }