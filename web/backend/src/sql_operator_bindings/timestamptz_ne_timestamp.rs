//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(TimestamptzNeTimestamp, " <> ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `<>` operator.
pub trait HasTimestamptzNeTimestamp:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Timestamp>
{
    /// The function to create the `TimestamptzNeTimestamp` struct representing the `<>` operator.
    fn timestamptz_ne_timestamp<Rhs>(
        self,
        rhs: Rhs,
    ) -> TimestamptzNeTimestamp<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Timestamp>,
    {
        TimestamptzNeTimestamp::new(self, rhs.as_expression())
    }
}

impl<T> HasTimestamptzNeTimestamp for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Timestamp>
{
}