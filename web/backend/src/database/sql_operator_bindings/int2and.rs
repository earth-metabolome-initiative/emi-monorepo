//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(Int2and, " & ", diesel::sql_types::SmallInt, backend: diesel::pg::Pg);
/// Trait for the `&` operator.
pub trait HasInt2and:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::SmallInt>
{
    /// The function to create the `Int2and` struct representing the `&` operator.
    fn int2and<Rhs>(self, rhs: Rhs) -> Int2and<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::SmallInt>,
    {
        Int2and::new(self, rhs.as_expression())
    }
}

impl<T> HasInt2and for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::SmallInt>
{
}