use diesel::{
    pg::PgConnection, BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, Queryable,
    QueryableByName, RunQueryDsl, Selectable, SelectableHelper,
};

use super::{Column, PgConstraint, PgExtension, PgProc};
use crate::errors::WebCodeGenError;

use super::Column;

#[derive(Queryable, QueryableByName, Debug, Selectable)]
#[diesel(table_name = crate::schema::check_constraints)]
/// A struct representing a check constraint
pub struct CheckConstraint {
    /// The name of the constraint catalog
    pub constraint_catalog: String,
    /// The name of the constraint schema
    pub constraint_schema: String,
    /// The name of the constraint
    pub constraint_name: String,
    /// The check clause of the constraint
    pub check_clause: String,
}

impl CheckConstraint {
    /// Returns the [`TokenStream`](proc_macro2::TokenStream) of the check
    /// clause, including all functions specified from the provided
    /// extensions, and considering the provided column as the one to
    /// be primarily checked.
    ///
    /// # Arguments
    ///
    /// * `column` - The column to be primarily checked
    /// * `extensions` - The extensions to consider
    /// * `use_self` - Whether the other columns should be assumed as from the
    ///   `self`
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    ///
    /// # Returns
    ///
    /// * `None` if the provided column is not involved in the check clause
    /// * `None` if the provided `extensions` are not involved in the check
    ///   clause
    pub fn to_syn<E: AsRef<PgExtension>>(
        &self,
        column: &Column,
        extensions: &[E],
        conn: &mut PgConnection,
    ) -> Result<Option<proc_macro2::TokenStream>, WebCodeGenError> {
        let involved_columns = self.columns(conn)?;
        if !involved_columns.contains(column) {
            return Ok(None);
        }
        let involved_functions = self
            .functions(conn)?
            .into_iter()
            .filter(|function| {
                function.extension(conn).ok().flatten().is_some_and(|extension| {
                    extensions.iter().any(|ext| ext.as_ref() == &extension)
                })
            })
            .collect::<Vec<PgProc>>();
        if involved_functions.is_empty() {
            return Ok(None);
        }
        // At this time, we know how to handle either the case of:
        //
        // 1. A single column check constraint with multiple functions, each one
        //    expecting the column.
        // 2. A multi-column check constraint with a single function expecting all the
        //    columns.
        //
        if involved_columns.len() == 1 {
            let column_ident = column.snake_case_ident()?;
            // Case 1 described above.
            return involved_functions
                .into_iter()
                .map(|function| {
                    let function_ident = function.ident();
                    let extension_ident = if let Some(extension) = function.extension(conn)? {
                        extension.ident()
                    } else {
                        unreachable!("The extension must be present")
                    };
                    Ok(quote::quote! {
                        #extension_ident::#function_ident(#column_ident)?;
                    })
                })
                .collect::<Result<proc_macro2::TokenStream, WebCodeGenError>>()
                .map(Some);
        }

        if involved_columns.len() > 1 && involved_functions.len() == 1 {
            // Case 2 described above.
            let function = &involved_functions[0];
            let function_ident = function.ident();
            let extension_ident = if let Some(extension) = function.extension(conn)? {
                extension.ident()
            } else {
                unreachable!("The extension must be present")
            };
            let column_idents = involved_columns
                .iter()
                .map(Column::snake_case_ident)
                .collect::<Result<Vec<_>, WebCodeGenError>>()?;
            return Ok(Some(quote::quote! {
                #extension_ident::#function_ident(#(#column_idents),*)?;
            }));
        }

        unimplemented!("It is currently unsupported to handle the provided check constraint")
    }

    /// Returns the vector of [`PgProc`] functions that are used in the check
    /// clause
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn functions(&self, conn: &mut PgConnection) -> Result<Vec<PgProc>, WebCodeGenError> {
        Ok(self.pg_constraint(conn)?.functions(conn)?)
    }

    /// Returns the [`PgConstraint`] that corresponds to this check constraint
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn pg_constraint(&self, conn: &mut PgConnection) -> Result<PgConstraint, WebCodeGenError> {
        use crate::schema::{pg_constraint, pg_namespace};
        pg_constraint::table
            .inner_join(pg_namespace::table.on(pg_constraint::connamespace.eq(pg_namespace::oid)))
            .filter(
                pg_constraint::conname
                    .eq(&self.constraint_name)
                    .and(pg_constraint::contype.eq("c")),
            )
            .filter(pg_namespace::nspname.eq(&self.constraint_schema))
            .select(PgConstraint::as_select())
            .first(conn)
            .map_err(WebCodeGenError::from)
    }

    /// Load all the check constraints from the database
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Vec` of `CheckConstraint` if the operation was
    /// successful, or a `WebCodeGenError` if an error occurred
    ///
    /// # Errors
    ///
    /// If an error occurs while loading the constraints from the database
    pub fn load_all_check_constraints(
        conn: &mut PgConnection,
    ) -> Result<Vec<Self>, WebCodeGenError> {
        use crate::schema::check_constraints;
        check_constraints::table.load::<CheckConstraint>(conn).map_err(WebCodeGenError::from)
    }

    /// Returns all the columns associated to this check constraint
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If their is an error while querying the database.
    pub fn columns(&self, conn: &mut PgConnection) -> Result<Vec<Column>, WebCodeGenError> {
        use crate::schema::{columns, constraint_column_usage};
        Ok(columns::table
            .inner_join(constraint_column_usage::table.on(
                constraint_column_usage::constraint_name.eq(&self.constraint_name).and(
                    constraint_column_usage::constraint_catalog.eq(&self.constraint_catalog).and(
                        constraint_column_usage::constraint_schema.eq(&self.constraint_schema),
                    ),
                ),
            ))
            .filter(
                constraint_column_usage::column_name.eq(&columns::column_name).and(
                    constraint_column_usage::table_catalog.eq(&columns::table_catalog).and(
                        constraint_column_usage::table_schema
                            .eq(&columns::table_schema)
                            .and(constraint_column_usage::table_name.eq(&columns::table_name)),
                    ),
                ),
            )
            .select(Column::as_select())
            .load(conn)?)
    }

    /// Returns whether the check constraint is associated to a single column.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If their is an error while querying the database.
    pub fn is_single_column_constraint(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        self.columns(conn).map(|columns| columns.len() == 1)
    }

    /// Returns whether the check is associated to multiple columns.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If their is an error while querying the database.
    pub fn is_multi_column_constraint(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        self.columns(conn).map(|columns| columns.len() > 1)
    }

    /// Load all the check constraints from the database
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Vec` of `CheckConstraint` if the operation was
    /// successful, or a `WebCodeGenError` if an error occurred
    ///
    /// # Errors
    ///
    /// If an error occurs while loading the constraints from the database
    pub fn load_check_constraints(
        conn: &mut PgConnection,
        constraint_name: &str,
        constraint_schema: Option<&str>,
        constraint_catalog: &str,
    ) -> Result<Vec<Self>, WebCodeGenError> {
        use crate::schema::check_constraints;
        let constraint_schema = constraint_schema.unwrap_or("public");
        check_constraints::table
            .filter(check_constraints::constraint_name.eq(constraint_name))
            .filter(check_constraints::constraint_schema.eq(constraint_schema))
            .filter(check_constraints::constraint_catalog.eq(constraint_catalog))
            .load::<CheckConstraint>(conn)
            .map_err(WebCodeGenError::from)
    }
}
