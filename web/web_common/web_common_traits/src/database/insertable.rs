//! Submodule defining traits regarding tables that can be extended with new
//! rows.
use std::convert::Infallible;

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

/// A trait for types that can be constructed in the frontend or backend to
/// execute the insert operation.
pub trait InsertableVariant<C>: Sized {
    /// The associated row type which can be inserted into the database.
    type Row: Insertable<
            InsertableBuilder = Self,
            InsertableVariant = <Self as InsertableVariant<C>>::InsertableVariant,
        > + diesel::associations::HasTable;
    /// The associated insertable variant type which can be constructed in the
    /// frontend or backend to execute the insert operation.
    type InsertableVariant: diesel::Insertable<<Self::Row as diesel::associations::HasTable>::Table>;
    /// The error type which may occur during the insert operation.
    type Error: From<diesel::result::Error>;
    /// The expected user ID type.
    type UserId;

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
}

#[cfg(feature = "postgres")]
/// A trait for types that can be constructed solely in the backend to
/// execute the insert operation.
pub trait UncheckedInsertableVariant:
    InsertableVariant<diesel::pg::PgConnection> + TryInto<Self::InsertableVariant>
where
    <Self as InsertableVariant<diesel::pg::PgConnection>>::Error:
        From<<Self as TryInto<Self::InsertableVariant>>::Error>,
    diesel::query_builder::InsertStatement<
        <Self::Row as diesel::associations::HasTable>::Table,
        <Self::InsertableVariant as diesel::Insertable<
            <Self::Row as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<'query, diesel::pg::PgConnection, Self::Row>,
{
    /// Inserts the row into the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - The connection to the database.
    ///
    /// # Returns
    ///
    /// The inserted row, if the operation was successful.
    ///
    /// # Errors
    ///
    /// * If the row cannot be inserted.
    ///
    /// # Safety
    ///
    /// This method is unsafe because it does not perform a check
    /// on the user ID on the applicative side. It is meant to be
    /// used on structural tables which are not meant to be inserted
    /// or modified by users, but rather by the backend itself.
    fn unchecked_insert(
        self,
        conn: &mut diesel::pg::PgConnection,
    ) -> Result<Self::Row, <Self as InsertableVariant<diesel::pg::PgConnection>>::Error> {
        use diesel::{RunQueryDsl, associations::HasTable};
        let insertable_struct: Self::InsertableVariant = self.try_into()?;
        Ok(diesel::insert_into(Self::Row::table()).values(insertable_struct).get_result(conn)?)
    }
}

#[cfg(feature = "postgres")]
impl<T> UncheckedInsertableVariant for T
where
    T: InsertableVariant<diesel::pg::PgConnection> + TryInto<Self::InsertableVariant>,
    <Self as InsertableVariant<diesel::pg::PgConnection>>::Error:
        From<<Self as TryInto<Self::InsertableVariant>>::Error>,
    diesel::query_builder::InsertStatement<
        <Self::Row as diesel::associations::HasTable>::Table,
        <Self::InsertableVariant as diesel::Insertable<
            <Self::Row as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<'query, diesel::pg::PgConnection, Self::Row>,
{
    // The implementation is provided by the trait definition.
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

impl<FieldName> From<diesel::result::Error> for InsertError<FieldName> {
    fn from(error: diesel::result::Error) -> Self {
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
