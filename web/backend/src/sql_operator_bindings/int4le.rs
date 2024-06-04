//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(Int4le, " <= ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `<=` operator.
pub trait HasInt4le:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Integer>
{
    /// The function to create the `Int4le` struct representing the `<=` operator.
    fn int4le<Rhs>(self, rhs: Rhs) -> Int4le<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Integer>,
    {
        Int4le::new(self, rhs.as_expression())
    }
}

impl<T> HasInt4le for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Integer>
{
}
