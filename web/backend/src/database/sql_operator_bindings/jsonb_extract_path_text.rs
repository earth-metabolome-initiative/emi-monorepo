//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(JsonbExtractPathText, " #>> ", diesel::sql_types::Text, backend: diesel::pg::Pg);
/// Trait for the `#>>` operator.
pub trait HasJsonbExtractPathText:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Jsonb>
{
    /// The function to create the `JsonbExtractPathText` struct representing the `#>>` operator.
    fn jsonb_extract_path_text<Rhs>(self, rhs: Rhs) -> JsonbExtractPathText<Self, Rhs::Expression>
    where
        Rhs:
            diesel::expression::AsExpression<diesel::pg::sql_types::Array<diesel::sql_types::Text>>,
    {
        JsonbExtractPathText::new(self, rhs.as_expression())
    }
}

impl<T> HasJsonbExtractPathText for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Jsonb>
{
}