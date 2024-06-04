//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(Int8lt, " < ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `<` operator.
pub trait HasInt8lt:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::BigInt>
{
    /// The function to create the `Int8lt` struct representing the `<` operator.
    fn int8lt<Rhs>(self, rhs: Rhs) -> Int8lt<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::BigInt>,
    {
        Int8lt::new(self, rhs.as_expression())
    }
}

impl<T> HasInt8lt for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::BigInt>
{
}
