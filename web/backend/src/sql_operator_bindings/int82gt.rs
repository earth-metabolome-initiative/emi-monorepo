//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(Int82gt, " > ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `>` operator.
pub trait HasInt82gt:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::BigInt>
{
    /// The function to create the `Int82gt` struct representing the `>` operator.
    fn int82gt<Rhs>(self, rhs: Rhs) -> Int82gt<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::SmallInt>,
    {
        Int82gt::new(self, rhs.as_expression())
    }
}

impl<T> HasInt82gt for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::BigInt>
{
}