//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(Oideq, " = ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `=` operator.
pub trait HasOideq:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Oid>
{
    /// The function to create the `Oideq` struct representing the `=` operator.
    fn oideq<Rhs>(self, rhs: Rhs) -> Oideq<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Oid>,
    {
        Oideq::new(self, rhs.as_expression())
    }
}

impl<T> HasOideq for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Oid>
{
}
