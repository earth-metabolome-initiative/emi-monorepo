use diesel::pg::PgConnection;
use diesel::{ExpressionMethods, QueryDsl, Queryable, QueryableByName, RunQueryDsl};

use crate::errors::WebCodeGenError;

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = crate::schema::constraint_column_usage)]
pub struct ConstraintColumnUsage {
    pub constraint_catalog: String,
    pub constraint_schema: String,
    pub constraint_name: String,
    pub table_catalog: String,
    pub table_schema: String,
    pub table_name: String,
    pub column_name: String,
}

impl ConstraintColumnUsage {

    /// Load all the constraint column usages from the database
    /// 
    /// # Arguments
    /// 
    /// * `conn` - A mutable reference to a `PgConnection`
    /// 
    /// # Returns
    /// 
    /// A `Result` containing a `Vec` of `ConstraintColumnUsage` if the operation was successful, or a `WebCodeGenError` if an error occurred
    /// 
    /// # Errors
    /// 
    /// If an error occurs while loading the column usages from the database
    pub fn load_all_constraint_column_usages(conn: &mut PgConnection) -> Result<Vec<Self>, WebCodeGenError> {
        use crate::schema::constraint_column_usage;
        constraint_column_usage::table
            .load::<ConstraintColumnUsage>(conn)
            .map_err(WebCodeGenError::from)
    }

    /// Load all the constraint column usages from the database
    /// 
    /// # Arguments
    /// 
    /// * `conn` - A mutable reference to a `PgConnection`
    /// * `constraint_name` - The name of the constraint to load the column usages for
    /// * `constraint_schema` - An optional schema name to filter the column usages by
    /// * `constraint_catalog` - The name of the catalog to filter the column usages by
    /// 
    /// # Returns
    /// 
    /// A `Result` containing a `Vec` of `ConstraintColumnUsage` if the operation was successful, or a `WebCodeGenError` if an error occurred
    /// 
    /// # Errors
    /// 
    /// If an error occurs while loading the column usages from the database
    pub fn load_constraint_column_usages(
        conn: &mut PgConnection,
        constraint_name: &str,
        constraint_schema: Option<&str>,
        constraint_catalog: &str,
    ) -> Result<Vec<Self>, WebCodeGenError> {
        use crate::schema::constraint_column_usage;
        let constraint_schema = constraint_schema.unwrap_or("public");
        constraint_column_usage::table
            .filter(constraint_column_usage::constraint_name.eq(constraint_name))
            .filter(constraint_column_usage::constraint_schema.eq(constraint_schema))
            .filter(constraint_column_usage::constraint_catalog.eq(constraint_catalog))
            .load::<ConstraintColumnUsage>(conn)
            .map_err(WebCodeGenError::from)
    }
}
