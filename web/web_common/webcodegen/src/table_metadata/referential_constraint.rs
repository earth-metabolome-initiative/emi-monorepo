use diesel::pg::PgConnection;
use diesel::{ExpressionMethods, QueryDsl, Queryable, QueryableByName, RunQueryDsl};

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = crate::schema::referential_constraints)]
pub struct ReferentialConstraint {
    pub constraint_catalog: String,
    pub constraint_schema: String,
    pub constraint_name: String,
    pub unique_constraint_catalog: Option<String>,
    pub unique_constraint_schema: Option<String>,
    pub unique_constraint_name: Option<String>,
    pub match_option: String,
    pub update_rule: String,
    pub delete_rule: String,
}

impl ReferentialConstraint {
    pub fn load_all_referential_constraints(conn: &mut PgConnection) -> Vec<Self> {
        use crate::schema::referential_constraints::dsl::*;
        referential_constraints
            .load::<ReferentialConstraint>(conn)
            .expect("Error loading referential constraints")
    }

    pub fn load_referential_constraints(
        conn: &mut PgConnection,
        constraint_name: &str,
        constraint_schema: Option<&str>,
        constraint_catalog: &str,
    ) -> Vec<Self> {
        use crate::schema::referential_constraints;
        let constraint_schema = constraint_schema.unwrap_or("public");
        referential_constraints::dsl::referential_constraints
            .filter(referential_constraints::dsl::constraint_name.eq(constraint_name))
            .filter(referential_constraints::dsl::constraint_schema.eq(constraint_schema))
            .filter(referential_constraints::dsl::constraint_catalog.eq(constraint_catalog))
            .load::<ReferentialConstraint>(conn)
            .expect("Error loading referential constraints")
    }
}
