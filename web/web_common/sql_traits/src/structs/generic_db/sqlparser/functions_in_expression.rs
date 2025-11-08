//! Functions to extract functions from SQL expressions.

use std::rc::Rc;

use sqlparser::ast::Expr;

use crate::traits::{DatabaseLike, function_like::FunctionLike};

pub(super) fn functions_in_expression<DB: DatabaseLike>(
    expr: &Expr,
    functions: &[Rc<DB::Function>],
) -> Vec<Rc<DB::Function>> {
    let mut result = Vec::new();

    match expr {
        Expr::Function(func) => {
            // Extract the function name from the ObjectName
            let function_name = func.name.to_string();

            // Find matching functions by name
            result.extend(functions.iter().filter(|f| f.name() == function_name.as_str()).cloned());

            // Recursively check function arguments for nested function calls
            match &func.args {
                sqlparser::ast::FunctionArguments::List(args) => {
                    for arg in args.args.iter() {
                        match arg {
                            sqlparser::ast::FunctionArg::Named { arg, .. } => {
                                match arg {
                                    sqlparser::ast::FunctionArgExpr::Expr(expr) => {
                                        result
                                            .extend(functions_in_expression::<DB>(expr, functions));
                                    }
                                    _ => {}
                                }
                            }
                            sqlparser::ast::FunctionArg::Unnamed(func_arg_expr) => {
                                match func_arg_expr {
                                    sqlparser::ast::FunctionArgExpr::Expr(expr) => {
                                        result
                                            .extend(functions_in_expression::<DB>(expr, functions));
                                    }
                                    sqlparser::ast::FunctionArgExpr::Wildcard
                                    | sqlparser::ast::FunctionArgExpr::QualifiedWildcard(_) => {}
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        Expr::BinaryOp { left, right, .. } => {
            result.extend(functions_in_expression::<DB>(left, functions));
            result.extend(functions_in_expression::<DB>(right, functions));
        }
        Expr::Nested(nested_expr) => {
            result.extend(functions_in_expression::<DB>(nested_expr, functions));
        }
        Expr::Between { expr, negated: _, low, high } => {
            result.extend(functions_in_expression::<DB>(expr, functions));
            result.extend(functions_in_expression::<DB>(low, functions));
            result.extend(functions_in_expression::<DB>(high, functions));
        }
        Expr::UnaryOp { expr, .. } => {
            result.extend(functions_in_expression::<DB>(expr, functions));
        }
        Expr::Cast { expr, .. } => {
            result.extend(functions_in_expression::<DB>(expr, functions));
        }
        Expr::Case { .. } => {
            // Case expressions can be complex - for now, skip detailed analysis
            // This would require proper handling of operand, conditions,
            // results, and else_result
        }
        Expr::InList { expr, list, .. } => {
            result.extend(functions_in_expression::<DB>(expr, functions));
            for list_expr in list {
                result.extend(functions_in_expression::<DB>(list_expr, functions));
            }
        }
        Expr::InSubquery { expr, .. } => {
            result.extend(functions_in_expression::<DB>(expr, functions));
            // Note: We don't traverse into subqueries as they have their own
            // scope
        }
        Expr::IsNull(expr) | Expr::IsNotNull(expr) => {
            result.extend(functions_in_expression::<DB>(expr, functions));
        }
        Expr::Exists { .. } => {
            // EXISTS subqueries have their own scope, so we don't traverse into
            // them
        }
        Expr::Subquery(_) => {
            // Subqueries have their own scope, so we don't traverse into them
        }
        // Literal values and identifiers don't contain function calls
        Expr::Value(_)
        | Expr::TypedString { .. }
        | Expr::Identifier(_)
        | Expr::CompoundIdentifier(_)
        | Expr::Wildcard(_)
        | Expr::QualifiedWildcard(_, _) => {}

        // For any other expression types, we do nothing (safe default)
        _ => {}
    }

    // Remove duplicates while preserving order
    let mut seen = std::collections::HashSet::new();
    result
        .into_iter()
        .filter(|func| {
            let ptr = Rc::as_ptr(func) as *const ();
            seen.insert(ptr)
        })
        .collect()
}
