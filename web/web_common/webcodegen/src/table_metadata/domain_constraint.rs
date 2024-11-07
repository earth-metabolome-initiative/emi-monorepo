use diesel::pg::PgConnection;
use diesel::{ExpressionMethods, QueryDsl, Queryable, QueryableByName, RunQueryDsl};

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
    pub fn load_all_domain_constraints(conn: &mut PgConnection) -> Vec<Self> {
        use crate::schema::domain_constraints::dsl::*;
        domain_constraints
            .load::<DomainConstraint>(conn)
            .expect("Error loading domain constraints")
    }

    pub fn load_domain_constraints(
        conn: &mut PgConnection,
        constraint_name: &str,
        constraint_schema: Option<&str>,
        constraint_catalog: &str,
    ) -> Vec<Self> {
        use crate::schema::domain_constraints;
        let constraint_schema = constraint_schema.unwrap_or("public");
        domain_constraints::dsl::domain_constraints
            .filter(domain_constraints::dsl::constraint_name.eq(constraint_name))
            .filter(domain_constraints::dsl::constraint_schema.eq(constraint_schema))
            .filter(domain_constraints::dsl::constraint_catalog.eq(constraint_catalog))
            .load::<DomainConstraint>(conn)
            .expect("Error loading domain constraints")
    }
}
