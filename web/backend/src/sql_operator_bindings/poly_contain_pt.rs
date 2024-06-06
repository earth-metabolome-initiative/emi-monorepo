//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(PolyContainPt, " @> ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `@>` operator.
pub trait HasPolyContainPt:
    Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
    /// The function to create the `PolyContainPt` struct representing the `@>` operator.
    fn poly_contain_pt<Rhs>(self, rhs: Rhs) -> PolyContainPt<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<postgis_diesel::sql_types::Geometry>,
    {
        PolyContainPt::new(self, rhs.as_expression())
    }
}

impl<T> HasPolyContainPt for T where
    T: Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
}