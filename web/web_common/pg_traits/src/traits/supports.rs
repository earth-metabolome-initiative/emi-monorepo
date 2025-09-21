//! Submodule providing the [`Supports`] trait, which indicates whether
//! the Rust type associated with a database object supports the requested trait
//! which is identified with a marker struct.

/// Constant listing types supporting `Copy`.
pub(crate) const COPY_TYPES: [&str; 10] = [
    "i16",
    "i32",
    "i64",
    "f32",
    "f64",
    "bool",
    "::rosetta_uuid::Uuid",
    "::rosetta_timestamp::TimestampUTC",
    "::iso_codes::CountryCode",
    "::cas_codes::CAS",
];

/// Constant listing types supporting `Eq`.
pub(crate) const EQ_TYPES: [&str; 12] = [
    "i16",
    "i32",
    "i64",
    "bool",
    "String",
    "::chrono::NaiveDateTime",
    "::rosetta_uuid::Uuid",
    "::rosetta_timestamp::TimestampUTC",
    "::iso_codes::CountryCode",
    "::cas_codes::CAS",
    "::media_types::MediaType",
    "::molecular_formulas::MolecularFormula",
];

/// Constant listing types supporting `PartialOrd`.
pub(crate) const PARTIAL_ORD_TYPES: [&str; 2] = ["f32", "f64"];

/// Constant listing types supporting `Ord`.
pub(crate) const ORD_TYPES: [&str; 12] = [
    "i16",
    "i32",
    "i64",
    "bool",
    "String",
    "::chrono::NaiveDateTime",
    "::rosetta_uuid::Uuid",
    "::rosetta_timestamp::TimestampUTC",
    "::iso_codes::CountryCode",
    "::cas_codes::CAS",
    "::media_types::MediaType",
    "::molecular_formulas::MolecularFormula",
];

/// Constant listing types supporting `Hash`.
pub(crate) const HASH_TYPES: [&str; 12] = [
    "i16",
    "i32",
    "i64",
    "bool",
    "String",
    "::chrono::NaiveDateTime",
    "::rosetta_uuid::Uuid",
    "::rosetta_timestamp::TimestampUTC",
    "::iso_codes::CountryCode",
    "::media_types::MediaType",
    "::cas_codes::CAS",
    "::molecular_formulas::MolecularFormula",
];

/// Trait indicating that the Rust type associated with a database object
/// supports the trait identified by the marker struct.
pub trait Supports<TraitMarker> {
    /// The error type returned by the method.
    type Error;

    /// Returns whether the Rust type associated with the object supports the trait.
    fn supports(&self, conn: &mut diesel::PgConnection) -> Result<bool, Self::Error>;
}

/// Marker struct indicating the `Copy` trait.
pub struct Copy;

/// Marker struct indicating the `Hash` trait.
pub struct Hash;


/// Returns whether the associated rust type supports `Copy`.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing a boolean indicating whether the associated rust
    /// type supports `Copy`, or an error if the type is not supported.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn supports_copy(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        if self.is_composite() {
            let mut supports_copy = true;
            for attribute in self.attributes(conn)? {
                supports_copy &= attribute.supports_copy(conn)?;
            }
            Ok(supports_copy)
        } else if self.is_user_defined(conn)? {
            self.base_type(conn)?
                .ok_or(WebCodeGenError::MissingBaseType(Box::new(self.clone())))?
                .supports_copy(conn)
        } else {
            Ok(COPY_TYPES.contains(&rust_type_str(&self.typname, conn)?))
        }
    }

    /// Returns whether the associated rust type supports `Hash`.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing a boolean indicating whether the associated rust
    /// type supports `Hash`, or an error if the type is not supported.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn supports_hash(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        if self.is_user_defined(conn)? || self.is_composite() {
            let mut supports_hash = true;
            for attribute in self.attributes(conn)? {
                supports_hash &= attribute.supports_hash(conn)?;
            }
            Ok(supports_hash)
        } else {
            Ok(HASH_TYPES.contains(&rust_type_str(&self.typname, conn)?))
        }
    }

    /// Returns whether the associated rust type supports `Eq`.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing a boolean indicating whether the associated rust
    /// type supports `Eq`, or an error if the type is not supported.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn supports_eq(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        if self.is_user_defined(conn)? || self.is_composite() {
            let mut supports_eq = true;
            for attribute in self.attributes(conn)? {
                supports_eq &= attribute.supports_eq(conn)?;
            }
            Ok(supports_eq)
        } else {
            Ok(EQ_TYPES.contains(&rust_type_str(&self.typname, conn)?))
        }
    }

    /// Returns whether the associated rust type supports `PartialOrd`.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing a boolean indicating whether the associated rust
    /// type supports `PartialOrd`, or an error if the type is not supported.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn supports_partial_ord(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        if self.is_user_defined(conn)? || self.is_composite() {
            let mut supports_partial_ord = true;
            for attribute in self.attributes(conn)? {
                supports_partial_ord &= attribute.supports_partial_ord(conn)?;
            }
            Ok(supports_partial_ord)
        } else {
            Ok(PARTIAL_ORD_TYPES.contains(&rust_type_str(&self.typname, conn)?))
        }
    }

    /// Returns whether the associated rust type supports `Ord`.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Postgres connection.
    ///
    /// # Returns
    ///
    /// A Result containing a boolean indicating whether the associated rust
    /// type supports `Ord`, or an error if the type is not supported.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection fails.
    pub fn supports_ord(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        if self.is_user_defined(conn)? || self.is_composite() {
            let mut supports_ord = true;
            for attribute in self.attributes(conn)? {
                supports_ord &= attribute.supports_ord(conn)?;
            }
            Ok(supports_ord)
        } else {
            Ok(ORD_TYPES.contains(&rust_type_str(&self.typname, conn)?))
        }
    }