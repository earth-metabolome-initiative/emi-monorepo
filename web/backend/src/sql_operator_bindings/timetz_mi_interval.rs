//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(TimetzMiInterval, " - ", diesel::sql_types::Time, backend: diesel::pg::Pg);
/// Trait for the `-` operator.
pub trait HasTimetzMiInterval:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Time>
{
    /// The function to create the `TimetzMiInterval` struct representing the `-` operator.
    fn timetz_mi_interval<Rhs>(self, rhs: Rhs) -> TimetzMiInterval<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Interval>,
    {
        TimetzMiInterval::new(self, rhs.as_expression())
    }
}

impl<T> HasTimetzMiInterval for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Time>
{
}
