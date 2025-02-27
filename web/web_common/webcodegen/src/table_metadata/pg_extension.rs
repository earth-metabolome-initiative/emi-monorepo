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

/// Represents the `pg_extension` system catalog table in `PostgreSQL`.
/// This table stores information about extensions.
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::pg_extension)]
pub struct PgExtension {
    /// The OID of the extension.
    pub oid: u32,
    /// The name of the extension.
    pub extname: String,
    /// The OID of the namespace that contains this extension.
    pub extnamespace: u32,
    /// The OID of the owner of the extension.
    pub extowner: u32,
    /// The version of the extension.
    pub extversion: String,
    /// The schema of the extension.
    pub extconfig: Option<Vec<u32>>,
    /// The configuration settings for the extension.
    pub extcondition: Option<Vec<String>>,
}

impl PgExtension {
    /// Loads the extension with the given name amd namespace from the database.
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
}
