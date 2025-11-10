//! Submodule providing the `TranslateExpression` struct for translating SQL
//! check constraint expressions into Rust code.
use proc_macro2::TokenStream;
use quote::quote;
use sql_traits::traits::{CheckConstraintLike, ColumnLike, DatabaseLike, FunctionLike, TableLike};
use sqlparser::ast::{
    BinaryOperator, Expr, FunctionArg, FunctionArgExpr, FunctionArgumentList, FunctionArguments,
    Ident, Value, ValueWithSpan,
};
use synql_attributes::traits::TableAttributesLike;
use synql_core::{
    prelude::Builder,
    structs::{
        DataVariantRef, ExternalFunctionRef, InternalToken, InternalTokenBuilder, Workspace,
    },
    traits::{ColumnSynLike, FunctionSynLike},
};

pub(super) struct TranslateExpression<'local, 'db, 'data, DB: DatabaseLike> {
    check_constraint: &'db DB::CheckConstraint,
    workspace: &'local Workspace<'data>,
    database: &'db DB,
    contextual_columns: &'local [&'db DB::Column],
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

/// Returns the direction-inverted operator for the provided binary operator.
fn invert_operator(op: &BinaryOperator) -> BinaryOperator {
    match op {
        BinaryOperator::Eq => BinaryOperator::Eq,
        BinaryOperator::NotEq => BinaryOperator::NotEq,
        BinaryOperator::Gt => BinaryOperator::Lt,
        BinaryOperator::Lt => BinaryOperator::Gt,
        BinaryOperator::GtEq => BinaryOperator::LtEq,
        BinaryOperator::LtEq => BinaryOperator::GtEq,
        _ => {
            unimplemented!("Cannot invert unsupported operator: {op:?}");
        }
    }
}

