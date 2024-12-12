use diesel::pg::PgConnection;
use diesel::{ExpressionMethods, QueryDsl, Queryable, QueryableByName, RunQueryDsl, Selectable};

#[derive(Queryable, QueryableByName, Debug, Selectable)]
#[diesel(table_name = crate::schema::check_constraints)]
pub struct CheckConstraint {
    pub constraint_catalog: String,
    pub constraint_schema: String,
    pub constraint_name: String,
    pub check_clause: String,
}

impl CheckConstraint {
    pub fn load_all_check_constraints(conn: &mut PgConnection) -> Vec<Self> {
        use crate::schema::check_constraints;
        check_constraints::table
            .load::<CheckConstraint>(conn)
            .expect("Error loading check constraints")
    }

    pub fn load_check_constraints(
        conn: &mut PgConnection,
        constraint_name: &str,
        constraint_schema: Option<&str>,
        constraint_catalog: &str,
    ) -> Vec<Self> {
        use crate::schema::check_constraints;
        let constraint_schema = constraint_schema.unwrap_or("public");
        check_constraints::table
            .filter(check_constraints::constraint_name.eq(constraint_name))
            .filter(check_constraints::constraint_schema.eq(constraint_schema))
            .filter(check_constraints::constraint_catalog.eq(constraint_catalog))
            .load::<CheckConstraint>(conn)
            .expect("Error loading check constraints")
    }
}
