//! Submodule providing the [`PgType`] struct and associated methods.

use diesel::{PgConnection, Queryable, QueryableByName, Selectable};

mod cached_queries;

use super::{PgAttribute, PgEnum, PgExtension};

/// Represents a `PostgreSQL` type.
///
/// This struct contains metadata about a `PostgreSQL` type, including its name,
/// OID (Object Identifier), namespace, and other properties.
#[derive(
    Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_type)]
#[allow(clippy::struct_excessive_bools)]
pub struct PgType {
    /// The OID (Object Identifier) of the type.
    pub oid: u32,
    /// The name of the type.
    pub typname: String,
    /// The namespace (schema) of the type.
    pub typnamespace: u32,
    /// The owner of the type.
    pub typowner: u32,
    /// The size of the type in bytes.
    pub typlen: i16,
    /// Indicates if the type is passed by value.
    pub typbyval: bool,
    /// The type of the type.
    pub typtype: String,
    /// The category of the type.
    pub typcategory: String,
    /// Indicates if the type is preferred within its category.
    pub typispreferred: bool,
    /// Indicates if the type is defined.
    pub typisdefined: bool,
    /// The delimiter for array elements of this type.
    pub typdelim: String,
    /// The relation ID for a composite type.
    pub typrelid: u32,
    /// The element type of an array type.
    pub typelem: u32,
    /// The array type of a base type.
    pub typarray: u32,
    /// The input function for the type.
    pub typinput: u32,
    /// The output function for the type.
    pub typoutput: u32,
    /// The receive function for the type.
    pub typreceive: u32,
    /// The send function for the type.
    pub typsend: u32,
    /// The modifier input function for the type.
    pub typmodin: u32,
    /// The modifier output function for the type.
    pub typmodout: u32,
    /// The analyze function for the type.
    pub typanalyze: u32,
    /// The alignment requirement of the type.
    pub typalign: String,
    /// The storage strategy for the type.
    pub typstorage: String,
    /// Indicates if the type is not nullable.
    pub typnotnull: bool,
    /// The base type of a domain type.
    pub typbasetype: u32,
    /// The type modifier.
    pub typtypmod: i32,
    /// The number of dimensions for an array type.
    pub typndims: i32,
    /// The collation for the type.
    pub typcollation: u32,
    /// The default binary representation of the type.
    pub typdefaultbin: Option<Vec<u8>>,
    /// The default text representation of the type.
    pub typdefault: Option<String>,
}

impl PgType {
    /// Returns the extension of the `PgType`, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// An option containing the `PgExtension` of the `PgType`,
    /// or None if the type is not from an extension.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn extension(&self, conn: &mut PgConnection) -> Result<PgExtension, crate::error::Error> {
        cached_queries::extension(self, conn)
    }

    /// Returns the internal custom types of the `PgType`, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing the internal custom types of the `PgType`, or an
    /// error if the type is not supported.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn internal_user_defined_types(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<PgType>, crate::error::Error> {
        let mut internal_user_defined_types = Vec::new();
        for attribute in self.attributes(conn)? {
            let pg_type = attribute.pg_type(conn)?;
            if pg_type.is_composite() || pg_type.is_enum() {
                internal_user_defined_types.extend(pg_type.internal_user_defined_types(conn)?);
                internal_user_defined_types.push(pg_type);
            }
        }

        Ok(internal_user_defined_types)
    }

    /// Returns the Type Base Type of the `PgType`.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing the Type Base Type of the `PgType`, or an error if
    /// the type is not supported.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn base_type(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<PgType>, crate::error::Error> {
        if self.typbasetype == 0 {
            Ok(None)
        } else {
            Ok(Some(Self::from_oid(self.typbasetype, conn)?))
        }
    }

    /// Returns the [`PgType`] from the given OID.
    ///
    /// # Arguments
    ///
    /// * `oid` - The OID of the type.
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn from_oid(oid: u32, conn: &mut PgConnection) -> Result<PgType, crate::error::Error> {
        cached_queries::from_oid(oid, conn)
    }

    /// Returns whether the Postgres type is a user-defined type.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing a boolean indicating whether the Postgres type is a
    /// user-defined type, or an error if the type is not supported.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn is_user_defined(&self, conn: &mut PgConnection) -> Result<bool, crate::error::Error> {
        Ok(&self.typcategory == "U" && self.base_type(conn)?.is_some())
    }

    #[must_use]
    /// Returns whether the Postgres type is a composite type.
    pub fn is_composite(&self) -> bool {
        &self.typcategory == "C"
    }

    #[must_use]
    /// Returns whether the Postgres type is an enum type.
    pub fn is_enum(&self) -> bool {
        &self.typcategory == "E"
    }

    /// Returns the attributes of the type, if it is a composite type.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing the attributes of the type if it is a composite
    /// type, or an error if it is not.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn attributes(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<PgAttribute>, crate::error::Error> {
        cached_queries::attributes(self, conn)
    }

    /// Returns the variants of the type, if it is an enum type.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing the variants of the type if it is an enum type, or
    /// an error if it is not.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn variants(&self, conn: &mut PgConnection) -> Result<Vec<PgEnum>, crate::error::Error> {
        cached_queries::variants(self, conn)
    }
}
