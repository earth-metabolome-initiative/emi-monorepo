//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(CashDivInt2, " / ", diesel::pg::sql_types::Money, backend: diesel::pg::Pg);
/// Trait for the `/` operator.
pub trait HasCashDivInt2:
    Sized + diesel::expression::Expression<SqlType = diesel::pg::sql_types::Money>
{
    /// The function to create the `CashDivInt2` struct representing the `/` operator.
    fn cash_div_int2<Rhs>(self, rhs: Rhs) -> CashDivInt2<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::SmallInt>,
    {
        CashDivInt2::new(self, rhs.as_expression())
    }
}

impl<T> HasCashDivInt2 for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::pg::sql_types::Money>
{
}