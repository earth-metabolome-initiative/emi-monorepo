//! Helper method for builders.

/// Trait for getting a column from a Diesel model.
pub trait GetColumn<C: diesel::Column>: Sized {
    /// The type of the column.
    type ColumnType: Clone;

    /// Gets the column, if it was set.
    fn get_column(&self) -> &Self::ColumnType;
}

/// Trait for getting a column from a Diesel model builder.
pub trait MaybeGetColumn<C: diesel::Column>: Sized {
    /// The type of the column.
    type ColumnType: Clone;

    /// Gets the column, if it was set.
    fn maybe_get_column(&self) -> Option<&Self::ColumnType>;
}

impl<T, C> MaybeGetColumn<C> for T
where
    T: GetColumn<C>,
    C: diesel::Column,
{
    type ColumnType = <T as GetColumn<C>>::ColumnType;

    fn maybe_get_column(&self) -> Option<&Self::ColumnType> {
        Some(self.get_column())
    }
}
