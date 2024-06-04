//! This file is automatically generated by the code generation suite.
//! Do not edit it manually.
//!
//! This file contains the bindings for the SQL operators in the database.

diesel::infix_operator!(WordSimilarityDistOp, " <<-> ", diesel::sql_types::Float, backend: diesel::pg::Pg);
/// Trait for the `<<->` operator.
pub trait HasWordSimilarityDistOp:
    Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Text>
{
    /// The function to create the `WordSimilarityDistOp` struct representing the `<<->` operator.
    fn word_similarity_dist_op<Rhs>(self, rhs: Rhs) -> WordSimilarityDistOp<Self, Rhs::Expression>
    where
        Rhs: diesel::expression::AsExpression<diesel::sql_types::Text>,
    {
        WordSimilarityDistOp::new(self, rhs.as_expression())
    }
}

impl<T> HasWordSimilarityDistOp for T where
    T: Sized + diesel::expression::Expression<SqlType = diesel::sql_types::Text>
{
}
