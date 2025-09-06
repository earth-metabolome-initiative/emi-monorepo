use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

use cached::{DiskCache, proc_macro::io_cached};
use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, Queryable,
    QueryableByName, Selectable, SelectableHelper,
};
use proc_macro2::TokenStream;
use sqlparser::{
    ast::{
        BinaryOperator, Expr, FunctionArg, FunctionArgExpr, FunctionArgumentList, FunctionArguments,
    },
    dialect::PostgreSqlDialect,
    parser::Parser,
};

use super::{Column, PgConstraint, PgExtension, PgOperator, PgProc, PgType};
use crate::{
    Table, TableConstraint,
    errors::{CheckConstraintError, UnsupportedCheckConstraintErrorSyntax, WebCodeGenError},
};

#[io_cached(
    map_error = r##"|e| WebCodeGenError::from(e)"##,
    disk = true,
    sync_to_disk_on_cache_change = true,
    create = r##" {
        DiskCache::new("check_constraints.columns")
            .set_disk_directory("cache")
            .build()
            .expect("error building disk cache")
    } "##,
    key = "CheckConstraint",
    convert = r#"{check_constraint.clone()}"#
)]
fn columns(
    check_constraint: &CheckConstraint,
    conn: &mut PgConnection,
) -> Result<Arc<Vec<Column>>, WebCodeGenError> {
    use diesel::RunQueryDsl;

    use crate::schema::{columns, constraint_column_usage};
    Ok(Arc::new(
        columns::table
            .inner_join(
                constraint_column_usage::table.on(constraint_column_usage::constraint_name
                    .eq(&check_constraint.constraint_name)
                    .and(
                        constraint_column_usage::constraint_catalog
                            .eq(&check_constraint.constraint_catalog)
                            .and(
                                constraint_column_usage::constraint_schema
                                    .eq(&check_constraint.constraint_schema),
                            ),
                    )),
            )
            .filter(
                constraint_column_usage::column_name.eq(&columns::column_name).and(
                    constraint_column_usage::table_catalog.eq(&columns::table_catalog).and(
                        constraint_column_usage::table_schema
                            .eq(&columns::table_schema)
                            .and(constraint_column_usage::table_name.eq(&columns::table_name)),
                    ),
                ),
            )
            .select(Column::as_select())
            .load(conn)?,
    ))
}

#[derive(
    Queryable,
    QueryableByName,
    Debug,
    Clone,
    Selectable,
    Ord,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
)]
#[diesel(table_name = crate::schema::check_constraints)]
/// A struct representing a check constraint
pub struct CheckConstraint {
    /// The name of the constraint catalog
    pub constraint_catalog: String,
    /// The name of the constraint schema
    pub constraint_schema: String,
    /// The name of the constraint
    pub constraint_name: String,
    /// The check clause of the constraint
    pub check_clause: String,
}

impl Display for CheckConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.constraint_catalog, self.constraint_schema, self.constraint_name)
    }
}

const POSTGIS_CONSTRAINTS: [&str; 1] = ["spatial_ref_sys_srid_check"];

struct TranslateExpression<'a, C1: Debug, C2: Debug> {
    check_constraint: &'a CheckConstraint,
    contextual_columns: &'a [C1],
    self_columns: &'a [C2],
    involved_columns: &'a [Column],
    functions: &'a [PgProc],
    attributes_enumeration: &'a syn::Type,
}

#[derive(Debug, Clone, PartialEq)]
/// The type returned by an expression
pub enum ReturningType {
    /// Empty tuple, i.e., `()`
    Unit,
    /// When the expression returns a `Result`
    Result(Box<ReturningType>),
    /// When the expression returns a `Boolean`
    Boolean,
    /// When the expression returns a `Textual`
    Textual,
    /// When the expression returns a `U16`
    U16,
    /// When the expression returns a `U32`
    U32,
    /// When the expression returns a `U64`
    U64,
    /// When the expression returns a `I16`
    I16,
    /// When the expression returns a `I32`
    I32,
    /// When the expression returns a `I64`
    I64,
    /// When the expression returns a `F32`
    F32,
    /// When the expression returns a `F64`
    F64,
    /// When the expression returns a `Custom` type
    Custom(Box<PgType>),
}

