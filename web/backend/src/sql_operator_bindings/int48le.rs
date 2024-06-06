//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(Int48le, " <= ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `<=` operator.
pub trait HasInt48le:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Integer>
{
    /// The function to create the `Int48le` struct representing the `<=` operator.
    fn int48le<Rhs>(self, rhs: Rhs) -> Int48le<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::BigInt>,
    {
        Int48le::new(self, rhs.as_expression())
    }
}

impl<T> HasInt48le for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Integer>
{
}