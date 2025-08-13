//! Submodule defining traits regarding tables that can be extended with new
//! rows.
use std::{convert::Infallible, str::FromStr};

use common_traits::prelude::BuilderError;
use generic_backend_request_errors::GenericBackendRequestError;

/// A trait for types that can be inserted into the database.
pub trait Insertable {
    /// The associated builder type which can be constructed in the frontend or
    /// backend to create the insertable variant.
    type InsertableBuilder: Default;
    /// The associated insertable variant type which can be constructed in the
    /// frontend or backend to execute the insert operation.
    type InsertableVariant;

    #[must_use]
    /// Returns a new insertable variant builder.
    fn new() -> Self::InsertableBuilder {
        Default::default()
    }
}

/// A trait defining a builder which can be extended by a builder of the same
/// type. This is necessary when there exist for any of several reasons two
/// instances of the same builder type which at some point need to be merged
/// together.
pub trait ExtendableBuilder: Sized {
    /// The attributes of the builder.
    type Attributes;

    /// Extends the current builder with another builder of the same type.
    ///
    /// # Arguments
    ///
    /// * `other` - The other builder to extend the current builder with.
    ///
    /// # Errors
    ///
    /// * If the extension fails due to a validation error or a build error.
    fn extend_builder(self, other: Self) -> Result<Self, InsertError<Self::Attributes>>;
}

/// A trait for Row Builder types which allow for the setting of a primary key
/// in the insertable variant.
pub trait SetPrimaryKey {
    /// The associated primary key type.
    type PrimaryKey;

    /// Sets the primary key for the insertable variant.
    fn set_primary_key(self, primary_key: Self::PrimaryKey) -> Self;
}

/// Macro implementation for the `SetPrimaryKey` trait for all types that
/// are commonly used as primary keys, such as `i16`, `i32`, `i64` and `Uuid`.
macro_rules! impl_set_primary_key_for_primitive {
    ($($type:ty),*) => {
        $(
            impl SetPrimaryKey for $type {
                type PrimaryKey = Self;

                /// Sets the primary key for the insertable variant.
                fn set_primary_key(self, primary_key: Self::PrimaryKey) -> Self {
                    primary_key
                }
            }
        )*
    };
}

impl_set_primary_key_for_primitive!(i16, i32, i64, rosetta_uuid::Uuid);

/// Implementation of `SetPrimaryKey` for all `Option<T>` types where `T`
/// implements `SetPrimaryKey`. This allows for setting the primary key on an
/// optional insertable variant.
impl<T> SetPrimaryKey for Option<T>
where
    T: SetPrimaryKey<PrimaryKey = T>,
{
    type PrimaryKey = T;

    /// Sets the primary key for the insertable variant.
    fn set_primary_key(self, primary_key: Self::PrimaryKey) -> Self {
        match self {
            Some(inner) => Some(inner.set_primary_key(primary_key)),
            None => Some(primary_key),
        }
    }
}

/// A trait for types that can be constructed in the frontend or backend to
/// execute the insert operation.
pub trait InsertableVariant<C> {
    /// The associated row type which can be inserted into the database.
    type Row: Insertable<InsertableVariant = <Self as InsertableVariant<C>>::InsertableVariant>
        + diesel::associations::HasTable;
    /// The associated insertable variant type which can be constructed in the
    /// frontend or backend to execute the insert operation.
    type InsertableVariant: diesel::Insertable<<Self::Row as diesel::associations::HasTable>::Table>;
    /// The expected user ID type.
    type UserId;
    /// The error type returned by the insert operation.
    type Error: From<diesel::result::Error>;

    /// Inserts the row into the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID.
    /// * `conn` - The connection to the database.
    ///
    /// # Returns
    ///
    /// The inserted row, if the operation was successful.
    ///
    /// # Errors
    ///
    /// * If the row cannot be inserted.
    /// * If the user is not authorized to insert the row.
    fn insert(self, user_id: Self::UserId, conn: &mut C) -> Result<Self::Row, Self::Error>;

    /// Attempts to convert the builder into an insertable object, executing
    /// the necessary database operations if required.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user performing the insert operation.
    /// * `conn` - A mutable reference to the database connection.
    ///
    /// # Errors
    ///
    /// * `InsertError` - If the insert operation fails, it returns an error
    ///   containing the attributes of the insertable object.
    fn try_insert(self, user_id: i32, conn: &mut C)
    -> Result<Self::InsertableVariant, Self::Error>;
}

#[derive(Clone, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
/// Enumeration of the possible errors associated to the frontend insert
/// operations.
pub enum InsertError<FieldName> {
    /// A build error occurred.
    BuilderError(BuilderError<FieldName>),
    /// A validation error occurred.
    ValidationError(validation_errors::Error<FieldName>),
    /// A diesel error occurred.
    DieselError(String),
    /// A compatibility rule was violated.
    CompatibilityRule(String, FieldName, FieldName),
    /// Unknown attribute error.
    UnknownAttribute(String),
    /// A server error occurred.
    ServerError(GenericBackendRequestError),
}

impl<FieldName> InsertError<FieldName> {
    /// Converts the `InsertError` into a new `InsertError` with a different
    /// field name.
    pub fn into_field_name<F, NewFieldName>(self, convert: F) -> InsertError<NewFieldName>
    where
        F: Fn(FieldName) -> NewFieldName,
    {
        match self {
            InsertError::BuilderError(error) => {
                InsertError::BuilderError(error.into_field_name(convert))
            }
            InsertError::ValidationError(error) => {
                InsertError::ValidationError(error.into_field_name(convert))
            }
            InsertError::DieselError(error) => InsertError::DieselError(error),
            InsertError::CompatibilityRule(constraint_name, left, right) => {
                InsertError::CompatibilityRule(constraint_name, convert(left), convert(right))
            }
            InsertError::UnknownAttribute(attribute) => InsertError::UnknownAttribute(attribute),
            InsertError::ServerError(error) => InsertError::ServerError(error),
        }
    }
}

