use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;
use diesel::{
    ExpressionMethods, QueryDsl, Queryable, QueryableByName, RunQueryDsl, Selectable,
    TextExpressionMethods,
};

/// Represents a row in the `pg_indexes` view
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq)]
#[diesel(table_name = crate::schema::pg_indexes)]
/// Represents a PostgreSQL index in the database.
pub struct Index {
    /// The name of the schema containing the index.
    pub schemaname: String,
    /// The name of the table associated with the index.
    pub tablename: String,
    /// The name of the index.
    pub indexname: String,
    /// The name of the tablespace containing the index.
    pub tablespace: Option<String>,
    /// The definition of the index.
    pub indexdef: String,
}

impl Index {
    /// Load all the indexes from the database
    /// 
    /// # Arguments
    /// 
    /// * `conn` - A mutable reference to a `PgConnection`
    /// * `table_schema` - An optional schema name to filter the indexes by
    /// 
    /// # Returns
    /// 
    /// A `Result` containing a `Vec` of `Index` if the operation was successful, or a `DieselError` if an error occurred
    /// 
    /// # Errors
    /// 
    /// If an error occurs while loading the indexes from the database
    pub fn load_all_unique(
        conn: &mut PgConnection,
        table_schema: Option<&str>,
    ) -> Result<Vec<Self>, DieselError> {
        use crate::schema::pg_indexes;
        pg_indexes::table
            .filter(pg_indexes::schemaname.eq(table_schema.unwrap_or("public")))
            .filter(pg_indexes::indexdef.like("%UNIQUE%"))
            .load::<Self>(conn)
    }

    /// Load all the GIN indexes from the database
    /// 
    /// # Arguments
    /// 
    /// * `conn` - A mutable reference to a `PgConnection`
    /// * `table_schema` - An optional schema name to filter the indexes by
    /// 
    /// # Returns
    /// 
    /// A `Result` containing a `Vec` of `Index` if the operation was successful, or a `DieselError` if an error occurred
    /// 
    /// # Errors
    /// 
    /// If an error occurs while loading the indexes from the database
    pub fn load_all_gin(
        conn: &mut PgConnection,
        table_schema: Option<&str>,
    ) -> Result<Vec<Self>, DieselError> {
        use crate::schema::pg_indexes;
        pg_indexes::table
            .filter(pg_indexes::schemaname.eq(table_schema.unwrap_or("public")))
            .filter(pg_indexes::indexdef.like("%USING gin%"))
            .load::<Self>(conn)
    }

    /// Load all the GIST indexes from the database
    /// 
    /// # Arguments
    /// 
    /// * `conn` - A mutable reference to a `PgConnection`
    /// * `table_schema` - An optional schema name to filter the indexes by
    /// 
    /// # Returns
    /// 
    /// A `Result` containing a `Vec` of `Index` if the operation was successful, or a `DieselError` if an error occurred
    /// 
    /// # Errors
    /// 
    /// If an error occurs while loading the indexes from the database
    pub fn load_all_gist(
        conn: &mut PgConnection,
        table_schema: Option<&str>,
    ) -> Result<Vec<Self>, DieselError> {
        use crate::schema::pg_indexes;
        pg_indexes::table
            .filter(pg_indexes::schemaname.eq(table_schema.unwrap_or("public")))
            .filter(pg_indexes::indexdef.like("%USING gist%"))
            .load::<Self>(conn)
    }

    #[must_use]
    /// Returns whether the index is unique
    pub fn is_unique(&self) -> bool {
        self.indexdef.contains("UNIQUE")
    }

    #[must_use]
    /// Returns whether the index is a GIN index
    pub fn is_gin(&self) -> bool {
        self.indexdef.contains("USING gin")
    }

    #[must_use]
    /// Returns whether the index is a GIST index
    pub fn is_gist(&self) -> bool {
        self.indexdef.contains("USING gist")
    }
}
