//! Helper method for builders.

/// The rust type for a column.
pub trait TypedColumn: diesel::Column {
    /// The rust type of the column.
    type Type: Clone;
}

/// Trait for getting a column from a Diesel model.
pub trait GetColumn<C: TypedColumn>: Sized {
    /// Gets the column, if it was set.
    fn get_column(&self) -> &C::Type;
}

/// Trait for getting a column from a Diesel model builder.
pub trait MaybeGetColumn<C: TypedColumn> {
    /// Gets the column, if it was set.
    fn maybe_get_column(&self) -> Option<&C::Type>;
}

impl<T, C> MaybeGetColumn<C> for T
where
    T: GetColumn<C>,
    C: TypedColumn,
{
    fn maybe_get_column(&self) -> Option<&C::Type> {
        Some(self.get_column())
    }
}
