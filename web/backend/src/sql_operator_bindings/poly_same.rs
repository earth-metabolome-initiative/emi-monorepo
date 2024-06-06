//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(PolySame, " ~= ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `~=` operator.
pub trait HasPolySame:
    Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
    /// The function to create the `PolySame` struct representing the `~=` operator.
    fn poly_same<Rhs>(self, rhs: Rhs) -> PolySame<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<postgis_diesel::sql_types::Geometry>,
    {
        PolySame::new(self, rhs.as_expression())
    }
}

impl<T> HasPolySame for T where
    T: Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
}