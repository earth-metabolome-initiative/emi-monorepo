//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(JsonArrayElementText, " ->> ", diesel::sql_types::Text, backend: diesel::pg::Pg);
/// Trait for the `->>` operator.
pub trait HasJsonArrayElementText:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Json>
{
    /// The function to create the `JsonArrayElementText` struct representing the `->>` operator.
    fn json_array_element_text<Rhs>(self, rhs: Rhs) -> JsonArrayElementText<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Integer>,
    {
        JsonArrayElementText::new(self, rhs.as_expression())
    }
}

impl<T> HasJsonArrayElementText for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Json>
{
}
