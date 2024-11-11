use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;
use diesel::{
    ExpressionMethods, QueryDsl, Queryable, QueryableByName, 
    RunQueryDsl, Selectable, SelectableHelper, TextExpressionMethods,
};


/// Represents a row in the pg_indexes view
#[derive(Queryable, QueryableByName, Selectable, Debug)]
#[diesel(table_name = crate::schema::pg_indexes)]
pub struct Index {
    pub schemaname: String,
    pub tablename: String,
    pub indexname: String,
    pub tablespace: Option<String>,
    pub indexdef: String,
}

impl Index {
    pub fn load_all_unique(conn: &mut PgConnection,
        table_schema: Option<&str>,
    ) -> Result<Vec<Self>, DieselError> {
        use crate::schema::pg_indexes;
        pg_indexes::dsl::pg_indexes
        .filter(pg_indexes::dsl::schemaname.eq(table_schema.unwrap_or("public")))
        .filter(pg_indexes::dsl::indexdef.like("%UNIQUE%"))
        .load::<Self>(conn)
    }

    pub fn load_all_gin(conn: &mut PgConnection,
        table_schema: Option<&str>,
    ) -> Result<Vec<Self>, DieselError> {
        use crate::schema::pg_indexes;
        pg_indexes::dsl::pg_indexes
        .filter(pg_indexes::dsl::schemaname.eq(table_schema.unwrap_or("public")))
        .filter(pg_indexes::dsl::indexdef.like("%USING gin%"))
        .load::<Self>(conn)
    }

    pub fn load_all_gist(conn: &mut PgConnection,
        table_schema: Option<&str>,
    ) -> Result<Vec<Self>, DieselError> {
        use crate::schema::pg_indexes;
        pg_indexes::dsl::pg_indexes
        .filter(pg_indexes::dsl::schemaname.eq(table_schema.unwrap_or("public")))
        .filter(pg_indexes::dsl::indexdef.like("%USING gist%"))
        .load::<Self>(conn)
    }

    pub fn is_unique(&self) -> bool {
        self.indexdef.contains("UNIQUE")
    }

    pub fn is_gin(&self) -> bool {
        self.indexdef.contains("USING gin")
    }

    pub fn is_gist(&self) -> bool {
        self.indexdef.contains("USING gist")
    }
}