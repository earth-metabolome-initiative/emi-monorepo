//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(GeographyEq, " = ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `=` operator.
pub trait HasGeographyEq:
    Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geography>
{
    /// The function to create the `GeographyEq` struct representing the `=` operator.
    fn geography_eq<Rhs>(self, rhs: Rhs) -> GeographyEq<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<postgis_diesel::sql_types::Geography>,
    {
        GeographyEq::new(self, rhs.as_expression())
    }
}

impl<T> HasGeographyEq for T where
    T: Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geography>
{
}