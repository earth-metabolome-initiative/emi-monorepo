use cached::proc_macro::cached;
use diesel::{
    ExpressionMethods, PgConnection, QueryDsl, Queryable, QueryableByName, RunQueryDsl, Selectable,
};
use proc_macro2::TokenStream;
use syn::Ident;

use super::{Column, Table};
use crate::{ReferentialConstraint, errors::WebCodeGenError};

#[cached(
    result = true,
    key = "String",
    convert = r#"{ format!("{}-{}-{}-{}-{}-{}", key_column_usage.constraint_catalog, key_column_usage.constraint_schema, key_column_usage.constraint_name, key_column_usage.table_catalog, key_column_usage.table_schema, key_column_usage.table_name) }"#
)]
fn referential_constraint(
    key_column_usage: &KeyColumnUsage,
    conn: &mut PgConnection,
) -> Result<ReferentialConstraint, diesel::result::Error> {
    use diesel::SelectableHelper;

    use crate::schema::referential_constraints;
    referential_constraints::table
        .filter(referential_constraints::constraint_name.eq(&key_column_usage.constraint_name))
        .filter(referential_constraints::constraint_schema.eq(&key_column_usage.constraint_schema))
        .filter(
            referential_constraints::constraint_catalog.eq(&key_column_usage.constraint_catalog),
        )
        .select(ReferentialConstraint::as_select())
        .first::<ReferentialConstraint>(conn)
}

#[cached(
    result = true,
    key = "String",
    convert = r#"{ format!("{}-{}-{}-{}-{}-{}", key_column_usage.constraint_catalog, key_column_usage.constraint_schema, key_column_usage.constraint_name, key_column_usage.table_catalog, key_column_usage.table_schema, key_column_usage.table_name) }"#
)]
fn foreign_columns(
    key_column_usage: &KeyColumnUsage,
    conn: &mut PgConnection,
) -> Result<Vec<Column>, diesel::result::Error> {
    use diesel::{BoolExpressionMethods, JoinOnDsl, SelectableHelper};

    use crate::schema::{columns, constraint_column_usage};

    // Find the referential constraint for this key_column_usage
    let referential_constraint = key_column_usage.referential_constraint(conn)?;

    // Find the columns in the referenced (unique) constraint
    constraint_column_usage::table
        .filter(
            constraint_column_usage::constraint_catalog.eq(referential_constraint
                .unique_constraint_catalog
                .ok_or(diesel::result::Error::NotFound)?),
        )
        .filter(
            constraint_column_usage::constraint_schema.eq(referential_constraint
                .unique_constraint_schema
                .ok_or(diesel::result::Error::NotFound)?),
        )
        .filter(constraint_column_usage::constraint_name.eq(
            referential_constraint.unique_constraint_name.ok_or(diesel::result::Error::NotFound)?,
        ))
        .inner_join(
            columns::table.on(columns::table_name
                .eq(constraint_column_usage::table_name)
                .and(columns::table_schema.eq(constraint_column_usage::table_schema))
                .and(columns::table_catalog.eq(constraint_column_usage::table_catalog))
                .and(columns::column_name.eq(constraint_column_usage::column_name))),
        )
        .order_by(columns::ordinal_position.asc())
        .select(Column::as_select())
        .load::<Column>(conn)
}

#[cached(
    result = true,
    key = "String",
    convert = r#"{ format!("{}-{}-{}-{}-{}-{}", key_column_usage.constraint_catalog, key_column_usage.constraint_schema, key_column_usage.constraint_name, key_column_usage.table_catalog, key_column_usage.table_schema, key_column_usage.table_name) }"#
)]
fn columns(
    key_column_usage: &KeyColumnUsage,
    conn: &mut PgConnection,
) -> Result<Vec<Column>, diesel::result::Error> {
    use diesel::{BoolExpressionMethods, JoinOnDsl, SelectableHelper};

    use crate::schema::{columns, key_column_usage};
    key_column_usage::table
        .filter(key_column_usage::constraint_name.eq(&key_column_usage.constraint_name))
        .filter(key_column_usage::constraint_schema.eq(&key_column_usage.constraint_schema))
        .filter(key_column_usage::constraint_catalog.eq(&key_column_usage.constraint_catalog))
        .inner_join(
            columns::table.on(columns::table_name
                .eq(key_column_usage::table_name)
                .and(columns::table_schema.eq(key_column_usage::table_schema))
                .and(columns::table_catalog.eq(key_column_usage::table_catalog))
                .and(columns::column_name.eq(key_column_usage::column_name))),
        )
        .order_by(key_column_usage::ordinal_position.asc())
        .select(Column::as_select())
        .load::<Column>(conn)
}

