//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(TimestamptzGeTimestamp, " >= ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `>=` operator.
pub trait HasTimestamptzGeTimestamp:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Timestamp>
{
    /// The function to create the `TimestamptzGeTimestamp` struct representing the `>=` operator.
    fn timestamptz_ge_timestamp<Rhs>(
        self,
        rhs: Rhs,
    ) -> TimestamptzGeTimestamp<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Timestamp>,
    {
        TimestamptzGeTimestamp::new(self, rhs.as_expression())
    }
}

impl<T> HasTimestamptzGeTimestamp for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Timestamp>
{
}
