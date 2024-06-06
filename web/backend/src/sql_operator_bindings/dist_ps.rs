//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(DistPs, " <-> ", diesel::sql_types::Double, backend: diesel::pg::Pg);
/// Trait for the `<->` operator.
pub trait HasDistPs:
    Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
    /// The function to create the `DistPs` struct representing the `<->` operator.
    fn dist_ps<Rhs>(self, rhs: Rhs) -> DistPs<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<postgis_diesel::sql_types::Geometry>,
    {
        DistPs::new(self, rhs.as_expression())
    }
}

impl<T> HasDistPs for T where
    T: Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
}