//! Submodule providing a struct [`PgExtension`] representing the `pg_extension`
//! table.

use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, OptionalExtension, PgConnection, QueryDsl,
    Queryable, QueryableByName, RunQueryDsl, Selectable, SelectableHelper,
};

use super::PgProc;
use crate::errors::WebCodeGenError;

/// Represents the `pg_extension` system catalog table in `PostgreSQL`.
/// This table stores information about extensions.
#[derive(Queryable, QueryableByName, Selectable, Clone, Debug, PartialEq, Eq, Hash)]
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
    pub fn load_all(conn: &mut PgConnection) -> Result<Vec<Self>, WebCodeGenError> {
        use crate::schema::pg_extension;
        pg_extension::table.load(conn).map_err(WebCodeGenError::from)
    }

    /// Loads the [`PgExtension`] with the given name amd namespace from the
    /// database.
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
    pub fn load(
        name: &str,
        namespace: &str,
        conn: &mut PgConnection,
    ) -> Result<Option<Self>, WebCodeGenError> {
        use crate::schema::{pg_extension, pg_namespace};
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
    pub fn functions(&self, conn: &mut PgConnection) -> Result<Vec<PgProc>, diesel::result::Error> {
        use crate::schema::{pg_depend, pg_proc};
        pg_depend::table
            .inner_join(pg_proc::table.on(pg_depend::objid.eq(pg_proc::oid)))
            .filter(pg_depend::refobjid.eq(self.oid))
            .select(PgProc::as_select())
            .load(conn)
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
    pub fn types(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<crate::PgType>, diesel::result::Error> {
        use crate::schema::{pg_depend, pg_type};
        pg_depend::table
            .inner_join(pg_type::table.on(pg_depend::objid.eq(pg_type::oid)))
            .filter(pg_depend::refobjid.eq(self.oid))
            .select(crate::PgType::as_select())
            .load(conn)
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
    pub fn enums(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<crate::PgEnum>, diesel::result::Error> {
        use crate::schema::{pg_depend, pg_enum, pg_type};

        pg_enum::table
            .inner_join(pg_type::table.on(pg_enum::enumtypid.eq(pg_type::oid)))
            .inner_join(pg_depend::table.on(pg_depend::objid.eq(pg_type::oid)))
            .filter(pg_depend::refobjid.eq(self.oid))
            .select(crate::PgEnum::as_select())
            .load(conn)
    }

    #[must_use]
    /// Returns the [`Ident`](syn::Ident) for the [`PgExtension`].
    pub fn ident(&self) -> syn::Ident {
        syn::Ident::new(&self.extname, proc_macro2::Span::call_site())
    }
}