impl<'local, 'db, 'data, DB> TranslateExpression<'local, 'db, 'data, DB>
where
    DB: DatabaseLike,
{
    pub(super) fn new(
        check_constraint: &'db DB::CheckConstraint,
        workspace: &'local Workspace<'data>,
        database: &'db DB,
        contextual_columns: &'local [&'db DB::Column],
    ) -> Self {
        Self { check_constraint, workspace, database, contextual_columns }
    }

    /// Maps the provided expression to a validation error, when applicable.
    fn map_expr_to_validation_error(&self, expr: &Expr) -> Option<InternalToken> {
        match expr {
            Expr::BinaryOp { left, right, op } => {
                match (left.as_ref(), right.as_ref()) {
                    (
                        Expr::Identifier(Ident { value: ident, .. }),
                        Expr::Value(ValueWithSpan { value, .. }),
                    ) => self.map_expr_to_single_field_error(ident, value, op),
                    (
                        Expr::Value(ValueWithSpan { value, .. }),
                        Expr::Identifier(Ident { value: ident, .. }),
                    ) => self.map_expr_to_single_field_error(ident, value, &invert_operator(op)),
                    (
                        Expr::Identifier(Ident { value: left_ident, .. }),
                        Expr::Identifier(Ident { value: right_ident, .. }),
                    ) => self.map_expr_to_double_field_error(left_ident, right_ident, op),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn map_expr_to_double_field_error(
        &self,
        left: &str,
        right: &str,
        op: &BinaryOperator,
    ) -> Option<InternalToken> {
        let left_column = self.column(left);
        let right_column = self.column(right);
        let formatted_left = self.formatted_column(left_column, true);
        let formatted_right = self.formatted_column(right_column, true);
        let table_attribute_enum = self.attributes_enumeration();
        let left_camel_cased = left_column.column_camel_ident();
        let right_camel_cased = right_column.column_camel_ident();
        let validation_error = self
            .workspace
            .external_type(&syn::parse_quote!(validation_errors::prelude::ValidationError))
            .unwrap();
        match op {
            BinaryOperator::NotEq => {
                Some(
                    InternalToken::new()
                        .private()
                        .stream(quote! {
                            if #formatted_left == #formatted_right {
                                return Err(#validation_error::equal(
                                    #table_attribute_enum::#left_camel_cased,
                                    #table_attribute_enum::#right_camel_cased
                                ));
                            }
                        })
                        .data(table_attribute_enum)
                        .build()
                        .unwrap(),
                )
            }
            BinaryOperator::LtEq => {
                Some(
                    InternalToken::new()
                        .private()
                        .stream(quote! {
                            if #formatted_left > #formatted_right {
                                return Err(#validation_error::smaller_than(
                                    #table_attribute_enum::#left_camel_cased,
                                    #table_attribute_enum::#right_camel_cased
                                ));
                            }
                        })
                        .data(table_attribute_enum)
                        .build()
                        .unwrap(),
                )
            }
            BinaryOperator::Lt => {
                Some(
                    InternalToken::new()
                        .private()
                        .stream(quote! {
                            if #formatted_left >= #formatted_right {
                                return Err(#validation_error::strictly_smaller_than(
                                    #table_attribute_enum::#left_camel_cased,
                                    #table_attribute_enum::#right_camel_cased
                                ));
                            }
                        })
                        .data(table_attribute_enum)
                        .build()
                        .unwrap(),
                )
            }
            BinaryOperator::Gt => {
                Some(
                    InternalToken::new()
                        .private()
                        .stream(quote! {
                            if #formatted_left <= #formatted_right {
                                return Err(#validation_error::strictly_greater_than(
                                    #table_attribute_enum::#left_camel_cased,
                                    #table_attribute_enum::#right_camel_cased
                                ));
                            }
                        })
                        .data(table_attribute_enum)
                        .build()
                        .unwrap(),
                )
            }
            BinaryOperator::GtEq => {
                Some(
                    InternalToken::new()
                        .private()
                        .stream(quote! {
                            if #formatted_left < #formatted_right {
                                return Err(#validation_error::greater_than(
                                    #table_attribute_enum::#left_camel_cased,
                                    #table_attribute_enum::#right_camel_cased
                                ));
                            }
                        })
                        .data(table_attribute_enum)
                        .build()
                        .unwrap(),
                )
            }
            _ => {
                unimplemented!("Operator {op:?} not supported for double field error mapping");
            }
        }
    }

    fn map_expr_to_single_field_error(
        &self,
        ident: &str,
        value: &Value,
        op: &BinaryOperator,
    ) -> Option<InternalToken> {
        let column = self.column(ident);
        let formatted_column = self.formatted_column(column, false);
        let table_attribute_enum = self.attributes_enumeration();
        let camel_cased = column.column_camel_ident();
        let validation_error = self
            .workspace
            .external_type(&syn::parse_quote!(validation_errors::prelude::ValidationError))?;
        match op {
            BinaryOperator::NotEq => {
                if column.is_textual(self.database)
                    && value == &Value::SingleQuotedString("".to_string())
                {
                    Some(
                        InternalToken::new()
                            .private()
                            .stream(quote! {
                                if #formatted_column.is_empty() {
                                    return Err(#validation_error::empty(#table_attribute_enum::#camel_cased));
                                }
                            })
                            .data(table_attribute_enum)
                            .build()
                            .unwrap(),
                    )
                } else {
                    unimplemented!("Operator {op:?} not supported for single field error mapping");
                }
            }
            BinaryOperator::LtEq => {
                let column_value = self.parse_column_value(column, value).0;
                let float_value = self.parse_value(value, Some(&DataVariantRef::f64())).0;
                Some(
                    InternalToken::new()
                        .private()
                        .stream(quote! {
                            if #formatted_column > #column_value {
                                return Err(#validation_error::smaller_than_value(
                                    #table_attribute_enum::#camel_cased,
                                    #float_value
                                ));
                            }
                        })
                        .data(table_attribute_enum)
                        .build()
                        .unwrap(),
                )
            }
            BinaryOperator::Lt => {
                let column_value = self.parse_column_value(column, value).0;
                let float_value = self.parse_value(value, Some(&DataVariantRef::f64())).0;
                Some(
                    InternalToken::new()
                        .private()
                        .stream(quote! {
                            if #formatted_column >= #column_value {
                                return Err(#validation_error::strictly_smaller_than_value(
                                    #table_attribute_enum::#camel_cased,
                                    #float_value
                                ));
                            }
                        })
                        .data(table_attribute_enum)
                        .build()
                        .unwrap(),
                )
            }
            BinaryOperator::Gt => {
                let column_value = self.parse_column_value(column, value).0;
                let float_value = self.parse_value(value, Some(&DataVariantRef::f64())).0;
                Some(
                    InternalToken::new()
                        .private()
                        .stream(quote! {
                            if #formatted_column <= #column_value {
                                return Err(#validation_error::strictly_greater_than_value(
                                    #table_attribute_enum::#camel_cased,
                                    #float_value
                                ));
                            }
                        })
                        .data(table_attribute_enum)
                        .build()
                        .unwrap(),
                )
            }
            BinaryOperator::GtEq => {
                let column_value = self.parse_column_value(column, value).0;
                let float_value = self.parse_value(value, Some(&DataVariantRef::f64())).0;
                Some(
                    InternalToken::new()
                        .private()
                        .stream(quote! {
                            if #formatted_column < #column_value {
                                return Err(#validation_error::greater_than_value(
                                    #table_attribute_enum::#camel_cased,
                                    #float_value
                                ));
                            }
                        })
                        .data(table_attribute_enum)
                        .build()
                        .unwrap(),
                )
            }
            _ => {
                unimplemented!("Operator {op:?} not supported for single field error mapping");
            }
        }
    }

    /// Returns whether the provided column is contextual.
    ///
    /// # Arguments
    ///
    /// * `column` - The column to check
    fn is_contextual(&self, column: &DB::Column) -> bool {
        self.contextual_columns.contains(&column)
    }

    /// Returns the formatted column for use as an argument.
    ///
    /// # Arguments
    ///
    /// * `column` - The column to format
    fn formatted_column(&self, column: &DB::Column, reference: bool) -> TokenStream {
        let column_ident = column.column_snake_ident();
        // When a column is either contextual or nullable, it can be accessed directly
        // because we expect it to be accessible in the current scope.
        if self.is_contextual(column) {
            if column.supports_copy(self.database, self.workspace) || !reference {
                quote! { #column_ident }
            } else {
                quote! { &#column_ident }
            }
        } else {
            quote! { #column_ident }
        }
    }

    /// Returns reference to the table of the check constraint.
    fn table(&self) -> &DB::Table {
        self.check_constraint.table(self.database)
    }

    /// Returns the attributes enumeration of the table of the check constraint.
    fn attributes_enumeration(&self) -> DataVariantRef {
        let table = self.table();
        let table_attributes = table
            .attributes_ref(self.workspace)
            .expect("Failed to get table attributes for check constraint table");
        table_attributes.into()
    }

    /// Returns reference to the requested function by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function
    ///
    /// # Panics
    ///
    /// * If the function does not exist, which should not happen as
    /// it would mean that the provided SQL defining the database is invalid.
    fn function(&self, name: &str) -> &DB::Function {
        self.check_constraint
            .function(self.database, name)
            .expect(&format!("Function `{}` not found for check constraint", name))
    }

    /// Returns reference to the requested involved column by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the column
    ///
    /// # Panics
    ///
    /// * If the column does not exist, which should not happen as
    /// it would mean that the provided SQL defining the database is invalid.
    fn column(&self, name: &str) -> &DB::Column {
        self.check_constraint.column(self.database, name).unwrap()
    }

    /// Translates the provided function argument to a
    /// [`TokenStream`](proc_macro2::TokenStream)
    fn parse_function_argument_expr(
        &self,
        arg: &FunctionArgExpr,
        arg_type: &DataVariantRef,
    ) -> (InternalToken, Option<&'_ DB::Column>) {
        match arg {
            FunctionArgExpr::Expr(expr) => {
                let (token_stream, mut scoped_columns, _returning_type) =
                    self.inner_parse(expr, Some(arg_type));
                if scoped_columns.len() > 1 {
                    unimplemented!("Multiple scoped columns not supported");
                }
                (token_stream, scoped_columns.pop())
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
        arg_type: &DataVariantRef,
    ) -> (InternalToken, Option<&'_ DB::Column>) {
        match arg {
            FunctionArg::Named { .. } => {
                unimplemented!("Named arguments not supported");
            }
            FunctionArg::ExprNamed { .. } => {
                unimplemented!("ExprNamed arguments not supported");
            }
            FunctionArg::Unnamed(arg) => self.parse_function_argument_expr(arg, arg_type),
        }
    }

    /// Translates the provided list of function arguments to a
    /// [`TokenStream`](proc_macro2::TokenStream)
    fn parse_function_argument_list(
        &self,
        args: &FunctionArgumentList,
        argument_types: &[DataVariantRef],
    ) -> (Vec<InternalToken>, Vec<&'_ DB::Column>) {
        let mut token_stream = Vec::with_capacity(args.args.len());
        let mut columns = Vec::new();
        assert_eq!(args.args.len(), argument_types.len());
        for (arg, arg_type) in args.args.iter().zip(argument_types.iter()) {
            let (column_token_stream, column) = self.parse_function_argument(arg, arg_type);
            token_stream.push(column_token_stream);
            columns.extend(column);
        }
        (token_stream, columns)
    }

    /// Translates the provided function arguments to a
    /// [`TokenStream`](proc_macro2::TokenStream)
    fn parse_function_arguments(
        &self,
        args: &FunctionArguments,
        argument_types: &[DataVariantRef],
    ) -> (Vec<InternalToken>, Vec<&'_ DB::Column>) {
        match args {
            FunctionArguments::None => (Vec::new(), Vec::new()),
            FunctionArguments::Subquery(_) => {
                unimplemented!("Subquery arguments not supported");
            }
            FunctionArguments::List(args) => {
                self.parse_function_argument_list(args, argument_types)
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
    ) -> (InternalToken, DataVariantRef) {
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
        let function = self.function(&name.to_string());

        let argument_types = function
            .argument_types(self.workspace, self.database)
            .map(|arg_type| {
                arg_type
                    .expect(&format!(
                        "Failed to get type for argument of function `{}`",
                        function.name()
                    ))
                    .into()
            })
            .collect::<Vec<DataVariantRef>>();

        let (args, scoped_columns) = self.parse_function_arguments(args, &argument_types);
        let function_ref: ExternalFunctionRef =
            function.external_function_ref(self.workspace).expect(&format!(
                "The function `{}` should have an external function reference",
                function.name()
            ));

        let mut internal_token_builder: InternalTokenBuilder = InternalToken::new();

        let map_err = if function_ref.can_fail() {
            let attributes_enumeration = self.attributes_enumeration();
            internal_token_builder = internal_token_builder.data(attributes_enumeration.clone());

            let attributes = scoped_columns.iter().map(|scoped_column| {
                let camel_cased = scoped_column.column_camel_ident();
                quote! { #attributes_enumeration::#camel_cased }
            });

            match scoped_columns.len() {
                1 => {
                    quote! {
                        .map_err(|e| e.rename_field(#(#attributes),* ))
                    }
                }
                2 => {
                    quote! {
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

        let internal_token = internal_token_builder
            .inherits(args.iter().cloned())
            .stream(quote! {
                #function_ref(#(#args),*)#map_err
            })
            .build()
            .unwrap();

        let data_variant_ref: DataVariantRef =
            function_ref.return_type().cloned().unwrap_or_else(|| DataVariantRef::unit());

        (internal_token, data_variant_ref)
    }

    /// Parses the provided [`Value`](sqlparser::ast::Value) for the provided
    /// column.
    ///
    /// # Arguments
    ///
    /// * `column` - The column for which the value is being parsed
    /// * `value` - The [`Value`](sqlparser::ast::Value) to
    ///
    /// # Panics
    ///
    /// * If the provided [`Value`](sqlparser::ast::Value) is not supported
    /// * If the type of the provided column cannot be determined
    fn parse_column_value(
        &self,
        column: &DB::Column,
        value: &Value,
    ) -> (proc_macro2::TokenStream, DataVariantRef) {
        let column_type =
            column.external_postgres_type(self.workspace, self.database).expect(&format!(
                "Failed to get type for column `{}.{}` ({})",
                column.table(self.database).table_name(),
                column.column_name(),
                column.normalized_data_type(self.database)
            ));
        self.parse_value(value, Some(&column_type.into()))
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
    /// # Panics
    ///
    /// * If the provided [`Value`](sqlparser::ast::Value) is not supported
    fn parse_value(
        &self,
        value: &Value,
        type_hint: Option<&DataVariantRef>,
    ) -> (proc_macro2::TokenStream, DataVariantRef) {
        match value {
            Value::Boolean(value) => (quote! { #value }, DataVariantRef::bool()),
            Value::Number(value, _) => {
                match type_hint {
                    Some(type_hint) => (type_hint.cast(value).unwrap(), type_hint.clone()),
                    None => {
                        unimplemented!(
                            "Number without type hint not supported: {:?}",
                            self.check_constraint
                        );
                    }
                }
            }
            Value::SingleQuotedString(value) => (quote! { #value }, DataVariantRef::str()),
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
    ///
    /// # Panics
    ///
    /// * If the provided [`ValueWithSpan`](sqlparser::ast::ValueWithSpan) is
    ///   not supported
    fn parse_value_with_span(
        &self,
        value: &sqlparser::ast::ValueWithSpan,
        type_hint: Option<&DataVariantRef>,
    ) -> (proc_macro2::TokenStream, DataVariantRef) {
        self.parse_value(&value.value, type_hint)
    }

    #[allow(clippy::too_many_lines)]
    /// Translates the provided expression to a
    /// [`TokenStream`](proc_macro2::TokenStream)
    pub(super) fn parse(&self, expr: &Expr) -> InternalToken {
        if let Some(validation_error_token) = self.map_expr_to_validation_error(expr) {
            return validation_error_token;
        }

        if self.check_constraint.has_functions(self.database) {
            return quote! {}.into();
        }

        let (internal_token, scoped_columns, returning_type) = self.inner_parse(expr, None);

        if !scoped_columns.is_empty() {
            unimplemented!("Scoped columns not supported");
        }

        assert!(
            returning_type.is_unit_result(),
            "Check constraint expressions must return Unit Result, got {:?}",
            returning_type
        );

        internal_token
    }

    #[allow(clippy::too_many_lines)]
    /// Translates the provided expression to a
    /// [`TokenStream`](proc_macro2::TokenStream)
    fn inner_parse(
        &self,
        expr: &Expr,
        type_hint: Option<&DataVariantRef>,
    ) -> (InternalToken, Vec<&'_ DB::Column>, DataVariantRef) {
        match expr {
            Expr::Function(function) => {
                let (token_stream, returning_type) = self.parse_function(function);
                (token_stream, Vec::new(), returning_type)
            }
            Expr::Cast { kind, expr, data_type: _, format } => {
                verify_cast_kind(kind);
                if format.is_some() {
                    unimplemented!("Format not supported");
                }
                self.inner_parse(expr, type_hint)
            }
            Expr::Nested(expr) => self.inner_parse(expr, type_hint),
            Expr::Identifier(ident) => {
                let column = self.column(&ident.value);
                (
                    self.formatted_column(column, false).into(),
                    vec![column],
                    column
                        .external_postgres_type(self.workspace, self.database)
                        .expect(&format!(
                            "Failed to get type for column `{}.{}` ({})",
                            column.table(self.database).table_name(),
                            column.column_name(),
                            column.normalized_data_type(self.database)
                        ))
                        .into(),
                )
            }
            Expr::BinaryOp { left, op, right } => {
                match op {
                    BinaryOperator::And => {
                        let (left, left_scoped_columns, left_returning_type) =
                            self.inner_parse(left, None);
                        let (right, right_scoped_columns, right_returning_type) =
                            self.inner_parse(right, None);
                        if !left_scoped_columns.is_empty() || !right_scoped_columns.is_empty() {
                            unimplemented!("Scoped columns not supported");
                        }
                        if left_returning_type.is_bool() && right_returning_type.is_bool() {
                            (
                                quote! {
                                    #left && #right
                                }
                                .into(),
                                Vec::new(),
                                DataVariantRef::bool(),
                            )
                        } else if left_returning_type.is_unit_result()
                            && right_returning_type.is_unit_result()
                            && left_returning_type == right_returning_type
                        {
                            (
                                quote! {
                                    #left.and_then(|_| #right)
                                }
                                .into(),
                                Vec::new(),
                                left_returning_type,
                            )
                        } else {
                            unimplemented!("Unsupported binary operation");
                        }
                    }
                    BinaryOperator::Or => {
                        let (left, left_scoped_columns, left_returning_type) =
                            self.inner_parse(left, None);
                        let (right, right_scoped_columns, right_returning_type) =
                            self.inner_parse(right, None);
                        if !left_scoped_columns.is_empty() || !right_scoped_columns.is_empty() {
                            unimplemented!("Scoped columns not supported");
                        }
                        if left_returning_type.is_bool() && right_returning_type.is_bool() {
                            (
                                quote! {
                                    #left || #right
                                }
                                .into(),
                                Vec::new(),
                                DataVariantRef::bool(),
                            )
                        } else if left_returning_type.is_unit_result()
                            && right_returning_type.is_unit_result()
                            && left_returning_type == right_returning_type
                        {
                            (
                                quote! {
                                    #left.or_else(|_| #right)
                                }
                                .into(),
                                Vec::new(),
                                left_returning_type,
                            )
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
                        let (left, _, left_returning_type) = self.inner_parse(left, None);
                        let (right, _, right_returning_type) =
                            self.inner_parse(right, Some(&left_returning_type));
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
                        (
                            quote! {
                                #left #operator_symbol #right
                            }
                            .into(),
                            Vec::new(),
                            DataVariantRef::bool(),
                        )
                    }
                    BinaryOperator::Plus
                    | BinaryOperator::Minus
                    | BinaryOperator::Multiply
                    | BinaryOperator::Divide
                    | BinaryOperator::Modulo => {
                        let (left, _, left_returning_type) = self.inner_parse(left, type_hint);
                        let (right, _, right_returning_type) = self.inner_parse(right, type_hint);
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
                            (
                                quote! {
                                    #left #operator_symbol #right
                                }
                                .into(),
                                Vec::new(),
                                left_returning_type,
                            )
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
                let (token_stream, returning_type) = self.parse_value_with_span(value, type_hint);
                (token_stream.into(), Vec::new(), returning_type)
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
