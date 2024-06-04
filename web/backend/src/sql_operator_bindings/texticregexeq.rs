//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(Texticregexeq, " ~* ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `~*` operator.
pub trait HasTexticregexeq:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Text>
{
    /// The function to create the `Texticregexeq` struct representing the `~*` operator.
    fn texticregexeq<Rhs>(self, rhs: Rhs) -> Texticregexeq<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Text>,
    {
        Texticregexeq::new(self, rhs.as_expression())
    }
}

impl<T> HasTexticregexeq for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Text>
{
}
