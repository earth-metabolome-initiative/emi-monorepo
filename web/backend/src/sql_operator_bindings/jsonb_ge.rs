//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(JsonbGe, " >= ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `>=` operator.
pub trait HasJsonbGe:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Jsonb>
{
    /// The function to create the `JsonbGe` struct representing the `>=` operator.
    fn jsonb_ge<Rhs>(self, rhs: Rhs) -> JsonbGe<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Jsonb>,
    {
        JsonbGe::new(self, rhs.as_expression())
    }
}

impl<T> HasJsonbGe for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Jsonb>
{
}
