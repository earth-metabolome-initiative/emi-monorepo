//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(DateLeTimestamp, " <= ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `<=` operator.
pub trait HasDateLeTimestamp:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Date>
{
    /// The function to create the `DateLeTimestamp` struct representing the `<=` operator.
    fn date_le_timestamp<Rhs>(self, rhs: Rhs) -> DateLeTimestamp<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Timestamp>,
    {
        DateLeTimestamp::new(self, rhs.as_expression())
    }
}

impl<T> HasDateLeTimestamp for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Date>
{
}
