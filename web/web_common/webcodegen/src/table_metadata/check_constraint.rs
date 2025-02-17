use diesel::{
    pg::PgConnection, ExpressionMethods, QueryDsl, Queryable, QueryableByName, RunQueryDsl,
    Selectable,
};

use crate::errors::WebCodeGenError;

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
