//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(ClosePl, " ## ", crate::database::sql_type_bindings::Point, backend: diesel::pg::Pg);
/// Trait for the `##` operator.
pub trait HasClosePl:
    Sized + diesel::expression::Expression<SqlType = crate::database::sql_type_bindings::Point>
{
    /// The function to create the `ClosePl` struct representing the `##` operator.
    fn close_pl<Rhs>(self, rhs: Rhs) -> ClosePl<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<postgis_diesel::sql_types::Geometry>,
    {
        ClosePl::new(self, rhs.as_expression())
    }
}

impl<T> HasClosePl for T where
    T: Sized + diesel::expression::Expression<SqlType = crate::database::sql_type_bindings::Point>
{
}