//! Submodule providing the `ReplaceFieldName` trait.

/// Trait for replacing the field name in validation errors.
pub trait ReplaceFieldName {
    /// The type of the field name.
    type FieldName;
    /// The output object with the new field name type.
    type Replaced<NewFieldName>;

    /// Converts the error into the provided new attribute type.
    fn into_field_name<Map, NewFieldName>(self, map: Map) -> Self::Replaced<NewFieldName>
    where
        Map: Fn(Self::FieldName) -> NewFieldName;
}
