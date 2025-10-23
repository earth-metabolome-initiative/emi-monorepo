//! Submodule providing a struct [`PgExtension`] representing the `pg_extension`
//! table.

use diesel::{
    ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, Queryable, QueryableByName, RunQueryDsl,
    Selectable, SelectableHelper,
};

use super::PgProc;
use crate::models::{PgEnum, PgType};

mod cached_queries;

/// Represents the `pg_extension` system catalog table in `PostgreSQL`.
/// This table stores information about extensions.
#[derive(Queryable, QueryableByName, Selectable, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_extension::pg_extension)]
pub struct PgExtension {
    /// The OID of the extension.
    pub oid: u32,
    /// The name of the extension.
    pub extname: String,
    /// The OID of the owner of the extension.
    pub extowner: u32,
    /// The OID of the namespace that contains this extension.
    pub extnamespace: u32,
    /// Whether the extension is relocatable.
    pub extrelocatable: bool,
    /// The version of the extension.
    pub extversion: String,
    /// The schema of the extension.
    pub extconfig: Option<Vec<u32>>,
    /// The configuration settings for the extension.
    pub extcondition: Option<Vec<String>>,
}

impl AsRef<PgExtension> for PgExtension {
    fn as_ref(&self) -> &PgExtension {
        self
    }
}

impl PgExtension {
    /// Loads all of the [`PgExtension`]s from the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn load_all(conn: &mut PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::pg_catalog::pg_extension::pg_extension;
        pg_extension::table.load(conn)
    }

    /// Loads the [`PgExtension`] with the given name amd namespace from the
    /// database.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the extension
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn load(name: &str, conn: &mut PgConnection) -> Result<Self, diesel::result::Error> {
        use crate::schema::pg_catalog::{pg_extension::pg_extension, pg_namespace::pg_namespace};
        pg_extension::table
            .inner_join(pg_namespace::table.on(pg_extension::extnamespace.eq(pg_namespace::oid)))
            .filter(pg_extension::extname.eq(name))
            .select(PgExtension::as_select())
            .first(conn)
    }

    /// Returns all [`PgProc`] functions associated with this extension.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn functions(&self, conn: &mut PgConnection) -> Result<Vec<PgProc>, diesel::result::Error> {
        cached_queries::functions(self, conn)
    }

    /// Returns all [`PgType`](crate::PgType) types associated with this
    /// extension.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn types(&self, conn: &mut PgConnection) -> Result<Vec<PgType>, diesel::result::Error> {
        cached_queries::types(self, conn)
    }

    /// Returns all [`PgEnum`](crate::PgEnum) enums associated with this
    /// extension.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn enums(&self, conn: &mut PgConnection) -> Result<Vec<PgEnum>, diesel::result::Error> {
        cached_queries::enums(self, conn)
    }
}
