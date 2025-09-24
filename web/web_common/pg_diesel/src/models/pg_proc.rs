//! Submodule providing a struct [`PgProc`] representing the `pg_proc` table.

use diesel::{PgConnection, Queryable, QueryableByName, Selectable};

use super::PgType;
use crate::models::PgExtension;

mod cached_queries;

/// Represents the `pg_proc` system catalog table in `PostgreSQL`.
/// This table stores information about functions and procedures.
#[derive(Queryable, QueryableByName, Selectable, Debug, Clone, PartialEq)]
#[diesel(table_name = crate::schema::pg_proc)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::struct_excessive_bools)]
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
    /// The volatility category of the function ('i' for immutable, 's' for
    /// stable, 'v' for volatile).
    pub provolatile: String,
    /// The parallel safety category of the function ('u' for unsafe, 'r' for
    /// restricted, 's' for safe).
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
    /// Returns the `Vec` of [`PgType`] representing the types of the arguments
    /// of the function.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the provided connection is invalid.
    pub fn argument_types(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<PgType>, diesel::result::Error> {
        self.proargtypes.iter().map(|oid| PgType::from_oid(*oid, conn)).collect()
    }

    /// Returns the return [`PgType`] associated to the function.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the return type does not exist.
    pub fn return_type(&self, conn: &mut PgConnection) -> Result<PgType, diesel::result::Error> {
        PgType::from_oid(self.prorettype, conn)
    }

    /// Returns the [`PgExtension`] that contains this function, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a
    ///   [`PgConnection`](diesel::PgConnection).
    ///
    /// # Errors
    ///
    /// * If the function is not contained in an extension
    pub fn extension(&self, conn: &mut PgConnection) -> Result<PgExtension, diesel::result::Error> {
        cached_queries::extension(self, conn)
    }
}
