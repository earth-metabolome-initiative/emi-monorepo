//! Submodule providing a struct [`PgProc`] representing the `pg_proc` table.

use crate::PgExtension;
use diesel::ExpressionMethods;
use diesel::JoinOnDsl;
use diesel::SelectableHelper;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::{PgConnection, Queryable, QueryableByName, Selectable};

/// Represents the `pg_proc` system catalog table in `PostgreSQL`.
/// This table stores information about functions and procedures.
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::pg_proc)]
pub struct PgProc {
    /// The OID of the function.
    pub oid: u32,
    /// The name of the function.
    pub proname: String,
    /// The OID of the namespace that contains this function.
    pub pronamespace: u32,
    /// The OID of the owner of the function.
    pub proowner: u32,
    /// The OID of the language in which the function is implemented.
    pub prolang: u32,
    /// The estimated execution cost of the function.
    pub procost: f32,
    /// The estimated number of rows returned by the function.
    pub prorows: f32,
    /// The OID of the variadic argument type, or 0 if none.
    pub provariadic: u32,
    /// The OID of the support function, or 0 if none.
    pub prosupport: u32,
    /// The kind of function ('f' for normal, 'p' for procedure, etc.).
    pub prokind: String,
    /// True if the function is a security definer.
    pub prosecdef: bool,
    /// True if the function is leakproof.
    pub proleakproof: bool,
    /// True if the function is strict (null in, null out).
    pub proisstrict: bool,
    /// True if the function returns a set.
    pub proretset: bool,
    /// The volatility category of the function ('i' for immutable, 's' for stable, 'v' for volatile).
    pub provolatile: String,
    /// The parallel safety category of the function ('u' for unsafe, 'r' for restricted, 's' for safe).
    pub proparallel: String,
    /// The number of arguments the function takes.
    pub pronargs: i16,
    /// The number of arguments with default values.
    pub pronargdefaults: i16,
    /// The OID of the return type.
    pub prorettype: u32,
    /// An array of OIDs of the argument types.
    pub proargtypes: Vec<u32>,
    /// An array of OIDs of all argument types, including OUT parameters.
    pub proallargtypes: Option<Vec<u32>>,
    /// An array of modes of the arguments ('i' for IN, 'o' for OUT, etc.).
    pub proargmodes: Option<Vec<String>>,
    /// An array of names of the arguments.
    pub proargnames: Option<Vec<String>>,
    /// An array of default values for the arguments.
    pub proargdefaults: Option<Vec<String>>,
    /// The source code of the function.
    pub prosrc: String,
    /// The binary representation of the function.
    pub probin: Option<Vec<u8>>,
    /// The SQL body of the function, if any.
    pub prosqlbody: Option<String>,
    /// The configuration settings for the function.
    pub proconfig: Option<Vec<String>>,
}

impl PgProc {
    /// Returns the [`PgExtension`] that contains this function, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If the function is not contained in an extension
    ///
    pub fn extension(&self, conn: &mut PgConnection) -> Result<Option<PgExtension>, diesel::result::Error> {
        use crate::schema::{pg_depend, pg_extension};
        pg_depend::table
            // Join pg_depend to pg_extension using the extension's OID.
            .inner_join(pg_extension::table.on(pg_depend::objid.eq(pg_extension::oid)))
            // Filter rows where the function's OID is referenced.
            .filter(pg_depend::refobjid.eq(self.oid))
            // Only consider dependencies that mark extension membership.
            .filter(pg_depend::deptype.eq("e"))
            // Select all columns from pg_extension.
            .select(PgExtension::as_select())
            // Get the first matching extension, if one exists.
            .first::<PgExtension>(conn)
            .optional()
    }
}
