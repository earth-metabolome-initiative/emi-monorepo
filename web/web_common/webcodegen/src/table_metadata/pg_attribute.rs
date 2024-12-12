//! Submodule providing the struct `PgAttribute` and associated methods.
use crate::{errors::WebCodeGenError, PgType};
use diesel::{
    QueryDsl, RunQueryDsl, ExpressionMethods, PgConnection, Queryable, QueryableByName, Selectable,
};

#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq)]
#[diesel(table_name = crate::schema::pg_attribute)]
pub struct PgAttribute {
    pub attrelid: u32,
    pub attname: String,
    pub atttypid: u32,
    pub attlen: i16,
    pub attnum: i16,
    pub attcacheoff: i32,
    pub atttypmod: i32,
    pub attndims: i16,
    pub attbyval: bool,
    pub attalign: String,
    pub attstorage: String,
    #[cfg(feature = "postgres_17")]
    pub attcompression: String,
    pub attnotnull: bool,
    pub atthasdef: bool,
    pub atthasmissing: bool,
    pub attidentity: String,
    pub attgenerated: String,
    pub attisdropped: bool,
    pub attislocal: bool,
    pub attinhcount: i16,
    pub attcollation: u32,
    pub attstattarget: Option<i16>,
    pub attacl: Option<Vec<u32>>,
    pub attoptions: Option<Vec<String>>,
    pub attfdwoptions: Option<Vec<String>>,
}

impl PgAttribute {
    /// Returns the `PgType` associated to the `PgAttribute`.
    pub fn pg_type(&self, conn: &mut PgConnection) -> Result<PgType, WebCodeGenError> {
        use crate::schema::pg_type;
        pg_type::table
            .filter(pg_type::oid.eq(self.atttypid))
            .first(conn)
            .map_err(WebCodeGenError::from)
    }

    /// Returns whether the associated rust type supports `Copy`.
    pub fn supports_copy(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        self.pg_type(conn)?.supports_copy(conn)
    }

    /// Returns whether the associated rust type supports `Hash`.
    pub fn supports_hash(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        self.pg_type(conn)?.supports_hash(conn)
    }

    /// Returns whether the associated rust type supports `Eq`.
    pub fn supports_eq(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        self.pg_type(conn)?.supports_eq(conn)
    }
}
