//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(Int4mod, " % ", diesel::sql_types::Integer, backend: diesel::pg::Pg);
/// Trait for the `%` operator.
pub trait HasInt4mod:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Integer>
{
    /// The function to create the `Int4mod` struct representing the `%` operator.
    fn int4mod<Rhs>(self, rhs: Rhs) -> Int4mod<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Integer>,
    {
        Int4mod::new(self, rhs.as_expression())
    }
}

impl<T> HasInt4mod for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Integer>
{
}
