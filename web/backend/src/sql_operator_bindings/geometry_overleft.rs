//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(GeometryOverleft, " &< ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `&<` operator.
pub trait HasGeometryOverleft:
    Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
    /// The function to create the `GeometryOverleft` struct representing the `&<` operator.
    fn geometry_overleft<Rhs>(self, rhs: Rhs) -> GeometryOverleft<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<postgis_diesel::sql_types::Geometry>,
    {
        GeometryOverleft::new(self, rhs.as_expression())
    }
}

impl<T> HasGeometryOverleft for T where
    T: Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
}
