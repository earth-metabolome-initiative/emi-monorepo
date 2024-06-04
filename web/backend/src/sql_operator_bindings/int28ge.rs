//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(Int28ge, " >= ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `>=` operator.
pub trait HasInt28ge:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::SmallInt>
{
    /// The function to create the `Int28ge` struct representing the `>=` operator.
    fn int28ge<Rhs>(self, rhs: Rhs) -> Int28ge<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::BigInt>,
    {
        Int28ge::new(self, rhs.as_expression())
    }
}

impl<T> HasInt28ge for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::SmallInt>
{
}
