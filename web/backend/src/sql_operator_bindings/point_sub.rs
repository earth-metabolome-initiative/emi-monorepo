//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(PointSub, " - ", postgis_diesel::sql_types::Geometry, backend: diesel::pg::Pg);
/// Trait for the `-` operator.
pub trait HasPointSub:
    Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
    /// The function to create the `PointSub` struct representing the `-` operator.
    fn point_sub<Rhs>(self, rhs: Rhs) -> PointSub<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<postgis_diesel::sql_types::Geometry>,
    {
        PointSub::new(self, rhs.as_expression())
    }
}

impl<T> HasPointSub for T where
    T: Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
}
