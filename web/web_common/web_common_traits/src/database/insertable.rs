//! Submodule defining traits regarding tables that can be extended with new
//! rows.
use common_traits::prelude::BuilderError;

/// A trait for types that can be inserted into the database.
pub trait Insertable {
    /// The associated type which can be constructed in the frontend or backend
    /// to execute the insert operation.
    type InsertableVariant: InsertableVariant<Row = Self>;
    /// The associated builder type which can be constructed in the frontend or
    /// backend to create the insertable variant.
    type InsertableBuilder: InsertableBuilder<Row = Self, Product = Self::InsertableVariant>;

    #[must_use]
    /// Returns a new insertable variant builder.
    fn new() -> Self::InsertableBuilder {
        Default::default()
    }
}

/// A trait for types that can be constructed in the frontend or backend to
/// execute the insert operation.
pub trait InsertableVariant {
    /// The associated row type which can be inserted into the database.
    type Row: Insertable<InsertableVariant = Self, InsertableBuilder = Self::InsertableBuilder>;
    /// The type of connection to the database.
    type Conn;
    /// The builder type which can be constructed in the frontend or backend to
    /// create the insertable variant.
    type InsertableBuilder: InsertableBuilder<Row = Self::Row, Product = Self>;
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
    fn insert(
        self,
        user_id: &Self::UserId,
        conn: &mut Self::Conn,
    ) -> impl core::future::Future<
        Output = Result<
            Self::Row,
            InsertError<<Self::InsertableBuilder as common_traits::prelude::Builder>::Attribute>,
        >,
    >;
}

#[cfg(feature = "backend")]
/// A trait for types that can be constructed in the backend only to
/// execute the insert operation without a user ID.
pub trait BackendInsertableVariant: InsertableVariant {
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
    fn backend_insert(
        self,
        conn: &mut Self::Conn,
    ) -> impl core::future::Future<
        Output = Result<
            Self::Row,
            InsertError<<Self::InsertableBuilder as common_traits::prelude::Builder>::Attribute>,
        >,
    >;
}

/// A trait for types that can be constructed in the frontend or backend to
/// create the insertable variant.
pub trait InsertableBuilder:
    common_traits::prelude::Builder<
    Object = <Self as InsertableBuilder>::Product,
    Error = InsertError<<Self as common_traits::prelude::Builder>::Attribute>,
>
{
    /// The associated row type which can be inserted into the database.
    type Row: Insertable<InsertableBuilder = Self, InsertableVariant = Self::Product>;
    /// The product that can be created by the builder.
    type Product: InsertableVariant<Row = Self::Row, InsertableBuilder = Self>
        + TryInto<Self, Error = Self::Error>;
}

/// Enumeration of the possible errors associated to the frontend insert
/// operations.
pub enum InsertError<FieldName> {
    /// A build error occurred.
    BuilderError(BuilderError<FieldName>),
    /// A validation error occurred.
    ValidationError(validation_errors::Error<FieldName>),
    /// A diesel error occurred.
    DieselError(diesel::result::Error),
    /// A server error occurred.
    ServerError(backend_errors::Error),
}

impl<FieldName: core::fmt::Display> core::error::Error for InsertError<FieldName> {}

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
                <diesel::result::Error as core::fmt::Display>::fmt(error, f)
            }
            InsertError::ServerError(error) => {
                <backend_errors::Error as core::fmt::Display>::fmt(error, f)
            }
        }
    }
}

impl<FieldName: core::fmt::Display> core::fmt::Debug for InsertError<FieldName> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        <InsertError<FieldName> as core::fmt::Display>::fmt(self, f)
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
        InsertError::DieselError(error)
    }
}

impl<FieldName> From<backend_errors::Error> for InsertError<FieldName> {
    fn from(error: backend_errors::Error) -> Self {
        InsertError::ServerError(error)
    }
}
