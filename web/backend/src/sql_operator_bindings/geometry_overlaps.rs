//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(GeometryOverlaps, " && ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `&&` operator.
pub trait HasGeometryOverlaps:
    Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
    /// The function to create the `GeometryOverlaps` struct representing the `&&` operator.
    fn geometry_overlaps<Rhs>(self, rhs: Rhs) -> GeometryOverlaps<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<postgis_diesel::sql_types::Geometry>,
    {
        GeometryOverlaps::new(self, rhs.as_expression())
    }
}

impl<T> HasGeometryOverlaps for T where
    T: Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
}
