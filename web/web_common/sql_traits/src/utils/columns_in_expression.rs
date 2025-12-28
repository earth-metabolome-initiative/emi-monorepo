//! Functions to extract columns from SQL expressions.

use std::rc::Rc;

use sqlparser::ast::Expr;

use crate::traits::{DatabaseLike, column::ColumnLike};

/// Extracts columns from a SQL expression.
///
/// # Arguments
///
/// * `expr` - The SQL expression to extract columns from.
/// * `table_name` - The name of the table the expression belongs to.
/// * `columns` - The list of columns available in the table.
///
/// # Returns
///
/// * A vector of columns found in the expression.
///
/// # Errors
///
/// * If a column in the expression is not found in the provided list of
///   columns.
pub fn columns_in_expression<DB: DatabaseLike>(
    expr: &Expr,
    table_name: &str,
    columns: &[Rc<DB::Column>],
) -> Result<Vec<Rc<DB::Column>>, crate::errors::Error> {
    let mut result = Vec::new();

    match expr {
        Expr::Identifier(ident) => {
            if let Some(col) = columns.iter().find(|col| col.column_name() == ident.value.as_str())
            {
                result.push(col.clone());
            } else {
                return Err(crate::errors::Error::UnknownColumnInCheckConstraint {
                    column_name: ident.value.clone(),
                    table_name: table_name.to_string(),
                });
            }
        }
        Expr::CompoundIdentifier(idents) => {
            if let Some(last_ident) = idents.last() {
                if let Some(col) =
                    columns.iter().find(|col| col.column_name() == last_ident.value.as_str())
                {
                    result.push(col.clone());
                } else {
                    return Err(crate::errors::Error::UnknownColumnInCheckConstraint {
                        column_name: last_ident.value.clone(),
                        table_name: table_name.to_string(),
                    });
                }
            }
        }
        Expr::BinaryOp { left, right, .. } => {
            result.extend(columns_in_expression::<DB>(left, table_name, columns)?);
            result.extend(columns_in_expression::<DB>(right, table_name, columns)?);
        }
        Expr::Nested(nested_expr) => {
            result.extend(columns_in_expression::<DB>(nested_expr, table_name, columns)?);
        }
        Expr::Between { expr, negated: _, low, high } => {
            result.extend(columns_in_expression::<DB>(expr, table_name, columns)?);
            result.extend(columns_in_expression::<DB>(low, table_name, columns)?);
            result.extend(columns_in_expression::<DB>(high, table_name, columns)?);
        }
        Expr::UnaryOp { expr, .. }
        | Expr::Cast { expr, .. }
        | Expr::IsNull(expr)
        | Expr::IsNotNull(expr) => {
            result.extend(columns_in_expression::<DB>(expr, table_name, columns)?);
        }
        Expr::Function(func) => {
            if let sqlparser::ast::FunctionArguments::List(args) = &func.args {
                for arg in &args.args {
                    match arg {
                        sqlparser::ast::FunctionArg::Named {
                            arg: sqlparser::ast::FunctionArgExpr::Expr(expr),
                            ..
                        }
                        | sqlparser::ast::FunctionArg::Unnamed(
                            sqlparser::ast::FunctionArgExpr::Expr(expr),
                        ) => {
                            result.extend(columns_in_expression::<DB>(expr, table_name, columns)?);
                        }
                        sqlparser::ast::FunctionArg::ExprNamed { .. }
                        | sqlparser::ast::FunctionArg::Named { .. }
                        | sqlparser::ast::FunctionArg::Unnamed(_) => {}
                    }
                }
            }
        }
        Expr::InList { expr, list, .. } => {
            result.extend(columns_in_expression::<DB>(expr, table_name, columns)?);
            for list_expr in list {
                result.extend(columns_in_expression::<DB>(list_expr, table_name, columns)?);
            }
        }
        Expr::InSubquery { expr, .. } => {
            result.extend(columns_in_expression::<DB>(expr, table_name, columns)?);
            // Note: We don't traverse into subqueries as they have their own
            // column scope
        }
        _ => {}
    }

    // Remove duplicates while preserving order
    let mut seen = std::collections::HashSet::new();
    Ok(result
        .into_iter()
        .filter(|col| {
            let ptr = Rc::as_ptr(col).cast::<()>();
            seen.insert(ptr)
        })
        .collect())
}