impl ReturningType {
    fn is_unit_result(&self) -> bool {
        match self {
            ReturningType::Result(inner) => matches!(**inner, ReturningType::Unit),
            _ => false,
        }
    }

    fn is_numeric(&self) -> bool {
        matches!(
            self,
            ReturningType::U16
                | ReturningType::U32
                | ReturningType::U64
                | ReturningType::I16
                | ReturningType::I32
                | ReturningType::I64
                | ReturningType::F32
                | ReturningType::F64
        )
    }

    fn cast(&self, value: &str) -> Result<TokenStream, WebCodeGenError> {
        match self {
            ReturningType::I16 => {
                let value = value.parse::<i16>()?;
                Ok(quote::quote! {
                    #value
                })
            }
            ReturningType::I32 => {
                let value = value.parse::<i32>()?;
                Ok(quote::quote! {
                    #value
                })
            }
            ReturningType::I64 => {
                let value = value.parse::<i64>()?;
                Ok(quote::quote! {
                    #value
                })
            }
            ReturningType::F32 => {
                let value = value.parse::<f32>()?;
                Ok(quote::quote! {
                    #value
                })
            }
            ReturningType::F64 => {
                let value = value.parse::<f64>()?;
                Ok(quote::quote! {
                    #value
                })
            }
            ReturningType::Boolean => {
                let value = value.parse::<bool>()?;
                Ok(quote::quote! {
                    #value
                })
            }
            unsupported => {
                unimplemented!("Unsupported cast from string `{value}` to {unsupported:?}.",);
            }
        }
    }

    fn try_from_pg_type(ty: PgType, conn: &mut PgConnection) -> Result<Self, WebCodeGenError> {
        if ty.is_boolean(conn)? {
            Ok(ReturningType::Boolean)
        } else if ty.is_i16(conn)? {
            Ok(ReturningType::I16)
        } else if ty.is_i32(conn)? {
            Ok(ReturningType::I32)
        } else if ty.is_i64(conn)? {
            Ok(ReturningType::I64)
        } else if ty.is_u16(conn)? {
            Ok(ReturningType::U16)
        } else if ty.is_u32(conn)? {
            Ok(ReturningType::U32)
        } else if ty.is_u64(conn)? {
            Ok(ReturningType::U64)
        } else if ty.is_f32(conn)? {
            Ok(ReturningType::F32)
        } else if ty.is_f64(conn)? {
            Ok(ReturningType::F64)
        } else if ty.is_text(conn)? {
            Ok(ReturningType::Textual)
        } else {
            Ok(ReturningType::Custom(Box::from(ty)))
        }
    }
}

