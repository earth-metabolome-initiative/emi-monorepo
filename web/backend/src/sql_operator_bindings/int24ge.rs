//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(Int24ge, " >= ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `>=` operator.
pub trait HasInt24ge:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::SmallInt>
{
    /// The function to create the `Int24ge` struct representing the `>=` operator.
    fn int24ge<Rhs>(self, rhs: Rhs) -> Int24ge<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Integer>,
    {
        Int24ge::new(self, rhs.as_expression())
    }
}

impl<T> HasInt24ge for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::SmallInt>
{
}
