//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(PtContainedPoly, " <@ ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `<@` operator.
pub trait HasPtContainedPoly:
    Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
    /// The function to create the `PtContainedPoly` struct representing the `<@` operator.
    fn pt_contained_poly<Rhs>(self, rhs: Rhs) -> PtContainedPoly<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<postgis_diesel::sql_types::Geometry>,
    {
        PtContainedPoly::new(self, rhs.as_expression())
    }
}

impl<T> HasPtContainedPoly for T where
    T: Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
}
