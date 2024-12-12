use diesel::pg::PgConnection;
use diesel::{ExpressionMethods, QueryDsl, Queryable, QueryableByName, RunQueryDsl, Selectable};

#[derive(Queryable, QueryableByName, Selectable, Debug)]
#[diesel(table_name = crate::schema::key_column_usage)]
pub struct KeyColumnUsage {
    pub constraint_catalog: String,
    pub constraint_schema: String,
    pub constraint_name: String,
    pub table_catalog: String,
    pub table_schema: String,
    pub table_name: String,
    pub column_name: String,
    pub ordinal_position: i32,
    pub position_in_unique_constraint: Option<i32>,
}

impl KeyColumnUsage {
    pub fn load_all_key_column_usages(conn: &mut PgConnection) -> Vec<Self> {
        use crate::schema::key_column_usage;
        key_column_usage::table
            .load::<KeyColumnUsage>(conn)
            .expect("Error loading key column usages")
    }

    pub fn load_key_column_usages(
        conn: &mut PgConnection,
        table_name: &str,
        table_schema: Option<&str>,
        table_catalog: &str,
    ) -> Vec<Self> {
        use crate::schema::key_column_usage;
        let table_schema = table_schema.unwrap_or("public");
        key_column_usage::table
            .filter(key_column_usage::table_name.eq(table_name))
            .filter(key_column_usage::table_schema.eq(table_schema))
            .filter(key_column_usage::table_catalog.eq(table_catalog))
            .load::<KeyColumnUsage>(conn)
            .expect("Error loading key column usages")
    }
}
