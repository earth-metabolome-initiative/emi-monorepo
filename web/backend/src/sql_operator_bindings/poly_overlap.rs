//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(PolyOverlap, " && ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `&&` operator.
pub trait HasPolyOverlap:
    Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
    /// The function to create the `PolyOverlap` struct representing the `&&` operator.
    fn poly_overlap<Rhs>(self, rhs: Rhs) -> PolyOverlap<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<postgis_diesel::sql_types::Geometry>,
    {
        PolyOverlap::new(self, rhs.as_expression())
    }
}

impl<T> HasPolyOverlap for T where
    T: Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
}
