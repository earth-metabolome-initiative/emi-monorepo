//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(InterSl, " ?# ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `?#` operator.
pub trait HasInterSl:
    Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
    /// The function to create the `InterSl` struct representing the `?#` operator.
    fn inter_sl<Rhs>(self, rhs: Rhs) -> InterSl<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<postgis_diesel::sql_types::Geometry>,
    {
        InterSl::new(self, rhs.as_expression())
    }
}

impl<T> HasInterSl for T where
    T: Sized + diesel::expression::Expression<SqlType = postgis_diesel::sql_types::Geometry>
{
}