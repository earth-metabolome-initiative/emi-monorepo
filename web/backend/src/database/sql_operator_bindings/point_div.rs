//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(PointDiv, " / ", crate::database::sql_type_bindings::Point, backend: diesel::pg::Pg);
/// Trait for the `/` operator.
pub trait HasPointDiv:
    Sized + diesel::expression::Expression<SqlType = crate::database::sql_type_bindings::Point>
{
    /// The function to create the `PointDiv` struct representing the `/` operator.
    fn point_div<Rhs>(self, rhs: Rhs) -> PointDiv<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<crate::database::sql_type_bindings::Point>,
    {
        PointDiv::new(self, rhs.as_expression())
    }
}

impl<T> HasPointDiv for T where
    T: Sized + diesel::expression::Expression<SqlType = crate::database::sql_type_bindings::Point>
{
}