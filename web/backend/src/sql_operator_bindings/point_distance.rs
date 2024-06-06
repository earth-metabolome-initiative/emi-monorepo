//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(PointDistance, " <-> ", diesel::sql_types::Double, backend: diesel::pg::Pg);
/// Trait for the `<->` operator.
pub trait HasPointDistance:
    Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
    /// The function to create the `PointDistance` struct representing the `<->` operator.
    fn point_distance<Rhs>(self, rhs: Rhs) -> PointDistance<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<postgis_diesel::sql_types::Geometry>,
    {
        PointDistance::new(self, rhs.as_expression())
    }
}

impl<T> HasPointDistance for T where
    T: Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
}