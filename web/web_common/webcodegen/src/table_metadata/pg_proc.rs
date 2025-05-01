//! Submodule providing a struct [`PgProc`] representing the `pg_proc` table.

use diesel::{
    ExpressionMethods, JoinOnDsl, OptionalExtension, QueryDsl, Queryable, QueryableByName,
    Selectable, SelectableHelper,
};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use syn::punctuated::Punctuated;

use super::PgType;
use crate::{PgExtension, errors::WebCodeGenError};

/// Represents the `pg_proc` system catalog table in `PostgreSQL`.
/// This table stores information about functions and procedures.
#[derive(Queryable, QueryableByName, Selectable, Debug, Clone, PartialEq)]
#[diesel(table_name = crate::schema::pg_proc)]
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
    /// Returns whether the current function may be return a `Result`.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the return type does not exist.
    pub async fn returns_result(
        &self,
        conn: &mut AsyncPgConnection,
    ) -> Result<bool, WebCodeGenError> {
        if self.proname.starts_with("must_be_") || self.proname.starts_with("must_not_be_") {
            Ok(self.return_type(conn).await?.is_boolean(conn).await?)
        } else {
            Ok(false)
        }
    }

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
    pub async fn argument_types(
        &self,
        conn: &mut AsyncPgConnection,
    ) -> Result<Vec<PgType>, diesel::result::Error> {
        let mut arg_types = Vec::new();
        for oid in &self.proargtypes {
            let arg_type = PgType::from_oid(*oid, conn).await?;
            arg_types.push(arg_type);
        }
        Ok(arg_types)
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
    pub async fn return_type(
        &self,
        conn: &mut AsyncPgConnection,
    ) -> Result<PgType, diesel::result::Error> {
        PgType::from_oid(self.prorettype, conn).await
    }

    /// Returns the [`PgProc`] associated to the given `name` and `namespace`.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function.
    /// * `namespace` - The name of the namespace that contains the function.
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the function does not exist.
    pub async fn load(
        name: &str,
        namespace: &str,
        conn: &mut AsyncPgConnection,
    ) -> Result<Option<PgProc>, diesel::result::Error> {
        use crate::schema::pg_proc;
        pg_proc::table
            .filter(pg_proc::proname.eq(name))
            .inner_join(
                crate::schema::pg_namespace::table
                    .on(pg_proc::pronamespace.eq(crate::schema::pg_namespace::oid)),
            )
            .filter(crate::schema::pg_namespace::nspname.eq(namespace))
            .select(PgProc::as_select())
            .first::<PgProc>(conn)
            .await
            .optional()
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
    pub async fn extension(
        &self,
        conn: &mut AsyncPgConnection,
    ) -> Result<Option<PgExtension>, diesel::result::Error> {
        use crate::schema::{pg_depend, pg_extension};
        pg_extension::table
            .inner_join(pg_depend::table.on(pg_extension::oid.eq(pg_depend::refobjid)))
            .filter(pg_depend::objid.eq(self.oid))
            .select(PgExtension::as_select())
            .first::<PgExtension>(conn)
            .await
            .optional()
    }

    /// Returns the path to the function.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a
    ///   [`PgConnection`](diesel::PgConnection).
    ///
    /// # Errors
    ///
    /// * If the database connection is invalid.
    /// * If the function is not contained in an extension.
    pub async fn path(
        &self,
        conn: &mut AsyncPgConnection,
    ) -> Result<syn::Path, diesel::result::Error> {
        let extension = self.extension(conn).await?;
        let extension = extension.map(|ext| ext.ident());
        let ident = syn::Ident::new(&self.proname, proc_macro2::Span::call_site());
        let segments: Vec<syn::PathSegment> = match extension {
            Some(extension) => vec![extension.into(), ident.into()],
            None => vec![ident.into()],
        };
        Ok(syn::Path { leading_colon: None, segments: Punctuated::from_iter(segments) })
    }
}
