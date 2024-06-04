//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(PolyLeft, " << ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `<<` operator.
pub trait HasPolyLeft:
    Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
    /// The function to create the `PolyLeft` struct representing the `<<` operator.
    fn poly_left<Rhs>(self, rhs: Rhs) -> PolyLeft<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<postgis_diesel::sql_types::Geometry>,
    {
        PolyLeft::new(self, rhs.as_expression())
    }
}

impl<T> HasPolyLeft for T where
    T: Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
}
