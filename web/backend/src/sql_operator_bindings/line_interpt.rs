//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(LineInterpt, " # ", postgis_diesel::sql_types::Geometry, backend: diesel::pg::Pg);
/// Trait for the `#` operator.
pub trait HasLineInterpt:
    Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
    /// The function to create the `LineInterpt` struct representing the `#` operator.
    fn line_interpt<Rhs>(self, rhs: Rhs) -> LineInterpt<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<postgis_diesel::sql_types::Geometry>,
    {
        LineInterpt::new(self, rhs.as_expression())
    }
}

impl<T> HasLineInterpt for T where
    T: Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
}