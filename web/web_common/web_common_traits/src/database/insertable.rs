//! Submodule defining traits regarding tables that can be extended with new
//! rows.
use std::{convert::Infallible, str::FromStr};

use common_traits::{builder::IsCompleteBuilder, prelude::BuilderError};
use generic_backend_request_errors::GenericBackendRequestError;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize,
)]
/// Enumeration defining either an identifier or a builder.
pub enum IdOrBuilder<Id, Builder> {
    /// An identifier.
    Id(Id),
    /// A builder.
    Builder(Builder),
}

impl<Id, B: IsCompleteBuilder> IsCompleteBuilder for IdOrBuilder<Id, B> {
    fn is_complete(&self) -> bool {
        match self {
            IdOrBuilder::Id(_) => true,
            IdOrBuilder::Builder(builder) => builder.is_complete(),
        }
    }
}

impl<Id, B: Default> Default for IdOrBuilder<Id, B> {
    fn default() -> Self {
        IdOrBuilder::Builder(B::default())
    }
}

impl<B> From<i16> for IdOrBuilder<i16, B> {
    fn from(id: i16) -> Self {
        IdOrBuilder::Id(id)
    }
}

impl<B> From<i32> for IdOrBuilder<i32, B> {
    fn from(id: i32) -> Self {
        IdOrBuilder::Id(id)
    }
}

impl<B> From<i64> for IdOrBuilder<i64, B> {
    fn from(id: i64) -> Self {
        IdOrBuilder::Id(id)
    }
}

impl<B> From<rosetta_uuid::Uuid> for IdOrBuilder<rosetta_uuid::Uuid, B> {
    fn from(id: rosetta_uuid::Uuid) -> Self {
        IdOrBuilder::Id(id)
    }
}

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

/// A trait for types that can have their most concrete table set.
pub trait MostConcreteTable {
    /// Sets the most concrete table for the type.
    ///
    /// # Arguments
    ///
    /// * `table_name` - The name of the most concrete table.
    fn set_most_concrete_table(&mut self, table_name: &str);
}

