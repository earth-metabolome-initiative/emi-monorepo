//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(GeometryDistanceCpa, " |=| ", diesel::sql_types::Double, backend: diesel::pg::Pg);
/// Trait for the `|=|` operator.
pub trait HasGeometryDistanceCpa:
    Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
    /// The function to create the `GeometryDistanceCpa` struct representing the `|=|` operator.
    fn geometry_distance_cpa<Rhs>(self, rhs: Rhs) -> GeometryDistanceCpa<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<postgis_diesel::sql_types::Geometry>,
    {
        GeometryDistanceCpa::new(self, rhs.as_expression())
    }
}

impl<T> HasGeometryDistanceCpa for T where
    T: Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
}