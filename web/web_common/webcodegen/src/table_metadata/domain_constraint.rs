use diesel::pg::PgConnection;
use diesel::{ExpressionMethods, QueryDsl, Queryable, QueryableByName, RunQueryDsl};
use crate::errors::WebCodeGenError;


#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = crate::schema::domain_constraints)]
pub struct DomainConstraint {
    pub constraint_catalog: String,
    pub constraint_schema: String,
    pub constraint_name: String,
    pub domain_catalog: Option<String>,
    pub domain_schema: Option<String>,
    pub domain_name: Option<String>,
    pub is_deferrable: String,
    pub initially_deferred: String,
}

impl DomainConstraint {
    /// Load all the domain constraints from the database
    /// 
    /// # Arguments
    /// 
    /// * `conn` - A mutable reference to a `PgConnection`
    /// 
    /// # Returns
    /// 
    /// A `Result` containing a `Vec` of `DomainConstraint` if the operation was successful, or a `WebCodeGenError` if an error occurred
    /// 
    /// # Errors
    /// 
    /// If an error occurs while loading the constraints from the database
    pub fn load_all_domain_constraints(conn: &mut PgConnection) -> Result<Vec<Self>, WebCodeGenError> {
        use crate::schema::domain_constraints;
        domain_constraints::table
            .load::<DomainConstraint>(conn)
            .map_err(WebCodeGenError::from)
    }

    /// Load all the domain constraints from the database
    /// 
    /// # Arguments
    /// 
    /// * `conn` - A mutable reference to a `PgConnection`
    /// * `constraint_name` - The name of the constraint to load
    /// * `constraint_schema` - An optional schema name to filter the constraints by
    /// * `constraint_catalog` - The name of the catalog to filter the constraints by
    /// 
    /// # Returns
    /// 
    /// A `Result` containing a `Vec` of `DomainConstraint` if the operation was successful, or a `WebCodeGenError` if an error occurred
    /// 
    /// # Errors
    /// 
    /// If an error occurs while loading the constraints from the database
    pub fn load_domain_constraints(
        conn: &mut PgConnection,
        constraint_name: &str,
        constraint_schema: Option<&str>,
        constraint_catalog: &str,
    ) -> Result<Vec<Self>, WebCodeGenError> {
        use crate::schema::domain_constraints;
        let constraint_schema = constraint_schema.unwrap_or("public");
        domain_constraints::table
            .filter(domain_constraints::constraint_name.eq(constraint_name))
            .filter(domain_constraints::constraint_schema.eq(constraint_schema))
            .filter(domain_constraints::constraint_catalog.eq(constraint_catalog))
            .load::<DomainConstraint>(conn)
            .map_err(WebCodeGenError::from)
    }
}
