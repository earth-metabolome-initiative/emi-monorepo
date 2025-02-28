//! Submodule providing a struct [`PgExtension`] representing the `pg_extension` table.

use diesel::BoolExpressionMethods;
use diesel::ExpressionMethods;
use diesel::JoinOnDsl;
use diesel::OptionalExtension;
use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::SelectableHelper;
use diesel::{Queryable, QueryableByName, Selectable};

use crate::errors::WebCodeGenError;

use super::PgProc;

/// Represents the `pg_extension` system catalog table in `PostgreSQL`.
/// This table stores information about extensions.
#[derive(Queryable, QueryableByName, Selectable, Clone, Debug, PartialEq)]
#[diesel(table_name = crate::schema::pg_extension)]
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
    ///
    pub fn load_all(conn: &mut PgConnection) -> Result<Vec<Self>, WebCodeGenError> {
        use crate::schema::pg_extension;
        pg_extension::table.load(conn).map_err(WebCodeGenError::from)
    }

    /// Loads the [`PgExtension`] with the given name amd namespace from the database.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the extension
    /// * `namespace` - The namespace of the extension
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    ///
    pub fn load(
        name: &str,
        namespace: &str,
        conn: &mut PgConnection,
    ) -> Result<Option<Self>, WebCodeGenError> {
        use crate::schema::pg_extension;
        use crate::schema::pg_namespace;
        pg_extension::table
            .inner_join(pg_namespace::table.on(pg_extension::extnamespace.eq(pg_namespace::oid)))
            .filter(pg_extension::extname.eq(name).and(pg_namespace::nspname.eq(namespace)))
            .select(PgExtension::as_select())
            .first(conn)
            .optional()
            .map_err(WebCodeGenError::from)
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
    ///
    pub fn functions(&self, conn: &mut PgConnection) -> Result<Vec<PgProc>, WebCodeGenError> {
        use crate::schema::{pg_depend, pg_proc};
        pg_depend::table
            .inner_join(pg_proc::table.on(pg_depend::objid.eq(pg_proc::oid)))
            .filter(pg_depend::refobjid.eq(self.oid))
            .select(PgProc::as_select())
            .load(conn)
            .map_err(WebCodeGenError::from)
    }
}
