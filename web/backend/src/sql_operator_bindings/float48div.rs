//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(Float48div, " / ", diesel::sql_types::Double, backend: diesel::pg::Pg);
/// Trait for the `/` operator.
pub trait HasFloat48div:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Float>
{
    /// The function to create the `Float48div` struct representing the `/` operator.
    fn float48div<Rhs>(self, rhs: Rhs) -> Float48div<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Double>,
    {
        Float48div::new(self, rhs.as_expression())
    }
}

impl<T> HasFloat48div for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Float>
{
}
