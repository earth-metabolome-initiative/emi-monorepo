//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(PointBelow, " <^ ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `<^` operator.
pub trait HasPointBelow:
    Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
    /// The function to create the `PointBelow` struct representing the `<^` operator.
    fn point_below<Rhs>(self, rhs: Rhs) -> PointBelow<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<postgis_diesel::sql_types::Geometry>,
    {
        PointBelow::new(self, rhs.as_expression())
    }
}

impl<T> HasPointBelow for T where
    T: Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
}
