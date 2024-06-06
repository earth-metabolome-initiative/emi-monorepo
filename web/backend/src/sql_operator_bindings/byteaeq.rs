//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(Byteaeq, " = ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `=` operator.
pub trait HasByteaeq:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Binary>
{
    /// The function to create the `Byteaeq` struct representing the `=` operator.
    fn byteaeq<Rhs>(self, rhs: Rhs) -> Byteaeq<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Binary>,
    {
        Byteaeq::new(self, rhs.as_expression())
    }
}

impl<T> HasByteaeq for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Binary>
{
}