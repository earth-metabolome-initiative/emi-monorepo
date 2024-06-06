//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(GeometryEq, " = ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `=` operator.
pub trait HasGeometryEq:
    Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
    /// The function to create the `GeometryEq` struct representing the `=` operator.
    fn geometry_eq<Rhs>(self, rhs: Rhs) -> GeometryEq<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<postgis_diesel::sql_types::Geometry>,
    {
        GeometryEq::new(self, rhs.as_expression())
    }
}

impl<T> HasGeometryEq for T where
    T: Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
}