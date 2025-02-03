use diesel::pg::PgConnection;
use diesel::{ExpressionMethods, QueryDsl, Queryable, QueryableByName, RunQueryDsl};
use crate::errors::WebCodeGenError;

/// Represents a domain constraint in the database.
///
/// A domain constraint is a rule that restricts the values that can be stored in a domain.
/// This struct maps to the `domain_constraints` table in the database.
#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = crate::schema::domain_constraints)]
pub struct DomainConstraint {
    /// The name of the database containing the constraint (always the current database).
    pub constraint_catalog: String,
    /// The name of the schema containing the constraint.
    pub constraint_schema: String,
    /// The name of the constraint.
    pub constraint_name: String,
    /// The name of the database containing the domain (always the current database).
    pub domain_catalog: Option<String>,
    /// The name of the schema containing the domain.
    pub domain_schema: Option<String>,
    /// The name of the domain.
    pub domain_name: Option<String>,
    /// Indicates if the constraint is deferrable (YES or NO).
    pub is_deferrable: String,
    /// Indicates if the constraint is initially deferred (YES or NO).
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
