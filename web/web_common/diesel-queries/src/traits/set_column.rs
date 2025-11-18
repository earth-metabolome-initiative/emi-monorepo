//! Helper method for builders.

use common_traits::builder::Attributed;
use validation_errors::prelude::ValidationError;

use crate::traits::{
    ForeignKeyCompatibleColumn, ForeignKeyCompatibleType, HorizontalSameAs, MaybeGetColumn,
    TableIsExtensionOf, VerticalSameAs, get_column::TypedColumn,
};

/// Trait for setting a column from a Diesel model builder which cannot fail.
pub trait SetColumn<C: TypedColumn>: Sized {
    /// Sets the column.
    fn set(self, value: C::Type) -> Self;
}

/// Trait for setting a column from a Diesel model builder.
pub trait TrySetColumn<C: TypedColumn>: Sized + Attributed {
    /// Sets the column.
    ///
    /// # Errors
    ///
    /// Returns an error if the column cannot be set.
    fn try_set(self, value: C::Type) -> Result<Self, ValidationError<Self::Attribute>>;
}

impl<T, C> TrySetColumn<C> for T
where
    T: SetColumn<C> + Attributed,
    C: TypedColumn,
{
    #[inline]
    fn try_set(self, value: C::Type) -> Result<Self, ValidationError<Self::Attribute>> {
        Ok(self.set(value))
    }
}

/// Trait for setting a horizontal same-as column from a Diesel model builder.
pub trait SetHorizontalColumn<
    HostColumn: HorizontalSameAs<Referenced, Key>,
    Referenced: TypedColumn<SqlType = HostColumn::SqlType, Type = HostColumn::Type>,
    Key: TypedColumn<Table = HostColumn::Table>,
>: SetColumn<HostColumn>
{
    #[inline]
    /// Sets the horizontal same-as column.
    fn set_horizontal(self, referred: &dyn MaybeGetColumn<Referenced>) -> Self {
        if let Some(value) = referred.maybe_get_column() { self.set(value.clone()) } else { self }
    }
}

impl<T, HostColumn, Referenced, Key> SetHorizontalColumn<HostColumn, Referenced, Key> for T
where
    T: SetColumn<HostColumn>,
    HostColumn: HorizontalSameAs<Referenced, Key>,
    Referenced: TypedColumn<SqlType = HostColumn::SqlType, Type = HostColumn::Type>,
    Key: TypedColumn<Table = HostColumn::Table>,
{
}

/// Trait for setting a horizontal same-as column from a Diesel model builder.
pub trait TrySetHorizontalColumn<
    HostColumn: HorizontalSameAs<Referenced, Key>,
    Referenced: TypedColumn<SqlType = HostColumn::SqlType, Type = HostColumn::Type>,
    Key: TypedColumn<Table = HostColumn::Table>,
>: TrySetColumn<HostColumn>
{
    #[inline]
    /// Sets the horizontal same-as column.
    ///
    /// # Errors
    ///
    /// Returns an error if the column cannot be set.
    fn try_set_horizontal(
        self,
        referred: &dyn MaybeGetColumn<Referenced>,
    ) -> Result<Self, ValidationError<Self::Attribute>> {
        if let Some(value) = referred.maybe_get_column() {
            self.try_set(value.clone())
        } else {
            Ok(self)
        }
    }
}

impl<T, HostColumn, Referenced, Key> TrySetHorizontalColumn<HostColumn, Referenced, Key> for T
where
    T: TrySetColumn<HostColumn>,
    HostColumn: HorizontalSameAs<Referenced, Key>,
    Referenced: TypedColumn<SqlType = HostColumn::SqlType, Type = HostColumn::Type>,
    Key: TypedColumn<Table = HostColumn::Table>,
{
}

/// Trait for setting a vertical same-as column from a Diesel model builder
/// which cannot fail.
pub trait SetVerticalColumn<HostColumn: VerticalSameAs<AncestorColumn>, AncestorColumn: TypedColumn>:
    SetColumn<AncestorColumn> + MaybeGetColumn<HostColumn>
where
    HostColumn::Table: TableIsExtensionOf<AncestorColumn::Table>,
    HostColumn: ForeignKeyCompatibleColumn<AncestorColumn>,
    HostColumn::Type: ForeignKeyCompatibleType<AncestorColumn::Type>,
{
    #[inline]
    /// Sets the vertical same-as column.
    fn set_vertical(self) -> Self {
        if let Some(value) = self.maybe_get_column().cloned().and_then(|v| v.optionify()) {
            self.set(value)
        } else {
            self
        }
    }
}

impl<T, HostColumn, AncestorColumn> SetVerticalColumn<HostColumn, AncestorColumn> for T
where
    T: SetColumn<AncestorColumn> + MaybeGetColumn<HostColumn>,
    HostColumn: VerticalSameAs<AncestorColumn>,
    AncestorColumn: TypedColumn<SqlType = HostColumn::SqlType, Type = HostColumn::Type>,
    HostColumn::Table: TableIsExtensionOf<AncestorColumn::Table>,
{
}

/// Trait for setting a vertical same-as column from a Diesel model builder.
pub trait TrySetVerticalColumn<
    HostColumn: VerticalSameAs<AncestorColumn>,
    AncestorColumn: TypedColumn,
>: TrySetColumn<AncestorColumn> + MaybeGetColumn<HostColumn> where
    HostColumn::Table: TableIsExtensionOf<AncestorColumn::Table>,
    HostColumn: ForeignKeyCompatibleColumn<AncestorColumn>,
    HostColumn::Type: ForeignKeyCompatibleType<AncestorColumn::Type>,
{
    #[inline]
    /// Sets the vertical same-as column.
    ///
    /// # Errors
    ///
    /// Returns an error if the column cannot be set.
    fn try_set_vertical(self) -> Result<Self, ValidationError<Self::Attribute>> {
        if let Some(value) = self.maybe_get_column().cloned().and_then(|v| v.optionify()) {
            self.try_set(value)
        } else {
            Ok(self)
        }
    }
}

impl<T, HostColumn, AncestorColumn> TrySetVerticalColumn<HostColumn, AncestorColumn> for T
where
    T: TrySetColumn<AncestorColumn> + MaybeGetColumn<HostColumn>,
    HostColumn: VerticalSameAs<AncestorColumn>,
    AncestorColumn: TypedColumn<SqlType = HostColumn::SqlType, Type = HostColumn::Type>,
    HostColumn::Table: TableIsExtensionOf<AncestorColumn::Table>,
{
}
