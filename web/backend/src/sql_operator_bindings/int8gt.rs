//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(Int8gt, " > ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `>` operator.
pub trait HasInt8gt:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::BigInt>
{
    /// The function to create the `Int8gt` struct representing the `>` operator.
    fn int8gt<Rhs>(self, rhs: Rhs) -> Int8gt<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::BigInt>,
    {
        Int8gt::new(self, rhs.as_expression())
    }
}

impl<T> HasInt8gt for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::BigInt>
{
}