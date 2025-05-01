use diesel::{
    ExpressionMethods, QueryDsl, Queryable, QueryableByName,
};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::errors::WebCodeGenError;

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = crate::schema::referential_constraints)]
/// A struct representing a referential constraint
pub struct ReferentialConstraint {
    /// The name of the constraint catalog
    pub constraint_catalog: String,
    /// The name of the constraint schema
    pub constraint_schema: String,
    /// The name of the constraint
    pub constraint_name: String,
    /// The name of the table catalog the constraint is associated with
    pub unique_constraint_catalog: Option<String>,
    /// The name of the table schema the constraint is associated with
    pub unique_constraint_schema: Option<String>,
    /// The name of the table the constraint is associated with
    pub unique_constraint_name: Option<String>,
    /// Match options
    pub match_option: String,
    /// Update rule
    pub update_rule: String,
    /// Delete rule
    pub delete_rule: String,
}

impl ReferentialConstraint {
    /// Load all the referential constraints from the database
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Vec` of `ReferentialConstraint` if the
    /// operation was successful, or a `WebCodeGenError` if an error occurred
    ///
    /// # Errors
    ///
    /// If an error occurs while loading the constraints from the database
    pub async fn load_all_referential_constraints(
        conn: &mut AsyncPgConnection,
    ) -> Result<Vec<Self>, WebCodeGenError> {
        use crate::schema::referential_constraints;
        referential_constraints::table
            .load::<ReferentialConstraint>(conn).await
            .map_err(WebCodeGenError::from)
    }

    /// Load all the referential constraints from the database
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    /// * `constraint_name` - The name of the constraint to load
    /// * `constraint_schema` - An optional schema name to filter the
    ///   constraints by
    /// * `constraint_catalog` - The name of the catalog to filter the
    ///   constraints by
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Vec` of `ReferentialConstraint` if the
    /// operation was successful, or a `WebCodeGenError` if an error occurred
    ///
    /// # Errors
    ///
    /// If an error occurs while loading the constraints from the database
    pub async fn load_referential_constraints(
        conn: &mut AsyncPgConnection,
        constraint_name: &str,
        constraint_schema: Option<&str>,
        constraint_catalog: &str,
    ) -> Result<Vec<Self>, WebCodeGenError> {
        use crate::schema::referential_constraints;
        let constraint_schema = constraint_schema.unwrap_or("public");
        referential_constraints::table
            .filter(referential_constraints::constraint_name.eq(constraint_name))
            .filter(referential_constraints::constraint_schema.eq(constraint_schema))
            .filter(referential_constraints::constraint_catalog.eq(constraint_catalog))
            .load::<ReferentialConstraint>(conn).await
            .map_err(WebCodeGenError::from)
    }
}
