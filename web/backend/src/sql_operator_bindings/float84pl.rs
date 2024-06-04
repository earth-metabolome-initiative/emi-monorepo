//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(Float84pl, " + ", diesel::sql_types::Double, backend: diesel::pg::Pg);
/// Trait for the `+` operator.
pub trait HasFloat84pl:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Double>
{
    /// The function to create the `Float84pl` struct representing the `+` operator.
    fn float84pl<Rhs>(self, rhs: Rhs) -> Float84pl<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Float>,
    {
        Float84pl::new(self, rhs.as_expression())
    }
}

impl<T> HasFloat84pl for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Double>
{
}
