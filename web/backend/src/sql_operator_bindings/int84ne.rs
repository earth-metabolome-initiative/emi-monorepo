//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(Int84ne, " <> ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `<>` operator.
pub trait HasInt84ne:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::BigInt>
{
    /// The function to create the `Int84ne` struct representing the `<>` operator.
    fn int84ne<Rhs>(self, rhs: Rhs) -> Int84ne<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Integer>,
    {
        Int84ne::new(self, rhs.as_expression())
    }
}

impl<T> HasInt84ne for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::BigInt>
{
}