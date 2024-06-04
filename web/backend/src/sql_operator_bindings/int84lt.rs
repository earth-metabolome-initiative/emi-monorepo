//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(Int84lt, " < ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `<` operator.
pub trait HasInt84lt:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::BigInt>
{
    /// The function to create the `Int84lt` struct representing the `<` operator.
    fn int84lt<Rhs>(self, rhs: Rhs) -> Int84lt<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Integer>,
    {
        Int84lt::new(self, rhs.as_expression())
    }
}

impl<T> HasInt84lt for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::BigInt>
{
}
