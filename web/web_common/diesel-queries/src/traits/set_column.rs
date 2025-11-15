//! Helper method for builders.

use crate::traits::GetColumn;

/// Trait for setting a column from a Diesel model builder.
pub trait SetColumn<C: diesel::Column>: GetColumn<C> {
    /// The error type returned when setting the column.
    type Error: std::error::Error;

    /// Sets the column.
    ///
    /// # Errors
    ///
    /// Returns an error if the column cannot be set.
    fn set_column(self, value: Self::ColumnType) -> Result<Self, Self::Error>;
}
