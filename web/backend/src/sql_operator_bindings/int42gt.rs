//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(Int42gt, " > ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `>` operator.
pub trait HasInt42gt:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Integer>
{
    /// The function to create the `Int42gt` struct representing the `>` operator.
    fn int42gt<Rhs>(self, rhs: Rhs) -> Int42gt<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::SmallInt>,
    {
        Int42gt::new(self, rhs.as_expression())
    }
}

impl<T> HasInt42gt for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Integer>
{
}