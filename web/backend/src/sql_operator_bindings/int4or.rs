//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(Int4or, " | ", diesel::sql_types::Integer, backend: diesel::pg::Pg);
/// Trait for the `|` operator.
pub trait HasInt4or:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Integer>
{
    /// The function to create the `Int4or` struct representing the `|` operator.
    fn int4or<Rhs>(self, rhs: Rhs) -> Int4or<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Integer>,
    {
        Int4or::new(self, rhs.as_expression())
    }
}

impl<T> HasInt4or for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Integer>
{
}
