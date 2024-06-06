//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(JsonExtractPath, " #> ", diesel::sql_types::Json, backend: diesel::pg::Pg);
/// Trait for the `#>` operator.
pub trait HasJsonExtractPath:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Json>
{
    /// The function to create the `JsonExtractPath` struct representing the `#>` operator.
    fn json_extract_path<Rhs>(self, rhs: Rhs) -> JsonExtractPath<Self, Rhs::Expression>
    where
        Rhs:
            diesel::expression::AsExpression<diesel::pg::sql_types::Array<diesel::sql_types::Text>>,
    {
        JsonExtractPath::new(self, rhs.as_expression())
    }
}

impl<T> HasJsonExtractPath for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Json>
{
}