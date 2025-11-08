//! Implementation of the `ReplaceFieldName` trait for validation errors.

use crate::{
    structs::{DoubleFieldError, SingleFieldError, ValidationError},
    traits::ReplaceFieldName,
};

impl<FieldName> ReplaceFieldName for SingleFieldError<FieldName> {
    type FieldName = FieldName;
    type Replaced<NewFieldName> = SingleFieldError<NewFieldName>;

    fn into_field_name<Map, NewFieldName>(self, map: Map) -> Self::Replaced<NewFieldName>
    where
        Map: Fn(Self::FieldName) -> NewFieldName,
    {
        match self {
            Self::MustNotBeEmpty(field) => SingleFieldError::MustNotBeEmpty(map(field)),
            Self::MustBeStrictlyPositive(field) => {
                SingleFieldError::MustBeStrictlyPositive(map(field))
            }
            Self::MustBePositive(field) => SingleFieldError::MustBePositive(map(field)),
            Self::MustBeStrictlySmallerThan(field, value) => {
                SingleFieldError::MustBeStrictlySmallerThan(map(field), value)
            }
            Self::MustBeSmallerThan(field, value) => {
                SingleFieldError::MustBeSmallerThan(map(field), value)
            }
            Self::MustBeStrictlyGreaterThan(field, value) => {
                SingleFieldError::MustBeStrictlyGreaterThan(map(field), value)
            }
            Self::MustBeGreaterThan(field, value) => {
                SingleFieldError::MustBeGreaterThan(map(field), value)
            }
            Self::Generic(field, error) => SingleFieldError::Generic(map(field), error),
        }
    }
}

impl<FieldName> ReplaceFieldName for DoubleFieldError<FieldName> {
    type FieldName = FieldName;
    type Replaced<NewFieldName> = DoubleFieldError<NewFieldName>;

    fn into_field_name<Map, NewFieldName>(self, map: Map) -> Self::Replaced<NewFieldName>
    where
        Map: Fn(Self::FieldName) -> NewFieldName,
    {
        match self {
            Self::MustBeDistinct(field1, field2) => {
                DoubleFieldError::MustBeDistinct(map(field1), map(field2))
            }
            Self::MustBeStrictlySmallerThan(field1, field2) => {
                DoubleFieldError::MustBeStrictlySmallerThan(map(field1), map(field2))
            }
            Self::MustBeSmallerThan(field1, field2) => {
                DoubleFieldError::MustBeSmallerThan(map(field1), map(field2))
            }
            Self::MustBeStrictlyGreaterThan(field1, field2) => {
                DoubleFieldError::MustBeStrictlyGreaterThan(map(field1), map(field2))
            }
            Self::MustBeGreaterThan(field1, field2) => {
                DoubleFieldError::MustBeGreaterThan(map(field1), map(field2))
            }
            Self::Generic(field1, field2, error) => {
                DoubleFieldError::Generic(map(field1), map(field2), error)
            }
        }
    }
}

impl<FieldName> ReplaceFieldName for ValidationError<FieldName> {
    type FieldName = FieldName;
    type Replaced<NewFieldName> = ValidationError<NewFieldName>;

    fn into_field_name<Map, NewFieldName>(self, map: Map) -> Self::Replaced<NewFieldName>
    where
        Map: Fn(Self::FieldName) -> NewFieldName,
    {
        match self {
            Self::SingleField(error) => ValidationError::SingleField(error.into_field_name(map)),
            Self::DoubleField(error) => ValidationError::DoubleField(error.into_field_name(map)),
        }
    }
}
