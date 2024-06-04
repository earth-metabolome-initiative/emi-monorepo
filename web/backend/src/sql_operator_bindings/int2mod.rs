//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(Int2mod, " % ", diesel::sql_types::SmallInt, backend: diesel::pg::Pg);
/// Trait for the `%` operator.
pub trait HasInt2mod:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::SmallInt>
{
    /// The function to create the `Int2mod` struct representing the `%` operator.
    fn int2mod<Rhs>(self, rhs: Rhs) -> Int2mod<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::SmallInt>,
    {
        Int2mod::new(self, rhs.as_expression())
    }
}

impl<T> HasInt2mod for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::SmallInt>
{
}
