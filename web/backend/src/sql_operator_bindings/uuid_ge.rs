//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(UuidGe, " >= ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `>=` operator.
pub trait HasUuidGe:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Uuid>
{
    /// The function to create the `UuidGe` struct representing the `>=` operator.
    fn uuid_ge<Rhs>(self, rhs: Rhs) -> UuidGe<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Uuid>,
    {
        UuidGe::new(self, rhs.as_expression())
    }
}

impl<T> HasUuidGe for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Uuid>
{
}