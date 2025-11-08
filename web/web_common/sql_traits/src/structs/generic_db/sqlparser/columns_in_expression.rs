//! Functions to extract columns from SQL expressions.

use std::rc::Rc;

use sqlparser::ast::Expr;

use crate::traits::{DatabaseLike, column::ColumnLike};

pub(super) fn columns_in_expression<DB: DatabaseLike>(
    expr: &Expr,
    columns: &[Rc<DB::Column>],
) -> Vec<Rc<DB::Column>> {
    let mut result = Vec::new();

    match expr {
        Expr::Identifier(ident) => {
            result.extend(
                columns.iter().filter(|col| col.column_name() == ident.value.as_str()).cloned(),
            );
        }
        Expr::CompoundIdentifier(idents) => {
            if let Some(last_ident) = idents.last() {
                result.extend(
                    columns
                        .iter()
                        .filter(|col| col.column_name() == last_ident.value.as_str())
                        .cloned(),
                );
            }
        }
        Expr::BinaryOp { left, right, .. } => {
            result.extend(columns_in_expression::<DB>(left, columns));
            result.extend(columns_in_expression::<DB>(right, columns));
        }
        Expr::Nested(nested_expr) => {
            result.extend(columns_in_expression::<DB>(nested_expr, columns));
        }
        Expr::Between { expr, negated: _, low, high } => {
            result.extend(columns_in_expression::<DB>(expr, columns));
            result.extend(columns_in_expression::<DB>(low, columns));
            result.extend(columns_in_expression::<DB>(high, columns));
        }
        Expr::UnaryOp { expr, .. } => {
            result.extend(columns_in_expression::<DB>(expr, columns));
        }
        Expr::Cast { expr, .. } => {
            result.extend(columns_in_expression::<DB>(expr, columns));
        }
        Expr::Function(func) => {
            match &func.args {
                sqlparser::ast::FunctionArguments::List(args) => {
                    for arg in args.args.iter() {
                        match arg {
                            sqlparser::ast::FunctionArg::Named { arg, .. } => {
                                match arg {
                                    sqlparser::ast::FunctionArgExpr::Expr(expr) => {
                                        result.extend(columns_in_expression::<DB>(expr, columns));
                                    }
                                    _ => {}
                                }
                            }
                            sqlparser::ast::FunctionArg::Unnamed(func_arg_expr) => {
                                match func_arg_expr {
                                    sqlparser::ast::FunctionArgExpr::Expr(expr) => {
                                        result.extend(columns_in_expression::<DB>(expr, columns));
                                    }
                                    sqlparser::ast::FunctionArgExpr::Wildcard => {}
                                    sqlparser::ast::FunctionArgExpr::QualifiedWildcard(_) => {}
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        Expr::Case { .. } => {
            // Case expressions can be complex - for now, skip detailed analysis
            // This would require proper handling of operand, conditions,
            // results, and else_result
        }
        Expr::InList { expr, list, .. } => {
            result.extend(columns_in_expression::<DB>(expr, columns));
            for list_expr in list {
                result.extend(columns_in_expression::<DB>(list_expr, columns));
            }
        }
        Expr::InSubquery { expr, .. } => {
            result.extend(columns_in_expression::<DB>(expr, columns));
            // Note: We don't traverse into subqueries as they have their own
            // column scope
        }
        Expr::IsNull(expr) | Expr::IsNotNull(expr) => {
            result.extend(columns_in_expression::<DB>(expr, columns));
        }
        Expr::Exists { .. } => {
            // EXISTS subqueries have their own column scope, so we don't
            // traverse into them
        }
        Expr::Subquery(_) => {
            // Subqueries have their own scope, so we don't traverse into them
        }
        // Literal values don't contain column references
        Expr::Value(_)
        | Expr::TypedString { .. }
        | Expr::Wildcard(_)
        | Expr::QualifiedWildcard(_, _) => {}

        // For any other expression types, we do nothing (safe default)
        _ => {}
    }

    // Remove duplicates while preserving order
    let mut seen = std::collections::HashSet::new();
    result
        .into_iter()
        .filter(|col| {
            let ptr = Rc::as_ptr(col) as *const ();
            seen.insert(ptr)
        })
        .collect()
}