impl<C1, C2> TranslateExpression<'_, C1, C2>
where
    C1: AsRef<Column> + Debug,
    C2: AsRef<Column> + Debug,
{
    /// Returns a reference to the column that corresponds to the provided
    /// column. It gives first priority to the columns in the
    /// `contextual_columns` and then to the columns in the `self_columns`.
    ///
    /// # Arguments
    ///
    /// * `column` - The column to get
    ///
    /// # Returns
    ///
    /// * A tuple containing the column and a boolean indicating whether the
    ///   column was found in the `contextual_columns` or not
    ///
    /// # Errors
    ///
    /// * If the column does not exist
    fn get_column(&self, column: &Column) -> Result<(&Column, bool), WebCodeGenError> {
        if let Some(contextual_column) =
            self.contextual_columns.iter().find(|c| c.as_ref().column_name == column.column_name)
        {
            Ok((contextual_column.as_ref(), true))
        } else if let Some(self_column) =
            self.self_columns.iter().find(|c| c.as_ref().column_name == column.column_name)
        {
            Ok((self_column.as_ref(), false))
        } else {
            Err(CheckConstraintError::NoInvolvedColumns(
                Box::new(column.clone()),
                Box::new(self.check_constraint.clone()),
            )
            .into())
        }
    }

    /// Returns the formatted column for use as an argument.
    ///
    /// # Arguments
    ///
    /// * `column` - The column to format
    /// * `unpacking` - Whether to unpack the column
    ///
    /// # Returns
    ///
    /// * The formatted column
    ///
    /// # Errors
    ///
    /// * If the column does not exist
    fn formatted_column(
        &self,
        column: &Column,
        unpacking: bool,
        conn: &mut PgConnection,
    ) -> Result<proc_macro2::TokenStream, WebCodeGenError> {
        let (argument_column, is_contextual) = self.get_column(column)?;
        let column_ident = argument_column.snake_case_ident()?;
        let mut column_ident = if is_contextual || !unpacking {
            quote::quote! { #column_ident }
        } else {
            quote::quote! { self.#column_ident }
        };

        if (unpacking || !argument_column.is_nullable()) && !argument_column.supports_copy(conn)? {
            column_ident = quote::quote! { #column_ident.as_ref() };
        }

        Ok(column_ident)
    }

    /// Returns whether the provide column is nullable.
    fn is_nullable(&self, column: &Column) -> Result<bool, WebCodeGenError> {
        self.get_column(column).map(|(column, _)| column.is_nullable())
    }

    /// Returns reference to the requested function by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function
    ///
    /// # Returns
    ///
    /// * A reference to the function if it exists
    ///
    /// # Errors
    ///
    /// * If the function does not exist
    fn get_function_by_name(&self, name: &str) -> Result<&PgProc, WebCodeGenError> {
        self.functions
            .iter()
            .find(|f| f.proname == name)
            .ok_or_else(|| WebCodeGenError::UnknownPostgresProc(name.to_string()))
    }

    /// Returns reference to the requested involved column by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the column
    ///
    /// # Returns
    ///
    /// * A reference to the column if it exists
    ///
    /// # Errors
    ///
    /// * If the column does not exist
    fn get_involved_column_by_name(&self, name: &str) -> Result<&Column, WebCodeGenError> {
        self.involved_columns
            .iter()
            .find(|c| c.column_name == name)
            .ok_or_else(|| WebCodeGenError::ColumnNotFound(name.to_string()))
    }

    /// Translates the provided function argument to a
    /// [`TokenStream`](proc_macro2::TokenStream)
    fn parse_function_argument_expr(
        &self,
        arg: &FunctionArgExpr,
        arg_type: &ReturningType,
        conn: &mut PgConnection,
    ) -> Result<(proc_macro2::TokenStream, Option<&'_ Column>), WebCodeGenError> {
        match arg {
            FunctionArgExpr::Expr(expr) => {
                let (token_stream, scoped_columns, _returning_type): (_, Vec<&Column>, _) =
                    self.parse(expr, Some(arg_type), conn)?;
                if scoped_columns.len() > 1 {
                    return Err(CheckConstraintError::UnsupportedSyntax(
                        Box::new(self.check_constraint.clone()),
                        UnsupportedCheckConstraintErrorSyntax::ExpectedSingleScopedColumn(
                            scoped_columns.len(),
                        ),
                    )
                    .into());
                }
                Ok((token_stream, scoped_columns.first().copied()))
            }
            FunctionArgExpr::QualifiedWildcard(_) => {
                unimplemented!("QualifiedWildcard not supported");
            }
            FunctionArgExpr::Wildcard => {
                unimplemented!("Wildcard not supported");
            }
        }
    }

    /// Translates the provided function argument to a
    /// [`TokenStream`](proc_macro2::TokenStream)
    fn parse_function_argument(
        &self,
        arg: &FunctionArg,
        arg_type: &ReturningType,
        conn: &mut PgConnection,
    ) -> Result<(proc_macro2::TokenStream, Option<&'_ Column>), WebCodeGenError> {
        match arg {
            FunctionArg::Named { .. } => {
                unimplemented!("Named arguments not supported");
            }
            FunctionArg::ExprNamed { .. } => {
                unimplemented!("ExprNamed arguments not supported");
            }
            FunctionArg::Unnamed(arg) => self.parse_function_argument_expr(arg, arg_type, conn),
        }
    }

    /// Translates the provided list of function arguments to a
    /// [`TokenStream`](proc_macro2::TokenStream)
    fn parse_function_argument_list(
        &self,
        args: &FunctionArgumentList,
        argument_types: &[ReturningType],
        conn: &mut PgConnection,
    ) -> Result<(Vec<proc_macro2::TokenStream>, Vec<&'_ Column>), WebCodeGenError> {
        let mut token_stream = Vec::with_capacity(args.args.len());
        let mut columns = Vec::new();
        assert_eq!(args.args.len(), argument_types.len());
        for (arg, arg_type) in args.args.iter().zip(argument_types.iter()) {
            let (column_token_stream, column) =
                self.parse_function_argument(arg, arg_type, conn)?;
            token_stream.push(column_token_stream);
            columns.extend(column);
        }
        Ok((token_stream, columns))
    }

    /// Translates the provided function arguments to a
    /// [`TokenStream`](proc_macro2::TokenStream)
    fn parse_function_arguments(
        &self,
        args: &FunctionArguments,
        argument_types: &[ReturningType],
        conn: &mut PgConnection,
    ) -> Result<(Vec<proc_macro2::TokenStream>, Vec<&'_ Column>), WebCodeGenError> {
        match args {
            FunctionArguments::None => Ok((Vec::new(), Vec::new())),
            FunctionArguments::Subquery(_) => {
                unimplemented!("Subquery arguments not supported");
            }
            FunctionArguments::List(args) => {
                self.parse_function_argument_list(args, argument_types, conn)
            }
        }
    }

    /// Translates the provided SQL function call to a
    /// [`TokenStream`](proc_macro2::TokenStream)
    fn parse_function(
        &self,
        sqlparser::ast::Function {
            name,
            uses_odbc_syntax,
            parameters,
            args,
            filter,
            null_treatment,
            over,
            within_group,
        }: &sqlparser::ast::Function,
        conn: &mut PgConnection,
    ) -> Result<(proc_macro2::TokenStream, ReturningType), WebCodeGenError> {
        if !within_group.is_empty() {
            unimplemented!("WithinGroup not supported");
        }
        if null_treatment.is_some() {
            unimplemented!("NullTreatment not supported");
        }
        if !matches!(parameters, FunctionArguments::None) {
            unimplemented!("Parameters not supported");
        }
        if over.is_some() {
            unimplemented!("Over not supported");
        }
        if filter.is_some() {
            unimplemented!("Filter not supported");
        }
        if *uses_odbc_syntax {
            unimplemented!("ODBC syntax not supported");
        }
        let function = self.get_function_by_name(&name.to_string())?;

        let mut argument_types = Vec::new();
        for argument_type in function.argument_types(conn)? {
            argument_types.push(ReturningType::try_from_pg_type(argument_type, conn)?);
        }

        let (args, scoped_columns) = self.parse_function_arguments(args, &argument_types, conn)?;
        let function_path = function.path(conn)?;

        let map_err = if function.returns_result(conn)? {
            let attributes_enumeration = self.attributes_enumeration;

            let attributes = scoped_columns
                .iter()
                .map(|scoped_column| {
                    let camel_cased = scoped_column.camel_case_ident()?;
                    Ok(quote::quote! { #attributes_enumeration::#camel_cased })
                })
                .collect::<Result<Vec<_>, WebCodeGenError>>()?;

            match scoped_columns.len() {
                0 => {
                    return Err(CheckConstraintError::UnsupportedSyntax(
                        Box::new(self.check_constraint.clone()),
                        UnsupportedCheckConstraintErrorSyntax::ExpectedScopedColumn,
                    )
                    .into());
                }
                1 => {
                    quote::quote! {
                        .map_err(|e| e.rename_field(#(#attributes),* ))
                    }
                }
                2 => {
                    quote::quote! {
                        .map_err(|e| e.rename_fields(#(#attributes),* ))
                    }
                }
                _ => {
                    unimplemented!("More than two scoped columns not supported");
                }
            }
        } else {
            TokenStream::new()
        };

        Ok((
            quote::quote! {
                #function_path(#(#args),*)#map_err
            },
            if function.returns_result(conn)? {
                ReturningType::Result(ReturningType::Unit.into())
            } else {
                ReturningType::try_from_pg_type(function.return_type(conn)?, conn)?
            },
        ))
    }

    /// Verifies that the [`CastKind`](sqlparser::ast::CastKind) is supported
    ///
    /// # Arguments
    ///
    /// * `kind` - The [`CastKind`](sqlparser::ast::CastKind) to verify
    fn verify_cast_kind(kind: &sqlparser::ast::CastKind) {
        match kind {
            sqlparser::ast::CastKind::DoubleColon => {}
            _ => {
                unimplemented!("Unsupported cast kind: {kind:?}");
            }
        }
    }

    /// Parses the provided [`Value`](sqlparser::ast::Value) to a
    /// [`TokenStream`](proc_macro2::TokenStream)
    ///
    /// # Arguments
    ///
    /// * `value` - The [`Value`](sqlparser::ast::Value) to parse
    /// * `type_hint` - The [`PgType`](crate::table_metadata::PgType) of the
    ///   value
    ///
    /// # Errors
    ///
    /// * If the provided [`Value`](sqlparser::ast::Value) is not supported
    fn parse_value(
        &self,
        value: &sqlparser::ast::Value,
        type_hint: Option<&ReturningType>,
    ) -> Result<(proc_macro2::TokenStream, ReturningType), WebCodeGenError> {
        match value {
            sqlparser::ast::Value::Boolean(value) => {
                Ok((quote::quote! { #value }, ReturningType::Boolean))
            }
            sqlparser::ast::Value::Number(value, _) => {
                match type_hint {
                    Some(type_hint) => Ok((type_hint.cast(value)?, type_hint.clone())),
                    None => {
                        unimplemented!(
                            "Number without type hint not supported: {:?}",
                            self.check_constraint
                        );
                    }
                }
            }
            sqlparser::ast::Value::SingleQuotedString(value) => {
                Ok((quote::quote! { #value }, ReturningType::Textual))
            }
            other => {
                unimplemented!("Unsupported value: {:?}", other);
            }
        }
    }

    /// Parses the provided [`ValueWithSpan`](sqlparser::ast::ValueWithSpan) to
    /// a [`TokenStream`](proc_macro2::TokenStream)
    ///
    /// # Arguments
    ///
    /// * `value` - The [`ValueWithSpan`](sqlparser::ast::ValueWithSpan) to
    ///   parse
    /// * `type_hint` - The [`PgType`](crate::table_metadata::PgType) of the
    ///   value
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If the provided [`ValueWithSpan`](sqlparser::ast::ValueWithSpan) is
    ///   not supported
    fn parse_value_with_span(
        &self,
        value: &sqlparser::ast::ValueWithSpan,
        type_hint: Option<&ReturningType>,
    ) -> Result<(proc_macro2::TokenStream, ReturningType), WebCodeGenError> {
        self.parse_value(&value.value, type_hint)
    }

    #[allow(clippy::too_many_lines)]
    /// Translates the provided expression to a
    /// [`TokenStream`](proc_macro2::TokenStream)
    fn parse(
        &self,
        expr: &Expr,
        type_hint: Option<&ReturningType>,
        conn: &mut PgConnection,
    ) -> Result<(proc_macro2::TokenStream, Vec<&'_ Column>, ReturningType), WebCodeGenError> {
        match expr {
            Expr::Function(function) => {
                let (token_stream, returning_type) = self.parse_function(function, conn)?;
                Ok((token_stream, Vec::new(), returning_type))
            }
            Expr::Cast { kind, expr, data_type: _, format } => {
                Self::verify_cast_kind(kind);
                if format.is_some() {
                    unimplemented!("Format not supported");
                }
                self.parse(expr, type_hint, conn)
            }
            Expr::Nested(expr) => self.parse(expr, type_hint, conn),
            Expr::Identifier(ident) => {
                let involved_column = self.get_involved_column_by_name(&ident.value)?;
                Ok((
                    self.formatted_column(involved_column, false, conn)?,
                    vec![involved_column],
                    ReturningType::try_from_pg_type(involved_column.pg_type(conn)?, conn)?,
                ))
            }
            Expr::BinaryOp { left, op, right } => {
                match op {
                    BinaryOperator::And => {
                        let (left, left_scoped_columns, left_returning_type) =
                            self.parse(left, None, conn)?;
                        let (right, right_scoped_columns, right_returning_type) =
                            self.parse(right, None, conn)?;
                        if !left_scoped_columns.is_empty() || !right_scoped_columns.is_empty() {
                            unimplemented!("Scoped columns not supported");
                        }
                        if matches!(left_returning_type, ReturningType::Boolean)
                            && matches!(right_returning_type, ReturningType::Boolean)
                        {
                            Ok((
                                quote::quote! {
                                    #left && #right
                                },
                                Vec::new(),
                                ReturningType::Boolean,
                            ))
                        } else if left_returning_type.is_unit_result()
                            || right_returning_type.is_unit_result()
                        {
                            Ok((
                                quote::quote! {
                                    #left.and_then(|_| #right)
                                },
                                Vec::new(),
                                ReturningType::Result(ReturningType::Unit.into()),
                            ))
                        } else {
                            unimplemented!("Unsupported binary operation");
                        }
                    }
                    BinaryOperator::Or => {
                        let (left, left_scoped_columns, left_returning_type) =
                            self.parse(left, None, conn)?;
                        let (right, right_scoped_columns, right_returning_type) =
                            self.parse(right, None, conn)?;
                        if !left_scoped_columns.is_empty() || !right_scoped_columns.is_empty() {
                            unimplemented!("Scoped columns not supported");
                        }
                        if matches!(left_returning_type, ReturningType::Boolean)
                            && matches!(right_returning_type, ReturningType::Boolean)
                        {
                            Ok((
                                quote::quote! {
                                    #left || #right
                                },
                                Vec::new(),
                                ReturningType::Boolean,
                            ))
                        } else if left_returning_type.is_unit_result()
                            || right_returning_type.is_unit_result()
                        {
                            Ok((
                                quote::quote! {
                                    #left.or_else(|_| #right)
                                },
                                Vec::new(),
                                ReturningType::Result(ReturningType::Unit.into()),
                            ))
                        } else {
                            unimplemented!("Unsupported binary operation");
                        }
                    }
                    BinaryOperator::NotEq
                    | BinaryOperator::Eq
                    | BinaryOperator::Gt
                    | BinaryOperator::Lt
                    | BinaryOperator::GtEq
                    | BinaryOperator::LtEq => {
                        let (left, _, left_returning_type) = self.parse(left, None, conn)?;
                        let (right, _, right_returning_type) =
                            self.parse(right, Some(&left_returning_type), conn)?;
                        if left_returning_type != right_returning_type {
                            unimplemented!(
                                "Equality between different types not supported: {left_returning_type:?} and {right_returning_type:?}. {:?}",
                                self.check_constraint
                            );
                        }
                        let operator_symbol: syn::BinOp = match op {
                            BinaryOperator::Eq => syn::BinOp::Eq(syn::token::EqEq::default()),
                            BinaryOperator::NotEq => syn::BinOp::Ne(syn::token::Ne::default()),
                            BinaryOperator::Gt => syn::BinOp::Gt(syn::token::Gt::default()),
                            BinaryOperator::Lt => syn::BinOp::Lt(syn::token::Lt::default()),
                            BinaryOperator::GtEq => syn::BinOp::Ge(syn::token::Ge::default()),
                            BinaryOperator::LtEq => syn::BinOp::Le(syn::token::Le::default()),
                            _ => unreachable!(),
                        };
                        Ok((
                            quote::quote! {
                                #left #operator_symbol #right
                            },
                            Vec::new(),
                            ReturningType::Boolean,
                        ))
                    }
                    BinaryOperator::Plus
                    | BinaryOperator::Minus
                    | BinaryOperator::Multiply
                    | BinaryOperator::Divide
                    | BinaryOperator::Modulo => {
                        let (left, _, left_returning_type) = self.parse(left, type_hint, conn)?;
                        let (right, _, right_returning_type) =
                            self.parse(right, type_hint, conn)?;
                        if left_returning_type != right_returning_type {
                            unimplemented!(
                                "Binary operation between different types not supported: {left_returning_type:?} and {right_returning_type:?}. {:?}",
                                self.check_constraint
                            );
                        }
                        if left_returning_type.is_numeric() && right_returning_type.is_numeric() {
                            let operator_symbol: syn::BinOp = match op {
                                BinaryOperator::Plus => {
                                    syn::BinOp::Add(syn::token::Plus::default())
                                }
                                BinaryOperator::Minus => {
                                    syn::BinOp::Sub(syn::token::Minus::default())
                                }
                                BinaryOperator::Multiply => {
                                    syn::BinOp::Mul(syn::token::Star::default())
                                }
                                BinaryOperator::Divide => {
                                    syn::BinOp::Div(syn::token::Slash::default())
                                }
                                BinaryOperator::Modulo => {
                                    syn::BinOp::Rem(syn::token::Percent::default())
                                }
                                _ => unreachable!(),
                            };
                            Ok((
                                quote::quote! {
                                    #left #operator_symbol #right
                                },
                                Vec::new(),
                                left_returning_type,
                            ))
                        } else {
                            unimplemented!(
                                "Unsupported binary operation {} between {:?} and {:?}",
                                op,
                                left_returning_type,
                                right_returning_type
                            );
                        }
                    }
                    operator => {
                        unimplemented!("Unsupported binary operator: {operator:?}");
                    }
                }
            }
            Expr::Value(value) => {
                let (token_stream, returning_type) =
                    self.parse_value_with_span(value, type_hint)?;
                Ok((token_stream, Vec::new(), returning_type))
            }
            _ => {
                unimplemented!(
                    "Unsupported expression: {:?}, from check constraint: {:?}",
                    expr,
                    self.check_constraint
                )
            }
        }
    }
}

impl CheckConstraint {
    #[must_use]
    /// Returns whether the current [`CheckConstraint`] is known to come
    /// from Postgis and therefore should most likely be ignored
    pub fn is_postgis_constraint(&self) -> bool {
        POSTGIS_CONSTRAINTS.contains(&self.constraint_name.as_str())
    }

    /// Returns the [`TokenStream`](proc_macro2::TokenStream) of the check
    /// clause, including all functions specified from the provided
    /// extensions.
    ///
    /// # Arguments
    ///
    /// * `contextual_columns` - The columns available in the context
    /// * `self_columns` - The columns that are part of the current table
    /// * `extensions` - The extensions to consider
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    ///
    /// # Returns
    ///
    /// * `None` if the provided column is not involved in the check clause
    /// * `None` if the provided `extensions` are not involved in the check
    ///   clause
    ///
    /// # Panics
    ///
    /// * If the parser check clause cannot be parsed
    pub fn to_syn<E: AsRef<PgExtension>, C1: AsRef<Column> + Debug, C2: AsRef<Column> + Debug>(
        &self,
        contextual_columns: &[C1],
        self_columns: &[C2],
        extensions: &[E],
        conn: &mut PgConnection,
    ) -> Result<proc_macro2::TokenStream, WebCodeGenError> {
        assert!(!contextual_columns.is_empty());

        let table = contextual_columns[0].as_ref().table(conn)?;
        let attributes_enumeration = table.insertable_enum_ty()?;

        let functions = self.functions(conn)?;
        let operators = self.operators(conn)?;

        // If any of the functions are not from the provided extensions,
        // then it is not possible to generate the check clause.
        for f in &functions {
            let Some(extension) = f.extension(conn)? else {
                return Err(CheckConstraintError::FunctionNotFromProvidedExtensions(
                    Box::new(f.clone()),
                    Box::new(self.clone()),
                )
                .into());
            };
            if !extensions.iter().any(|e| e.as_ref() == &extension) {
                return Err(CheckConstraintError::FunctionNotFromProvidedExtensions(
                    Box::new(f.clone()),
                    Box::new(self.clone()),
                )
                .into());
            }
        }

        if functions.is_empty() {
            return Err(CheckConstraintError::NoFunctionCalls(Box::new(self.clone())).into());
        }

        // If any of the operators are not plain Rust operators, then it is not
        // possible to generate the check clause.
        if !operators.is_empty() {
            return Err(CheckConstraintError::OperatorsNotSupported.into());
        }

        let parsed_check_clause = Parser::new(&PostgreSqlDialect {})
            .try_with_sql(&self.check_clause)
            .expect("Failed to parse check clause")
            .parse_expr()
            .expect("Failed to parse check clause");

        let involved_columns = self.columns(conn)?;
        let translator = TranslateExpression {
            check_constraint: self,
            contextual_columns,
            self_columns,
            involved_columns: involved_columns.as_slice(),
            attributes_enumeration: &attributes_enumeration,
            functions: &functions,
        };

        let (mut translated_check_clause, scoped_columns, returning_type) =
            translator.parse(&parsed_check_clause, None, conn)?;

        if !scoped_columns.is_empty() {
            return Err(CheckConstraintError::UnsupportedSyntax(
                Box::new(self.clone()),
                UnsupportedCheckConstraintErrorSyntax::ExpectedNoScopedColumn(scoped_columns.len()),
            )
            .into());
        }

        if !returning_type.is_unit_result() {
            return Err(
                CheckConstraintError::TopLevelExpressionNotResult(Box::new(self.clone())).into()
            );
        }

        translated_check_clause = quote::quote! {
            #translated_check_clause?;
        };

        let optional_involved_columns = involved_columns
            .iter()
            .filter(|involved_column| translator.is_nullable(involved_column).unwrap_or(false))
            .collect::<Vec<_>>();

        if !optional_involved_columns.is_empty() {
            let mut left_assignment = Vec::new();
            let mut right_assignment = Vec::new();
            for optional_involved_column in optional_involved_columns {
                let column_ident = optional_involved_column.snake_case_ident()?;
                let formatted_column =
                    translator.formatted_column(optional_involved_column, true, conn)?;
                left_assignment.push(quote::quote! { Some(#column_ident) });
                right_assignment.push(formatted_column);
            }
            let left_assignment = if left_assignment.len() > 1 {
                quote::quote! { (#(#left_assignment),*) }
            } else {
                quote::quote! { #(#left_assignment)* }
            };
            let right_assignment = if right_assignment.len() > 1 {
                quote::quote! { (#(#right_assignment),*) }
            } else {
                quote::quote! { #(#right_assignment)* }
            };
            translated_check_clause = quote::quote! {
                if let #left_assignment = #right_assignment {
                    #translated_check_clause
                }
            };
        }

        Ok(translated_check_clause)
    }

    /// Returns the vector of [`PgProc`] functions that are used in the check
    /// clause
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn functions(&self, conn: &mut PgConnection) -> Result<Vec<PgProc>, WebCodeGenError> {
        Ok(self.pg_constraint(conn)?.functions(conn)?)
    }

    /// Returns the vector of [`PgOperator`] operators that are used in the
    /// check clause
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn operators(&self, conn: &mut PgConnection) -> Result<Vec<PgOperator>, WebCodeGenError> {
        Ok(self.pg_constraint(conn)?.operators(conn)?)
    }

    /// Returns the [`PgConstraint`] that corresponds to this check constraint
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn pg_constraint(&self, conn: &mut PgConnection) -> Result<PgConstraint, WebCodeGenError> {
        use diesel::RunQueryDsl;

        use crate::schema::{pg_constraint, pg_namespace};
        pg_constraint::table
            .inner_join(pg_namespace::table.on(pg_constraint::connamespace.eq(pg_namespace::oid)))
            .filter(
                pg_constraint::conname
                    .eq(&self.constraint_name)
                    .and(pg_constraint::contype.eq("c")),
            )
            .filter(pg_namespace::nspname.eq(&self.constraint_schema))
            .select(PgConstraint::as_select())
            .first(conn)
            .map_err(WebCodeGenError::from)
    }

    /// Returns the table constraint associated with this check constraint
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn table_constraint(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TableConstraint, WebCodeGenError> {
        use diesel::RunQueryDsl;

        use crate::schema::table_constraints;

        Ok(table_constraints::table
            .filter(
                table_constraints::constraint_name
                    .eq(&self.constraint_name)
                    .and(table_constraints::constraint_catalog.eq(&self.constraint_catalog))
                    .and(table_constraints::constraint_schema.eq(&self.constraint_schema)),
            )
            .select(TableConstraint::as_select())
            .first(conn)?)
    }

    /// Returns the table that this check constraint belongs to
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn table(&self, conn: &mut PgConnection) -> Result<Arc<Table>, WebCodeGenError> {
        self.table_constraint(conn)?.table(conn)
    }

    /// Returns all the columns associated to this check constraint
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If their is an error while querying the database.
    pub fn columns(&self, conn: &mut PgConnection) -> Result<Arc<Vec<Column>>, WebCodeGenError> {
        columns(self, conn)
    }

    /// Returns whether the current check constraint is a `distinct` constraint,
    /// i.e. it checks whether two columns are distinct.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    /// * If their is an error while querying the database.
    pub fn is_distinct_constraint(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<Arc<Vec<Column>>>, WebCodeGenError> {
        let columns = self.columns(conn)?;
        if columns.len() != 2 {
            return Ok(None);
        }
        let parsed_check_clause = Parser::new(&PostgreSqlDialect {})
            .try_with_sql(&self.check_clause)
            .expect("Failed to parse check clause")
            .parse_expr()
            .expect("Failed to parse check clause");
        match parsed_check_clause {
            Expr::Function(function) => {
                if !function.name.to_string().starts_with("must_be_distinct") {
                    return Ok(None);
                }
                Ok(Some(columns))
            }
            _ => Ok(None),
        }
    }
}
