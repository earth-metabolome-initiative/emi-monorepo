//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(Float84mul, " * ", diesel::sql_types::Double, backend: diesel::pg::Pg);
/// Trait for the `*` operator.
pub trait HasFloat84mul:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Double>
{
    /// The function to create the `Float84mul` struct representing the `*` operator.
    fn float84mul<Rhs>(self, rhs: Rhs) -> Float84mul<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Float>,
    {
        Float84mul::new(self, rhs.as_expression())
    }
}

impl<T> HasFloat84mul for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Double>
{
}