#[cached(
    result = true,
    key = "String",
    convert = r#"{ format!("{}-{}-{}-{}-{}-{}", key_column_usage.constraint_catalog, key_column_usage.constraint_schema, key_column_usage.constraint_name, key_column_usage.table_catalog, key_column_usage.table_schema, key_column_usage.table_name) }"#
)]
/// Returns the foreign table associated with this key column usage
///
/// # Arguments
///
/// * `conn` - A mutable reference to a `PgConnection`
///
/// # Errors
///
/// * If an error occurs while loading the foreign table from the database
fn foreign_table(
    key_column_usage: &KeyColumnUsage,
    conn: &mut PgConnection,
) -> Result<Option<Table>, diesel::result::Error> {
    use diesel::{BoolExpressionMethods, JoinOnDsl, OptionalExtension, SelectableHelper};

    use crate::schema::{constraint_table_usage, tables};

    let constraint = key_column_usage.referential_constraint(conn)?;

    constraint_table_usage::table
        .inner_join(
            tables::table.on(tables::table_name
                .eq(constraint_table_usage::table_name)
                .and(tables::table_schema.eq(constraint_table_usage::table_schema))
                .and(tables::table_catalog.eq(constraint_table_usage::table_catalog))),
        )
        .filter(constraint_table_usage::constraint_name.eq(&constraint.constraint_name))
        .filter(constraint_table_usage::constraint_schema.eq(&constraint.constraint_schema))
        .filter(constraint_table_usage::constraint_catalog.eq(&constraint.constraint_catalog))
        .select(Table::as_select())
        .first::<Table>(conn)
        .optional()
}

/// Represents a row in the `key_column_usage` table, which contains information
/// about columns that are constrained by a unique or primary key constraint.
///
/// For more details, see [`PostgreSQL`](https://www.postgresql.org/docs/current/infoschema-key-column-usage.html)
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[diesel(table_name = crate::schema::key_column_usage)]
pub struct KeyColumnUsage {
    /// The name of the database that contains the constraint.
    pub constraint_catalog: String,
    /// The name of the schema that contains the constraint.
    pub constraint_schema: String,
    /// The name of the constraint.
    pub constraint_name: String,
    /// The name of the database that contains the table.
    pub table_catalog: String,
    /// The name of the schema that contains the table.
    pub table_schema: String,
    /// The name of the table that contains the column.
    pub table_name: String,
    /// The name of the column that is constrained.
    pub column_name: String,
    /// The position of the column within the constraint.
    pub ordinal_position: i32,
    /// The position of the column within the unique constraint, if applicable.
    pub position_in_unique_constraint: Option<i32>,
}

impl KeyColumnUsage {
    /// Load all the key column usages from the database
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Vec` of `KeyColumnUsage` if the operation was
    /// successful, or a `WebCodeGenError` if an error occurred
    ///
    /// # Errors
    /// If an error occurs while loading the key column usages from the database
    pub fn load_all_key_column_usages(
        conn: &mut PgConnection,
    ) -> Result<Vec<Self>, WebCodeGenError> {
        use crate::schema::key_column_usage;
        key_column_usage::table.load::<KeyColumnUsage>(conn).map_err(WebCodeGenError::from)
    }

    /// Load all the key column usages from the database
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    /// * `table_name` - The name of the table to load the key column usages for
    /// * `table_schema` - An optional schema name to filter the key column
    ///   usages by
    /// * `table_catalog` - The name of the catalog to filter the key column
    ///   usages by
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Vec` of `KeyColumnUsage` if the operation was
    /// successful, or a `WebCodeGenError` if an error occurred
    ///
    /// # Errors
    ///
    /// If an error occurs while loading the key column usages from the database
    pub fn load_key_column_usages(
        conn: &mut PgConnection,
        table_name: &str,
        table_schema: Option<&str>,
        table_catalog: &str,
    ) -> Result<Vec<Self>, WebCodeGenError> {
        use crate::schema::key_column_usage;
        let table_schema = table_schema.unwrap_or("public");
        key_column_usage::table
            .filter(key_column_usage::table_name.eq(table_name))
            .filter(key_column_usage::table_schema.eq(table_schema))
            .filter(key_column_usage::table_catalog.eq(table_catalog))
            .load::<KeyColumnUsage>(conn)
            .map_err(WebCodeGenError::from)
    }

    /// Returns the referential constraint associated with this key column usage
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the referential constraint from the
    ///   database
    pub fn referential_constraint(
        &self,
        conn: &mut PgConnection,
    ) -> Result<ReferentialConstraint, diesel::result::Error> {
        referential_constraint(self, conn)
    }

    /// Returns the table associated with this key column usage
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the table from the database
    pub fn table(&self, conn: &mut PgConnection) -> Result<Table, diesel::result::Error> {
        use diesel::SelectableHelper;

        use crate::schema::tables;

        tables::table
            .filter(tables::table_name.eq(&self.table_name))
            .filter(tables::table_schema.eq(&self.table_schema))
            .filter(tables::table_catalog.eq(&self.table_catalog))
            .select(Table::as_select())
            .first::<Table>(conn)
    }

