//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(Int84mul, " * ", diesel::sql_types::BigInt, backend: diesel::pg::Pg);
/// Trait for the `*` operator.
pub trait HasInt84mul:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::BigInt>
{
    /// The function to create the `Int84mul` struct representing the `*` operator.
    fn int84mul<Rhs>(self, rhs: Rhs) -> Int84mul<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Integer>,
    {
        Int84mul::new(self, rhs.as_expression())
    }
}

impl<T> HasInt84mul for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::BigInt>
{
}