impl<FieldName: core::fmt::Debug + core::fmt::Display> core::error::Error
    for InsertError<FieldName>
{
}

impl<FieldName: core::fmt::Display> core::fmt::Display for InsertError<FieldName> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertError::BuilderError(error) => {
                <BuilderError<FieldName> as core::fmt::Display>::fmt(error, f)
            }
            InsertError::ValidationError(error) => {
                <validation_errors::Error<FieldName> as core::fmt::Display>::fmt(error, f)
            }
            InsertError::DieselError(error) => {
                write!(f, "Diesel error: {error}")
            }
            InsertError::CompatibilityRule(constraint_name, left, right) => {
                write!(
                    f,
                    "Compatibility rule `{constraint_name}` missing: provided `{left}` is not known to be compatible with `{right}`"
                )
            }
            InsertError::UnknownAttribute(attribute) => {
                write!(f, "Unknown attribute error: `{attribute}`")
            }
            InsertError::ServerError(error) => {
                <GenericBackendRequestError as core::fmt::Display>::fmt(error, f)
            }
        }
    }
}

impl<FieldName: core::fmt::Debug> core::fmt::Debug for InsertError<FieldName> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertError::BuilderError(error) => {
                <BuilderError<FieldName> as core::fmt::Debug>::fmt(error, f)
            }
            InsertError::ValidationError(error) => {
                <validation_errors::Error<FieldName> as core::fmt::Debug>::fmt(error, f)
            }
            InsertError::DieselError(error) => {
                write!(f, "Diesel error: {error:?}")
            }
            InsertError::CompatibilityRule(constraint_name, left, right) => {
                write!(
                    f,
                    "Compatibility rule `{constraint_name}` missing: provided `{left:?}` is not known to be compatible with `{right:?}`"
                )
            }
            InsertError::UnknownAttribute(attribute) => {
                write!(f, "Unknown attribute error: `{attribute:?}`")
            }
            InsertError::ServerError(error) => {
                <GenericBackendRequestError as core::fmt::Debug>::fmt(error, f)
            }
        }
    }
}

impl<FieldName> From<BuilderError<FieldName>> for InsertError<FieldName> {
    fn from(error: BuilderError<FieldName>) -> Self {
        InsertError::BuilderError(error)
    }
}

impl<FieldName> From<validation_errors::Error<FieldName>> for InsertError<FieldName> {
    fn from(error: validation_errors::Error<FieldName>) -> Self {
        InsertError::ValidationError(error)
    }
}

impl<FieldName> From<validation_errors::SingleFieldError<FieldName>> for InsertError<FieldName> {
    fn from(error: validation_errors::SingleFieldError<FieldName>) -> Self {
        let validation_error: validation_errors::Error<FieldName> = error.into();
        validation_error.into()
    }
}

impl<FieldName> From<validation_errors::DoubleFieldError<FieldName>> for InsertError<FieldName> {
    fn from(error: validation_errors::DoubleFieldError<FieldName>) -> Self {
        let validation_error: validation_errors::Error<FieldName> = error.into();
        validation_error.into()
    }
}

impl<FieldName> From<diesel::result::Error> for InsertError<FieldName>
where
    FieldName: FromStr,
{
    fn from(error: diesel::result::Error) -> Self {
        // We check whether the error is a foreign key violation, and
        // if it is a compatibility rule violation.
        if let diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::ForeignKeyViolation,
            info,
        ) = &error
        {
            if let Some(detail) = info.details() {
                if detail.contains("compatibility_rules") {
                    // We retrieve the names of the two columns involved in the
                    // compatibility rule violation.
                    //
                    // An example of a detail is:
                    //
                    // Some("Key (frozen_with,
                    // frozen_container_id)=(af54ac83-a40f-4b83-940c-998fc70332ac,
                    // c11af75e-42cd-4972-bdd0-2edce335b5af) is not present in table
                    // \"compatibility_rules\".")
                    //
                    // First, we split the detail by the `=` character to get the
                    // part that contains the two columns.
                    if let Some(first_part) = detail.split('=').next() {
                        // Then, we remove the leading `Key (` and trailing `)` characters.
                        let first_part =
                            first_part.trim_start_matches("Key (").trim_end_matches(')');

                        // Now, we split the first part by the `,` character to get the two
                        // column names, which need to be trimmed of any whitespace.
                        if let Ok(mut field_names) = first_part
                            .split(',')
                            .map(str::trim)
                            .map(|s| s.parse::<FieldName>())
                            .collect::<Result<Vec<FieldName>, _>>()
                        {
                            if let (Some(second), Some(first)) =
                                (field_names.pop(), field_names.pop())
                            {
                                // We return a compatibility rule error with the two field names.
                                return InsertError::CompatibilityRule(
                                    info.constraint_name()
                                        .unwrap_or("Unknown constraint")
                                        .to_owned(),
                                    first,
                                    second,
                                );
                            }
                        }
                    }
                }
            }
        }

        InsertError::DieselError(error.to_string())
    }
}

impl<FieldName> From<GenericBackendRequestError> for InsertError<FieldName> {
    fn from(error: GenericBackendRequestError) -> Self {
        InsertError::ServerError(error)
    }
}

impl<FieldName> From<Infallible> for InsertError<FieldName> {
    fn from(_error: Infallible) -> Self {
        unreachable!()
    }
}
