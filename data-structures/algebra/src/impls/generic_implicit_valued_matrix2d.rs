//! Submodule proving a 2D matrix with implicit values.

use core::marker::PhantomData;

use crate::traits::{ImplicitValuedMatrix, ImplicitValuedMatrix2D, Matrix2D, Vector};

/// A 2D matrix with implicit values.
pub struct GenericImplicitValuedMatrix2D<Sources, Destinations, Value, ImplicitValueMap> {
    sources: Sources,
    destinations: Destinations,
    function: ImplicitValueMap,
    _value: PhantomData<Value>,
}

impl<Sources, Destinations, Value, ImplicitValueMap> Matrix2D
    for GenericImplicitValuedMatrix2D<Sources, Destinations, Value, ImplicitValueMap>
where
    Sources: Vector,
    Destinations: Vector,
{
    type RowIndex = <Sources as Vector>::Index;
    type ColumnIndex = <Destinations as Vector>::Index;

    fn number_of_rows(&self) -> Self::RowIndex {
        self.sources.len()
    }

    fn number_of_columns(&self) -> Self::ColumnIndex {
        self.destinations.len()
    }
}

impl<Sources, Destinations, Value, ImplicitValueMap> ImplicitValuedMatrix
    for GenericImplicitValuedMatrix2D<Sources, Destinations, Value, ImplicitValueMap>
where
    Sources: Vector,
    Destinations: Vector,
    ImplicitValueMap:
        for<'a> Fn(&'a <Sources as Vector>::Value, &'a <Destinations as Vector>::Value) -> Value,
{
    type Value = Value;

    fn value(
        &self,
        &(row, column): &(<Self as Matrix2D>::RowIndex, <Self as Matrix2D>::ColumnIndex),
    ) -> Value {
        (self.function)(&self.sources[row], &self.destinations[column])
    }
}

impl<Sources, Destinations, Value, ImplicitValueMap> ImplicitValuedMatrix2D
    for GenericImplicitValuedMatrix2D<Sources, Destinations, Value, ImplicitValueMap>
where
    Sources: Vector,
    Destinations: Vector,
    ImplicitValueMap:
        for<'a> Fn(&'a <Sources as Vector>::Value, &'a <Destinations as Vector>::Value) -> Value,
{
    type RowIndices<'a>
        = <Sources as Vector>::Indices<'a>
    where
        Self: 'a;
    type ColumnIndices<'a>
        = <Destinations as Vector>::Indices<'a>
    where
        Self: 'a;

    fn row_indices(&self) -> Self::RowIndices<'_> {
        self.sources.indices()
    }

    fn column_indices(&self) -> Self::ColumnIndices<'_> {
        self.destinations.indices()
    }
}
