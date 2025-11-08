use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, Queryable,
    QueryableByName, Selectable, SelectableHelper,
};
use sql_traits::{structs::metadata::CheckMetadata, traits::FunctionLike};

use crate::{
    model_metadata::TableMetadata,
    models::{Column, PgConstraint, PgOperator, PgProc, Table, TableConstraint},
};

mod cached_queries;

#[derive(
    Queryable, QueryableByName, Debug, Clone, Selectable, Ord, PartialEq, Eq, Hash, PartialOrd,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::check_constraints::check_constraints)]
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

impl Display for CheckConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.constraint_catalog, self.constraint_schema, self.constraint_name)
    }
}

const POSTGIS_CONSTRAINTS: [&str; 1] = ["spatial_ref_sys_srid_check"];

impl CheckConstraint {
    #[must_use]
    /// Returns whether the current [`CheckConstraint`] is known to come
    /// from Postgis and therefore should most likely be ignored
    pub fn is_postgis_constraint(&self) -> bool {
        POSTGIS_CONSTRAINTS.contains(&self.constraint_name.as_str())
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
    pub fn functions(&self, conn: &mut PgConnection) -> Result<Vec<PgProc>, diesel::result::Error> {
        self.pg_constraint(conn)?.functions(conn)
    }

    /// Returns the vector of [`PgOperator`] operators that are used in the
    /// check clause
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn operators(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<PgOperator>, diesel::result::Error> {
        self.pg_constraint(conn)?.operators(conn)
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
    pub fn pg_constraint(
        &self,
        conn: &mut PgConnection,
    ) -> Result<PgConstraint, diesel::result::Error> {
        use diesel::RunQueryDsl;

        use crate::schema::pg_catalog::{pg_constraint::pg_constraint, pg_namespace::pg_namespace};
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
            .map_err(diesel::result::Error::from)
    }

    /// Returns the table constraint associated with this check constraint
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn table_constraint(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TableConstraint, diesel::result::Error> {
        cached_queries::table_constraint(self, conn)
    }

    /// Returns the table that this check constraint belongs to
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn table(&self, conn: &mut PgConnection) -> Result<Table, diesel::result::Error> {
        self.table_constraint(conn)?.table(conn)
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
    pub fn columns(&self, conn: &mut PgConnection) -> Result<Vec<Column>, diesel::result::Error> {
        cached_queries::columns(self, conn)
    }

    /// Returns the metadata for this check constraint
    ///
    /// # Arguments
    ///
    /// * `table` - The table this check constraint belongs to
    /// * `table_metadata` - The metadata of the table
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If there is an error while querying the database.
    ///
    /// # Panics
    ///
    /// * If the check clause cannot be parsed into an expression, which should
    ///  not happen if the database is consistent.
    pub fn metadata(
        &self,
        table: Rc<Table>,
        table_metadata: &TableMetadata,
        functions: &[Rc<PgProc>],
        conn: &mut PgConnection,
    ) -> Result<CheckMetadata<CheckConstraint>, diesel::result::Error> {
        use sqlparser::parser::Parser;
        let expression = Parser::new(&sqlparser::dialect::PostgreSqlDialect {})
            .try_with_sql(self.check_clause.as_str())
            .expect("Failed to parse unique constraint expression")
            .parse_expr()
            .expect("No expression found in parsed unique constraint");

        Ok(CheckMetadata::new(
            expression,
            table,
            self.columns(conn)?
                .into_iter()
                .filter_map(|col| {
                    table_metadata
                        .column_rcs()
                        .find(|table_col| table_col.column_name == col.column_name)
                        .cloned()
                })
                .collect(),
            self.functions(conn)?
                .into_iter()
                .filter_map(|func| {
                    functions
                        .iter()
                        .find(|table_func| table_func.name() == func.proname.as_str())
                        .cloned()
                })
                .collect(),
        ))
    }
}
