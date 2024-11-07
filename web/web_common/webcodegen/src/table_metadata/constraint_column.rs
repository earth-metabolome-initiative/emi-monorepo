use diesel::pg::PgConnection;
use diesel::{ExpressionMethods, QueryDsl, Queryable, QueryableByName, RunQueryDsl};

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
    pub fn load_all_constraint_column_usages(conn: &mut PgConnection) -> Vec<Self> {
        use crate::schema::constraint_column_usage::dsl::*;
        constraint_column_usage
            .load::<ConstraintColumnUsage>(conn)
            .expect("Error loading constraint column usages")
    }

    pub fn load_constraint_column_usages(
        conn: &mut PgConnection,
        constraint_name: &str,
        constraint_schema: Option<&str>,
        constraint_catalog: &str,
    ) -> Vec<Self> {
        use crate::schema::constraint_column_usage;
        let constraint_schema = constraint_schema.unwrap_or("public");
        constraint_column_usage::dsl::constraint_column_usage
            .filter(constraint_column_usage::dsl::constraint_name.eq(constraint_name))
            .filter(constraint_column_usage::dsl::constraint_schema.eq(constraint_schema))
            .filter(constraint_column_usage::dsl::constraint_catalog.eq(constraint_catalog))
            .load::<ConstraintColumnUsage>(conn)
            .expect("Error loading constraint column usages")
    }
}