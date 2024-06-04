//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(TextLe, " <= ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `<=` operator.
pub trait HasTextLe:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Text>
{
    /// The function to create the `TextLe` struct representing the `<=` operator.
    fn text_le<Rhs>(self, rhs: Rhs) -> TextLe<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Text>,
    {
        TextLe::new(self, rhs.as_expression())
    }
}

impl<T> HasTextLe for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Text>
{
}
