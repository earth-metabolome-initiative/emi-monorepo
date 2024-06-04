//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(Float48le, " <= ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `<=` operator.
pub trait HasFloat48le:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Float>
{
    /// The function to create the `Float48le` struct representing the `<=` operator.
    fn float48le<Rhs>(self, rhs: Rhs) -> Float48le<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Double>,
    {
        Float48le::new(self, rhs.as_expression())
    }
}

impl<T> HasFloat48le for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Float>
{
}
