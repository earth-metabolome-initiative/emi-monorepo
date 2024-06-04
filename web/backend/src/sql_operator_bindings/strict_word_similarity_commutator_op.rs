//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(StrictWordSimilarityCommutatorOp, " %>> ", diesel::sql_types::Bool, backend: diesel::pg::Pg);
/// Trait for the `%>>` operator.
pub trait HasStrictWordSimilarityCommutatorOp:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Text>
{
    /// The function to create the `StrictWordSimilarityCommutatorOp` struct representing the `%>>` operator.
    fn strict_word_similarity_commutator_op<Rhs>(
        self,
        rhs: Rhs,
    ) -> StrictWordSimilarityCommutatorOp<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Text>,
    {
        StrictWordSimilarityCommutatorOp::new(self, rhs.as_expression())
    }
}

impl<T> HasStrictWordSimilarityCommutatorOp for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Text>
{
}
