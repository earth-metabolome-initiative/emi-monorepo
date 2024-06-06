//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(TimetzGt, " > ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `>` operator.
pub trait HasTimetzGt:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Time>
{
    /// The function to create the `TimetzGt` struct representing the `>` operator.
    fn timetz_gt<Rhs>(self, rhs: Rhs) -> TimetzGt<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Time>,
    {
        TimetzGt::new(self, rhs.as_expression())
    }
}

impl<T> HasTimetzGt for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Time>
{
}