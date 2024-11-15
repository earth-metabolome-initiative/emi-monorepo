use diesel::pg::PgConnection;
use diesel::{ExpressionMethods, QueryDsl, Queryable, QueryableByName, RunQueryDsl};

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = crate::schema::constraint_table_usage)]
pub struct ConstraintTableUsage {
    pub constraint_catalog: String,
    pub constraint_schema: String,
    pub constraint_name: String,
    pub table_catalog: String,
    pub table_schema: String,
    pub table_name: String,
}

impl ConstraintTableUsage {
    pub fn load_all(conn: &mut PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::constraint_table_usage::dsl::*;
        constraint_table_usage
            .load::<ConstraintTableUsage>(conn)
        }
}