impl<T> MostConcreteTable for Option<T> {
    fn set_most_concrete_table(&mut self, _table_name: &str) {}
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
    /// A foreign key was violated.
    ForeignKeyViolation {
        /// The name of the table where the row is being inserted.
        table: String,
        /// The name of the table where the foreign key points to.
        foreign_table: String,
        /// The name of the foreign key constraint that was violated.
        foreign_key: String,
        /// The names of the columns involved in the foreign key violation.
        columns: Vec<FieldName>,
        /// The expected values of the columns.
        expected_values: Vec<String>,
    },
    /// A unique constraint was violated.
    UniqueConstraintViolation {
        /// The name of the table where the row is being inserted.
        table: String,
        /// The names of the columns involved in the unique constraint
        /// violation.
        columns: Vec<FieldName>,
        /// The expected values of the columns.
        expected_values: Vec<String>,
    },
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
            InsertError::ForeignKeyViolation {
                table,
                foreign_table,
                foreign_key,
                columns,
                expected_values,
            } => {
                InsertError::ForeignKeyViolation {
                    table,
                    foreign_table,
                    foreign_key,
                    columns: columns.into_iter().map(convert).collect(),
                    expected_values,
                }
            }
            InsertError::UniqueConstraintViolation { table, columns, expected_values } => {
                InsertError::UniqueConstraintViolation {
                    table,
                    columns: columns.into_iter().map(convert).collect(),
                    expected_values,
                }
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
            InsertError::ForeignKeyViolation {
                table,
                foreign_table,
                foreign_key,
                columns,
                expected_values,
            } => {
                write!(
                    f,
                    "Foreign key \"{foreign_key}\" violated while attempting to insert a row in table \"{foreign_table}\": there is no row in table \"{table}\" with the requested values {{{}}}. Possibly you forgot to insert a required row first, or you misplaced it?",
                    columns
                        .iter()
                        .zip(expected_values.iter())
                        .map(|(col, val)| format!("{}: {}", col, val))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            InsertError::UniqueConstraintViolation { table, columns, expected_values } => {
                write!(
                    f,
                    "Duplication: there is already a row in table \"{table}\" with the requested values {{{}}}.",
                    columns
                        .iter()
                        .zip(expected_values.iter())
                        .map(|(col, val)| format!("{}: {}", col, val))
                        .collect::<Vec<_>>()
                        .join(", ")
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

impl<FieldName: core::fmt::Debug + core::fmt::Display> core::fmt::Debug for InsertError<FieldName> {
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
            InsertError::UniqueConstraintViolation { .. }
            | InsertError::ForeignKeyViolation { .. } => <Self as core::fmt::Display>::fmt(self, f),
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
                // We retrieve the names of the columns involved in the
                // foreign key violation.
                //
                // An example of a detail is:
                //
                // Some("Key (frozen_with,
                // frozen_container_id)=(af54ac83-a40f-4b83-940c-998fc70332ac,
                // c11af75e-42cd-4972-bdd0-2edce335b5af) is not present in table
                // \"compatibility_rules\".")
                //

                // We extract the names of the columns, their values and the name
                // of the associated table.

                let mut equal_split = detail.split('=');
                let before_equal = equal_split.next().and_then(|s| {
                    Some(
                        s.strip_suffix(")")?
                            .split_once("(")?
                            .1
                            .split(",")
                            .map(str::trim)
                            .map(FieldName::from_str)
                            .collect::<Result<Vec<_>, _>>()
                            .ok()?,
                    )
                });
                let after_equal = equal_split
                    .next()
                    .and_then(|s| Some(s.strip_prefix("(")?.rsplit(")").last()?.split(",")));
                let table_name = detail.rsplit_once("table \"").and_then(|(_, after)| {
                    after.strip_suffix("\".") // We remove the trailing quote and dot.
                });
                if let (Some(columns), Some(expected_values), Some(table), Some(foreign_table)) =
                    (before_equal, after_equal, table_name, info.table_name())
                {
                    let expected_values =
                        expected_values.map(str::trim).map(String::from).collect();
                    return InsertError::ForeignKeyViolation {
                        table: table.to_owned(),
                        foreign_table: foreign_table.to_owned(),
                        foreign_key: info.constraint_name().unwrap_or("unknown").to_string(),
                        columns,
                        expected_values,
                    };
                }
            }
        }

        if let diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            info,
        ) = &error
        {
            if let Some(detail) = info.details() {
                // We retrieve the names of the columns involved in the
                // unique violation.
                //
                // An example of a detail is:
                //
                // Some("Key (procedure_template, name)=(5, Phone) already exists.")
                // Some("Key (procedure_template, name)=(18, Metal Bead 3mm) already exists.")
                //

                // We extract the names of the columns, their values and the name
                // of the associated table.

                let mut equal_split = detail.split('=');
                let before_equal = equal_split.next().and_then(|s| {
                    Some(
                        s.strip_suffix(")")?
                            .split_once("(")?
                            .1
                            .split(",")
                            .map(str::trim)
                            .map(FieldName::from_str)
                            .collect::<Result<Vec<_>, _>>()
                            .ok()?,
                    )
                });
                let after_equal = equal_split
                    .next()
                    .and_then(|s| Some(s.strip_prefix("(")?.rsplit(")").last()?.split(",")));

                if let (Some(columns), Some(expected_values), Some(table)) =
                    (before_equal, after_equal, info.table_name())
                {
                    let expected_values =
                        expected_values.map(str::trim).map(String::from).collect();
                    return InsertError::UniqueConstraintViolation {
                        table: table.to_owned(),
                        columns,
                        expected_values,
                    };
                }
            }
        }

        if let diesel::result::Error::DatabaseError(_, info) = &error {
            println!("Message: {}", info.message());
            if let Some(constraint) = info.constraint_name() {
                println!("Constraint: {}", constraint);
            }
            if let Some(table) = info.table_name() {
                println!("Table: {}", table);
            }
            if let Some(details) = info.details() {
                println!("Details: {}", details);
            }
            if let Some(hint) = info.hint() {
                println!("Hint: {}", hint);
            }
            if let Some(column_name) = info.column_name() {
                println!("Column: {}", column_name);
            }
        } else {
            println!("Diesel error did not contain database error info.");
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
