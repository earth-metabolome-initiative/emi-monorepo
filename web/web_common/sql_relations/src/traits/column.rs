

#[cached(result = true, key = "String", convert = r#"{ format!("{column}") }"#)]
fn all_ancestral_same_as_columns(
    column: &Column,
    conn: &mut PgConnection,
) -> Result<Vec<Column>, WebCodeGenError> {
    let mut reachable_set = column.ancestral_same_as_reachable_set(conn)?;
    // The frontier contains the set of columns which so far can only be reached
    // from the current column. Once a column in the frontier is found to be
    // reachable from another column in the reachable set, it is marked as true
    // and will not be used to expand the reachable set anymore.
    let mut frontier: HashMap<Column, bool> =
        column.ancestral_same_as_columns(conn)?.into_iter().map(|c| (c, false)).collect();
    let table = column.table(conn)?;
    let ancestral_extension_tables = table.ancestral_extension_tables(conn)?;
    let mut changed = true;

    while changed {
        changed = false;
        for ancestor in ancestral_extension_tables.iter() {
            for ancestor_column in ancestor.columns(conn)?.iter() {
                // If the ancestor node is already in the reachable set, skip it.
                if reachable_set.contains(ancestor_column) {
                    continue;
                }

                let ancestor_reachable_set =
                    ancestor_column.ancestral_same_as_reachable_set(conn)?;

                // We update the frontier to mark as true columns which we have now discovered
                // can be reached also from this ancestor column.
                for (frontier_column, is_reachable) in &mut frontier {
                    if !*is_reachable && ancestor_reachable_set.contains(frontier_column) {
                        *is_reachable = true;
                        changed = true;
                    }
                }

                // If the ancestor reachable set intersects with the current reachable
                // set, then the ancestor column is inferred to be ancestrally same-as
                // the current column. We then merge the ancestor reachable set into
                // the current reachable set, and add the ancestor column to the
                // frontier.
                if !reachable_set.is_disjoint(&ancestor_reachable_set) {
                    reachable_set.insert(ancestor_column.clone());
                    frontier.insert(ancestor_column.clone(), false);
                    reachable_set.extend(ancestor_reachable_set);
                    changed = true;
                }
            }
        }
    }

    // We then consider as ancestrally same-as only those columns in the frontier
    // which are still marked as false, meaning they could not be reached
    // from any other column in the reachable set.
    let mut ancestral_same_as_columns = frontier
        .into_iter()
        .filter_map(|(column, is_reachable)| if is_reachable { None } else { Some(column) })
        .collect::<Vec<_>>();
    ancestral_same_as_columns.sort_unstable();

    Ok(ancestral_same_as_columns)
}


    /// Returns whether the current column is ancestrally same-as the provided
    /// column.
    ///
    /// # Implementative details
    ///
    /// A column is ancestrally same-as another column if:
    ///
    /// - The two columns are linked by a same-as constraint or inferred to be.
    /// - The same-as constraint includes the primary key of the table of the
    ///   current column.
    ///
    /// # Arguments
    ///
    /// * `other` - The other column to compare with
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_ancestrally_same_as(
        &self,
        other: &Column,
        conn: &mut PgConnection,
    ) -> Result<bool, crate::error::Error> {
        self.all_ancestral_same_as_columns(conn).map(|columns| columns.contains(other))
    }

    /// Returns the distinct check constraints that apply to the column.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn distinct_check_constraints(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<(Column, CheckConstraint)>, crate::error::Error> {
        let check_constraints = self.check_constraints(conn)?;
        let mut distinct_check_constraints = Vec::new();

        for check_constraint in check_constraints.iter() {
            if check_constraint.is_distinct_constraint(conn)?.is_some() {
                let involved_columns = check_constraint.columns(conn)?;
                let Some(distinct_column) = involved_columns.iter().find(|c| c != &self) else {
                    continue;
                };
                distinct_check_constraints
                    .push((distinct_column.clone(), check_constraint.clone()));
            }
        }

        Ok(distinct_check_constraints)
    }

    /// Returns the inherited distinct check constraints that apply to the
    /// column.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn inherited_distinct_check_constraints(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<(Column, CheckConstraint)>, crate::error::Error> {
        let mut inherited_distinct_check_constraints = HashSet::new();
        let mut distinct_columns_and_constraints = Vec::new();
        let table_columns = self.table(conn)?.columns(conn)?;
        let mut possible_matching_columns = Vec::new();

        for column in table_columns.iter() {
            if column == self {
                continue;
            }
            if self.has_compatible_data_type(column, conn)? {
                possible_matching_columns.push(column);
            }
        }

        for column in self.all_ancestral_same_as_columns(conn)? {
            for (distinct_column, distinct_check_constraint) in
                column.distinct_check_constraints(conn)?
            {
                // If we have already added this constraint, skip it
                if inherited_distinct_check_constraints.contains(&distinct_check_constraint) {
                    continue;
                }

                for possible_matching_column in &possible_matching_columns {
                    if possible_matching_column.is_ancestrally_same_as(&distinct_column, conn)? {
                        distinct_columns_and_constraints.push((
                            (*possible_matching_column).clone(),
                            distinct_check_constraint.clone(),
                        ));
                    }
                }

                inherited_distinct_check_constraints.insert(distinct_check_constraint);
            }
        }

        Ok(distinct_columns_and_constraints)
    }

    /// Returns all distinct check constraints that apply to the column,
    /// including inherited ones.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn all_distinct_check_constraints(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<(Column, CheckConstraint)>, crate::error::Error> {
        let mut all_distinct_check_constraints = self.distinct_check_constraints(conn)?;
        all_distinct_check_constraints.extend(self.inherited_distinct_check_constraints(conn)?);
        Ok(all_distinct_check_constraints)
    }

    /// Returns whether the column has exactly one foreign key constraint
    /// that references exactly one column.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn has_singleton_foreign_key(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, crate::error::Error> {
        Ok(self.foreign_keys(conn)?.iter().any(|key| key.is_singleton(conn).unwrap_or(false)))
    }


    /// Returns the ancestral same-as constraints for the column, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn ancestral_same_as_constraints(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<KeyColumnUsage>, crate::error::Error> {
        Ok(self
            .foreign_keys(conn)?
            .iter()
            .filter(|foreign_key| {
                foreign_key
                    .is_ancestral_same_as_constraint(conn)
                    .map(|index| index.is_some())
                    .unwrap_or(false)
            })
            .cloned()
            .collect())
    }

    /// Returns the associated same-as constraints for the column, if any.
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
    pub fn associated_same_as_constraints(
        &self,
        include_local_primary_key: bool,
        conn: &mut PgConnection,
    ) -> Result<Vec<KeyColumnUsage>, crate::error::Error> {
        let mut associated_same_as_constraints = Vec::new();

        for foreign_key in self.foreign_keys(conn)?.iter() {
            if foreign_key
                .is_associated_same_as_constraint(include_local_primary_key, conn)?
                .is_none()
            {
                continue;
            }
            associated_same_as_constraints.push(foreign_key.clone());
        }

        Ok(associated_same_as_constraints)
    }

    /// Returns whether the current column is a foreign definer.
    ///
    /// # Arguments
    ///
    /// * `include_mandatory_partial_builder`: Whether to include mandatory
    ///   partial builders
    /// * `conn`: A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn is_foreign_definer(
        &self,
        include_mandatory_partial_builder: bool,
        conn: &mut PgConnection,
    ) -> Result<bool, crate::error::Error> {
        Ok(!self.foreign_definer_constraints(include_mandatory_partial_builder, conn)?.is_empty())
    }


