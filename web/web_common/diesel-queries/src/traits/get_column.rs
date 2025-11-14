//! Helper method for builders.

/// Trait for getting a column from a Diesel model builder.
pub trait GetColumn<C: diesel::Column>: Sized {
    /// The type of the column.
    type ColumnType: Clone;

    /// Gets the column, if it was set.
    fn get_column(&self) -> Option<&Self::ColumnType>;
}
