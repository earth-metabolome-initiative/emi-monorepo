//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(PointLeft, " << ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `<<` operator.
pub trait HasPointLeft:
    Sized + diesel::expression::Expression<SqlType = crate::database::sql_type_bindings::Point>
{
    /// The function to create the `PointLeft` struct representing the `<<` operator.
    fn point_left<Rhs>(self, rhs: Rhs) -> PointLeft<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<crate::database::sql_type_bindings::Point>,
    {
        PointLeft::new(self, rhs.as_expression())
    }
}

impl<T> HasPointLeft for T where
    T: Sized + diesel::expression::Expression<SqlType = crate::database::sql_type_bindings::Point>
{
}
