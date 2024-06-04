//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(TsqueryEq, " = ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `=` operator.
pub trait HasTsqueryEq:
    Sized + diesel::expression::Expression<SqlType = diesel_full_text_search::TsQuery>
{
    /// The function to create the `TsqueryEq` struct representing the `=` operator.
    fn tsquery_eq<Rhs>(self, rhs: Rhs) -> TsqueryEq<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel_full_text_search::TsQuery>,
    {
        TsqueryEq::new(self, rhs.as_expression())
    }
}

impl<T> HasTsqueryEq for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel_full_text_search::TsQuery>
{
}
