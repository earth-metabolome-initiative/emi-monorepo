//! Splits a check expression into the individual top-level OR-separated
//! sub-expressions.

use sqlparser::ast::{BinaryOperator, Expr};

/// Splits the provided expression into its top-level OR-separated
/// sub-expressions.
///
/// # Arguments
/// * `expr` - The expression to split.
pub(super) fn sub_expressions(expr: &Expr) -> Vec<&Expr> {
    match expr {
        Expr::BinaryOp { left, op: BinaryOperator::Or, right } => {
            let mut expressions = sub_expressions(left);
            expressions.extend(sub_expressions(right));
            expressions
        }
        Expr::Nested(expr) => sub_expressions(expr),
        _ => vec![expr],
    }
}
