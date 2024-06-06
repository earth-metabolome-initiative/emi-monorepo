//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(Int42ne, " <> ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `<>` operator.
pub trait HasInt42ne:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Integer>
{
    /// The function to create the `Int42ne` struct representing the `<>` operator.
    fn int42ne<Rhs>(self, rhs: Rhs) -> Int42ne<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::SmallInt>,
    {
        Int42ne::new(self, rhs.as_expression())
    }
}

impl<T> HasInt42ne for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Integer>
{
}