//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(DateEqTimestamptz, " = ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `=` operator.
pub trait HasDateEqTimestamptz:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Date>
{
    /// The function to create the `DateEqTimestamptz` struct representing the `=` operator.
    fn date_eq_timestamptz<Rhs>(self, rhs: Rhs) -> DateEqTimestamptz<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Timestamp>,
    {
        DateEqTimestamptz::new(self, rhs.as_expression())
    }
}

impl<T> HasDateEqTimestamptz for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Date>
{
}
