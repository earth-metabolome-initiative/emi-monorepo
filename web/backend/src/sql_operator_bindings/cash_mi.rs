//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(CashMi, " - ", diesel::pg::sql_types::Money, backend: diesel::pg::Pg);
/// Trait for the `-` operator.
pub trait HasCashMi:
    Sized + diesel::expression::Expression<SqlType = diesel::pg::sql_types::Money>
{
    /// The function to create the `CashMi` struct representing the `-` operator.
    fn cash_mi<Rhs>(self, rhs: Rhs) -> CashMi<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::pg::sql_types::Money>,
    {
        CashMi::new(self, rhs.as_expression())
    }
}

impl<T> HasCashMi for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::pg::sql_types::Money>
{
}
