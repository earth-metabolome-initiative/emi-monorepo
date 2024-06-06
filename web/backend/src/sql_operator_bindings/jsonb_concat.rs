//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(JsonbConcat, " || ", diesel::sql_types::Jsonb, backend: diesel::pg::Pg);
/// Trait for the `||` operator.
pub trait HasJsonbConcat:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Jsonb>
{
    /// The function to create the `JsonbConcat` struct representing the `||` operator.
    fn jsonb_concat<Rhs>(self, rhs: Rhs) -> JsonbConcat<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Jsonb>,
    {
        JsonbConcat::new(self, rhs.as_expression())
    }
}

impl<T> HasJsonbConcat for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Jsonb>
{
}