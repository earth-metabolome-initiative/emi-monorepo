//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(Float8div, " / ", diesel::sql_types::Double, backend: diesel::pg::Pg);
/// Trait for the `/` operator.
pub trait HasFloat8div:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Double>
{
    /// The function to create the `Float8div` struct representing the `/` operator.
    fn float8div<Rhs>(self, rhs: Rhs) -> Float8div<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Double>,
    {
        Float8div::new(self, rhs.as_expression())
    }
}

impl<T> HasFloat8div for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Double>
{
}