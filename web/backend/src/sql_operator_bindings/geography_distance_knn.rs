//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(GeographyDistanceKnn, " <-> ", diesel::sql_types::Double, backend: diesel::pg::Pg);
/// Trait for the `<->` operator.
pub trait HasGeographyDistanceKnn:
    Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geography>
{
    /// The function to create the `GeographyDistanceKnn` struct representing the `<->` operator.
    fn geography_distance_knn<Rhs>(self, rhs: Rhs) -> GeographyDistanceKnn<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<postgis_diesel::sql_types::Geography>,
    {
        GeographyDistanceKnn::new(self, rhs.as_expression())
    }
}

impl<T> HasGeographyDistanceKnn for T where
    T: Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geography>
{
}