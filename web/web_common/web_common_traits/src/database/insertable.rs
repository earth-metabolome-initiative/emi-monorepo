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
    /// 
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
pub enum InsertError<A> {
    /// A build error occurred.
    BuilderError(BuilderError<A>),
    /// A validation error occurred.
    ValidationError(validation_errors::Error),
    /// A diesel error occurred.
    DieselError(diesel::result::Error),
    /// A server error occurred.
    ServerError(server_errors::Error),
}

impl<A: core::fmt::Display> core::error::Error for InsertError<A> {}

impl<A: core::fmt::Display> core::fmt::Display for InsertError<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InsertError::BuilderError(error) => <BuilderError<A> as core::fmt::Display>::fmt(error, f),
            InsertError::ValidationError(error) => <validation_errors::Error as core::fmt::Display>::fmt(error, f),
            InsertError::DieselError(error) => <diesel::result::Error as core::fmt::Display>::fmt(error, f),
            InsertError::ServerError(error) => <server_errors::Error as core::fmt::Display>::fmt(error, f),
        }
    }
}

impl<A: core::fmt::Display> core::fmt::Debug for InsertError<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        <InsertError<A> as core::fmt::Display>::fmt(self, f)
    }
}

impl<A> From<BuilderError<A>> for InsertError<A> {
    fn from(error: BuilderError<A>) -> Self {
        InsertError::BuilderError(error)
    }
}

impl<A> From<validation_errors::Error> for InsertError<A> {
    fn from(error: validation_errors::Error) -> Self {
        InsertError::ValidationError(error)
    }
}

impl<A> From<diesel::result::Error> for InsertError<A> {
    fn from(error: diesel::result::Error) -> Self {
        InsertError::DieselError(error)
    }
}

impl<A> From<server_errors::Error> for InsertError<A> {
    fn from(error: server_errors::Error) -> Self {
        InsertError::ServerError(error)
    }
}