/// Returns the subset of the table's columns which define other
/// columns' values via foreign key constraints.
///
/// # Arguments
///
/// * `include_mandatory_partial_builders` - Whether to include columns that
///   require a partial builder.
/// * `conn` - The database connection.
///
/// # Errors
///
/// * If the columns cannot be loaded from the database.
pub(crate) fn foreign_definer_columns(
    &self,
    include_mandatory_partial_builders: bool,
    conn: &mut PgConnection,
) -> Result<Vec<Column>, WebCodeGenError> {
    let mut foreign_definer_columns = Vec::new();
    for column in self.columns(conn)?.as_ref() {
        if column.is_foreign_definer(include_mandatory_partial_builders, conn)? {
            foreign_definer_columns.push(column.clone());
        }
    }
    Ok(foreign_definer_columns)
}


    /// Returns whether the column is foreignely defined.
    ///
    /// # Arguments
    ///
    /// * `include_mandatory_partial_builder`: Whether to include mandatory
    ///   partial builders
    /// * `conn`: A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn is_foreignely_defined(
        &self,
        include_mandatory_partial_builder: bool,
        conn: &mut PgConnection,
    ) -> Result<bool, crate::error::Error> {
        Ok(!self.foreign_definer_columns(include_mandatory_partial_builder, conn)?.is_empty())
    }

    pub(crate) fn foreign_definer_constraints(
        &self,
        include_mandatory_partial_builder: bool,
        conn: &mut PgConnection,
    ) -> Result<Vec<KeyColumnUsage>, crate::error::Error> {
        if !include_mandatory_partial_builder
            && let Some((PartialBuilderKind::Mandatory, _, _)) =
                self.requires_partial_builder(conn)?
        {
            return Ok(Vec::new());
        }

        let mut foreign_definer_constraints = Vec::new();
        for foreign_key in self.foreign_keys(conn)?.iter() {
            let foreign_table = foreign_key.foreign_table(conn)?;
            if self.is_foreign_primary_key_of_table(&foreign_table, conn)?.is_none() {
                continue;
            }
            if foreign_key.includes_local_primary_key(conn)? {
                continue;
            }
            if !foreign_key.includes_foreign_primary_key(conn)? {
                continue;
            }
            if foreign_table.primary_key_columns(conn)?.len()
                == foreign_key.foreign_columns(conn)?.len()
            {
                continue;
            }
            if foreign_key.is_same_as_constraint(conn)?.is_some() {
                foreign_definer_constraints.push(foreign_key.clone());
            }
        }

        Ok(foreign_definer_constraints)
    }

    /// Returns a map of foreign definer constraints by the table they belong
    /// to.
    ///
    /// # Arguments
    ///
    /// * `include_mandatory_partial_builder`: Whether to include mandatory
    ///   partial builders
    /// * `conn`: A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn foreign_definer_constraints_by_table(
        &self,
        include_mandatory_partial_builder: bool,
        conn: &mut PgConnection,
    ) -> Result<HashMap<Arc<Table>, Vec<KeyColumnUsage>>, crate::error::Error> {
        let mut foreign_definer_constraints_by_table = HashMap::new();
        for foreign_definer_constraint in
            self.foreign_definer_constraints(include_mandatory_partial_builder, conn)?
        {
            let foreign_table = foreign_definer_constraint.foreign_table(conn)?;
            foreign_definer_constraints_by_table
                .entry(foreign_table)
                .or_insert_with(Vec::new)
                .push(foreign_definer_constraint);
        }
        Ok(foreign_definer_constraints_by_table)
    }

    /// Returns the set of columns which are uniquely defined by values
    /// associated with the foreign table associated with the column.
    ///
    /// # Arguments
    ///
    /// * `include_mandatory_partial_builder` - Whether to include mandatory
    ///   partial builders
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn foreign_defined_columns(
        &self,
        include_mandatory_partial_builder: bool,
        conn: &mut PgConnection,
    ) -> Result<Vec<Column>, crate::error::Error> {
        let mut foreign_defined_columns = Vec::new();
        for foreign_definer_constraint in
            self.foreign_definer_constraints(include_mandatory_partial_builder, conn)?
        {
            foreign_defined_columns.extend(
                foreign_definer_constraint.columns(conn)?.iter().filter(|c| c != &self).cloned(),
            );
        }

        foreign_defined_columns.sort_unstable();

        Ok(foreign_defined_columns)
    }

    /// Returns the set of columns which uniquely define the current column.
    ///
    /// # Arguments
    ///
    /// * `include_mandatory_partial_builder` - Whether to include mandatory
    ///   partial builders
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn foreign_definer_columns(
        &self,
        include_mandatory_partial_builder: bool,
        conn: &mut PgConnection,
    ) -> Result<Vec<Column>, crate::error::Error> {
        let mut foreign_definer_columns = Vec::new();
        let table = self.table(conn)?;
        for column in table.columns(conn)?.iter() {
            if column == self {
                continue;
            }
            if column
                .foreign_defined_columns(include_mandatory_partial_builder, conn)?
                .contains(self)
            {
                foreign_definer_columns.push(column.clone());
            }
        }
        Ok(foreign_definer_columns)
    }

    /// Returns the ancestral same-as columns for the column, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn ancestral_same_as_columns(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Column>, crate::error::Error> {
        self.ancestral_same_as_constraints(conn)?
            .into_iter()
            .map(|constraint| {
                let local_columns = constraint.columns(conn)?;
                let foreign_columns = constraint.foreign_columns(conn)?;
                Ok(local_columns
                    .iter()
                    .zip(foreign_columns.iter())
                    .find_map(|(local_column, foreign_column)| {
                        if local_column == self { Some(foreign_column.clone()) } else { None }
                    })
                    .unwrap())
            })
            .collect::<Result<Vec<Column>, crate::error::Error>>()
    }

    /// Returns the ancestral same-as reachable set for the column, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn ancestral_same_as_reachable_set(
        &self,
        conn: &mut PgConnection,
    ) -> Result<HashSet<Column>, crate::error::Error> {
        ancestral_same_as_reachable_set(self, conn)
    }

    /// Returns the associated same-as columns for the column, if any.
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
    pub(crate) fn associated_same_as_columns(
        &self,
        include_local_primary_key: bool,
        conn: &mut PgConnection,
    ) -> Result<Vec<(Column, KeyColumnUsage)>, crate::error::Error> {
        associated_same_as_columns(self, include_local_primary_key, conn)
    }

    /// Returns the normal and inferred ancestral same-as constraints for the
    /// column, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn all_ancestral_same_as_columns(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Self>, crate::error::Error> {
        all_ancestral_same_as_columns(self, conn)
    }



    /// Returns whether the column is to be handled as a partial builder.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    ///
    /// # Implementation details
    ///
    /// A column must be handled as a partial builder if:
    /// - The column C from table T is not nullable
    /// - The table T is an extension table, meaning it extends some other table
    ///   E.
    /// - The column C from table T is a foreign key to the primary key of the
    ///   table F, and F != E.
    /// - The table F has a same-as UNIQUE index constraint on the primary key
    ///   of the table E.
    /// - The table T has a foreign key same-as constraint to the same-as UNIQUE
    ///   index constraint of the table F.
    pub fn requires_partial_builder(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<(PartialBuilderKind, KeyColumnUsage, KeyColumnUsage)>, crate::error::Error> {
        let mut partial_builder_foreign_keys = Vec::new();

        for foreign_key in self.foreign_keys(conn)?.as_ref() {
            let Some((partial_builder_kind, potential_same_as_constraint)) =
                foreign_key.is_partial_builder_constraint(conn)?
            else {
                continue;
            };
            partial_builder_foreign_keys.push((
                partial_builder_kind,
                potential_same_as_constraint,
                foreign_key.clone(),
            ));
        }

        if partial_builder_foreign_keys.len() > 1 {
            unreachable!(
                "Column {self} seems to be is part of {} partial builder constraints, which is not supported. The builders include the columns: {}",
                partial_builder_foreign_keys.len(),
                partial_builder_foreign_keys
                    .iter()
                    .map(|(_, _, key)| {
                        key.columns(conn)
                            .unwrap_or_default()
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(", ")
                    })
                    .collect::<Vec<_>>()
                    .join(" | ")
            );
        }

        Ok(partial_builder_foreign_keys.pop())
    }


    /// Returns whether the column contains the update user and is defined by
    /// the SESSION user
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_updated_by(&self, conn: &mut PgConnection) -> Result<bool, crate::error::Error> {
        Ok(self.column_name == "updated_by"
            && self.foreign_keys(conn)?.iter().any(|key| {
                let Ok(foreign_columns) = key.foreign_columns(conn) else {
                    return false;
                };

                foreign_columns.len() == 1
                    && foreign_columns[0].table_name == "users"
                    && foreign_columns[0].column_name == "id"
            }))
    }

    /// Returns whether the column contains the creation user and is defined by
    /// the SESSION user
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_created_by(&self, conn: &mut PgConnection) -> Result<bool, crate::error::Error> {
        Ok(self.column_name == "created_by"
            && self.foreign_keys(conn)?.iter().any(|key| {
                let Ok(foreign_columns) = key.foreign_columns(conn) else {
                    return false;
                };

                foreign_columns.len() == 1
                    && foreign_columns[0].table_name == "users"
                    && foreign_columns[0].column_name == "id"
            }))
    }

    /// Returns whether the column contains the most concrete table variant in
    /// an extension hierarchy
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_most_concrete_table(&self, conn: &mut PgConnection) -> Result<bool, crate::error::Error> {
        Ok(self.column_name == "most_concrete_table"
            && self.data_type == "text"
            && !self.is_nullable()
            && !self.table(conn)?.is_extension(conn)?)
    }

    #[must_use]
    /// Returns whether the column is a timestamp which has to be updated at
    /// each update operation
    pub fn is_updated_at(&self) -> bool {
        self.column_name == "updated_at" && self.data_type == "timestamp with time zone"
    }

    #[must_use]
    /// Returns whether the column is a timestamp which has to be set at the
    /// insert operation
    pub fn is_created_at(&self) -> bool {
        self.column_name == "created_at" && self.data_type == "timestamp with time zone"
    }


    /// Returns whether the column is compatible with the provided column
    ///
    /// # Arguments
    ///
    /// * `column` - A reference to a `Column`
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    /// * If the underlying data type of the column is not compatible
    ///
    /// # Implementative details
    ///
    /// The two columns are considered compatible if their data type is the
    /// same, and if they have shared ancestors.
    pub fn has_compatible_data_type(
        &self,
        column: &Column,
        conn: &mut PgConnection,
    ) -> Result<bool, crate::error::Error> {
        if self.str_rust_data_type(conn)? != column.str_rust_data_type(conn)? {
            return Ok(false);
        }

        let mut local_referenced_tables: Vec<Table> = self
            .referenced_by_foreign_keys(conn)?
            .into_iter()
            .map(|table| table.as_ref().clone())
            .collect::<Vec<_>>();
        let mut other_referenced_tables: Vec<Table> = column
            .referenced_by_foreign_keys(conn)?
            .into_iter()
            .map(|table| table.as_ref().clone())
            .collect::<Vec<_>>();

        if local_referenced_tables.is_empty() && other_referenced_tables.is_empty() {
            // If both columns are not foreign keys, they are compatible.
            return Ok(true);
        }

        // We determine the set of ancestors of the referenced tables.
        let local_referenced_ancestors = local_referenced_tables
            .iter()
            .flat_map(|table| {
                table.ancestral_extension_tables(conn).unwrap_or_default().as_ref().clone()
            })
            .collect::<Vec<_>>();
        let other_referenced_ancestors = other_referenced_tables
            .iter()
            .flat_map(|table| {
                table.ancestral_extension_tables(conn).unwrap_or_default().as_ref().clone()
            })
            .collect::<Vec<_>>();

        // We extend the referenced tables with their ancestors.
        local_referenced_tables.extend(local_referenced_ancestors);
        other_referenced_tables.extend(other_referenced_ancestors);

        Ok(local_referenced_tables.iter().any(|table| other_referenced_tables.contains(table)))
    }


    /// Returns the string data type
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn str_rust_data_type(
        &self,
        conn: &mut PgConnection,
    ) -> Result<String, crate::error::Error> {
        str_rust_data_type(self, conn)
    }



    /// Returns whether the column is a foreign primary key of the provided
    /// table.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_foreign_primary_key_of_table(
        &self,
        table: &Table,
        conn: &mut PgConnection,
    ) -> Result<Option<KeyColumnUsage>, crate::error::Error> {
        for foreign_primary_key in self.foreign_primary_keys(conn)? {
            let foreign_table = foreign_primary_key.foreign_table(conn)?;
            if foreign_table.as_ref() == table || foreign_table.is_extending(table, conn)? {
                return Ok(Some(foreign_primary_key));
            }
        }
        Ok(None)
    }



    /// Returns whether the column is part of an extension primary key
    /// constraint.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_part_of_extension_primary_key(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<KeyColumnUsage>, crate::error::Error> {
        Ok(self
            .foreign_keys(conn)?
            .iter()
            .find(|key| key.is_extension(conn).unwrap_or(false))
            .cloned())
    }

    /// Returns the set of unique tables that are referenced by foreign keys
    /// associated to the current column where the current column is a primary
    /// key in the foreign table.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn referenced_by_foreign_keys(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Table>, crate::error::Error> {
        let mut referenced_tables = Vec::new();

        if self.is_primary_key(conn)? {
            let table = self.table(conn)?;
            referenced_tables.push(table);
        }

        for key in self.foreign_keys(conn)?.iter() {
            if key.is_foreign_primary_key(conn)? {
                let foreign_table = key.foreign_table(conn)?;
                referenced_tables.push(foreign_table);
            }
        }

        referenced_tables.sort_unstable();
        referenced_tables.dedup();

        Ok(referenced_tables)
    }