    /// Returns the foreign table associated with this key column usage
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the foreign table from the database
    pub fn foreign_table(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<Table>, diesel::result::Error> {
        foreign_table(self, conn)
    }

    /// Returns all the columns involved in the constraint
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the key column usages from the
    ///   database
    pub fn columns(&self, conn: &mut PgConnection) -> Result<Vec<Column>, diesel::result::Error> {
        columns(self, conn)
    }

    /// Returns whether it is a composite key column usage
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the key column usages from the
    ///   database
    pub fn is_composite(&self, conn: &mut PgConnection) -> Result<bool, diesel::result::Error> {
        self.columns(conn).map(|columns| columns.len() > 1)
    }

    /// Returns whether any column involved in the constraint is nullable
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the key column usages from the
    ///   database
    pub fn is_nullable(&self, conn: &mut PgConnection) -> Result<bool, diesel::result::Error> {
        self.columns(conn).map(|columns| columns.iter().any(Column::is_nullable))
    }

    /// Returns whether any foreign column involved in the constraint is
    /// nullable
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the foreign key column usages from
    ///   the database
    pub fn foreign_is_nullable(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, diesel::result::Error> {
        self.foreign_columns(conn).map(|columns| columns.iter().any(Column::is_nullable))
    }

    /// Returns the columns in the foreign table that are referenced by this key
    /// column usage
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the foreign key column usages from
    ///   the database
    pub fn foreign_columns(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Column>, diesel::result::Error> {
        foreign_columns(self, conn)
    }

    /// Returns whether the key column usage refers to a foreign primary key
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_foreign_primary_key(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        let foreign_table = self.foreign_table(conn)?;

        if let Some(foreign_table) = foreign_table {
            // Check if the foreign table has a primary key
            let primary_keys = foreign_table.primary_key_columns(conn)?;
            let foreign_columns = self.foreign_columns(conn)?;
            Ok(primary_keys == foreign_columns)
        } else {
            Ok(false)
        }
    }

    /// Returns whether the key column usage refers to a foreign unique key
    /// constraint
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_foreign_unique_key(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        let foreign_table = self.foreign_table(conn)?;
        let foreign_columns = self.foreign_columns(conn)?;

        if let Some(foreign_table) = foreign_table {
            // Check if the foreign table has a unique key constraint
            let unique_constraints = foreign_table.unique_indices(conn)?;
            for unique_constraint in unique_constraints {
                // Check if the foreign columns match the unique constraint columns
                let unique_columns = unique_constraint.columns(conn)?;
                if unique_columns.len() == foreign_columns.len()
                    && unique_columns.iter().all(|c| foreign_columns.contains(c))
                {
                    return Ok(true);
                }
            }
            Ok(false)
        } else {
            Ok(false)
        }
    }

    /// Returns the number of columns in this key column usage
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn number_of_columns(&self, conn: &mut PgConnection) -> Result<usize, WebCodeGenError> {
        let columns = self.columns(conn)?;
        Ok(columns.len())
    }

    /// Returns the standardized getter name for this key column usage
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while retrieving the getter name
    pub fn getter_name(&self, conn: &mut PgConnection) -> Result<String, WebCodeGenError> {
        let columns = self.columns(conn)?;
        if columns.len() == 1 {
            // If there is only one column, we use the column name as the getter name
            return columns[0].getter_name();
        }

        Ok(self.constraint_name.clone())
    }

    /// Returns the identifier for this key column usage getter.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while retrieving the getter name
    pub fn getter_ident(&self, conn: &mut PgConnection) -> Result<Ident, WebCodeGenError> {
        let getter_name = self.getter_name(conn)?;
        Ok(Ident::new(&getter_name, proc_macro2::Span::call_site()))
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
    pub fn where_statement(
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
        let foreign_key_table = self.foreign_table(conn)?.unwrap();
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
    ) -> Result<Option<Column>, WebCodeGenError> {
        // First, we check whether the foreign key is a composite foreign key
        if !self.is_composite(conn)? {
            // If the foreign key is not a composite foreign key, we skip it
            return Ok(None);
        }

        // Next, we check whether the foreign key is referring to a UNIQUE constraint
        if !self.is_foreign_unique_key(conn)? {
            // If the foreign key is not referring to a UNIQUE constraint, we skip it
            return Ok(None);
        }

        let foreign_key_columns = self.foreign_columns(conn)?;

        // Finally, if the UNIQUE constraint is composed of the primary key
        // of the foreign table and the foreign column
        // associated with the current column, we consider it a `same-as`
        // constraint.
        let Some(foreign_table) = self.foreign_table(conn)? else {
            unreachable!(
                "Column \"{}\".\"{}\" is part of a composite foreign key which refers to a UNIQUE constraint, but the foreign table could not be found.",
                self.table_name, self.column_name
            );
        };

        // We identify the foreign column corresponding to the current column.
        for foreign_column in &foreign_key_columns {
            let mut expected_foreign_columns = foreign_table.primary_key_columns(conn)?;

            // If the foreign column is not part of the foreign table's primary key, we add
            // it
            if !expected_foreign_columns.contains(foreign_column) {
                expected_foreign_columns.push(foreign_column.clone());
            }

            // If the foreign key's foreign columns are not the same as the expected foreign
            // columns, then this constraint is not a `same-as` constraint.
            if foreign_key_columns == expected_foreign_columns {
                return Ok(Some(foreign_column.clone()));
            }
        }

        Ok(None)
    }
}
