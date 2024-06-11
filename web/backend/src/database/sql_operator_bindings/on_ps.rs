//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(OnPs, " <@ ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `<@` operator.
pub trait HasOnPs:
    Sized + diesel::expression::Expression<SqlType = crate::database::sql_type_bindings::Point>
{
    /// The function to create the `OnPs` struct representing the `<@` operator.
    fn on_ps<Rhs>(self, rhs: Rhs) -> OnPs<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<postgis_diesel::sql_types::Geometry>,
    {
        OnPs::new(self, rhs.as_expression())
    }
}

impl<T> HasOnPs for T where
    T: Sized + diesel::expression::Expression<SqlType = crate::database::sql_type_bindings::Point>
{
}
