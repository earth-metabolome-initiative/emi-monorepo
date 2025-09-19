//! Submodule defining a schema for the translation between `PostgreSQL` and
//! `SQLite`.

use sqlparser::{
    ast::{
        BeginEndStatements, CreateFunction, CreateFunctionBody, CreateTable, DollarQuotedString,
        Expr, ReturnStatement, ReturnStatementValue, Statement, Value, ValueWithSpan,
        helpers::attached_token::AttachedToken,
    },
    keywords::Keyword,
    tokenizer::{Token, TokenWithSpan, Word},
};

use crate::traits::table_like::TableLike;

/// Trait to define a schema for the translation between `PostgreSQL` and
/// `SQLite`.
pub trait Schema {
    /// Returns a reference to a table defined in the schema by its name, if it
    /// exists.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the table to be searched.
    fn table(&self, name: &str) -> Option<&CreateTable>;

    /// Returns whether the table with the given name has a primary key column
    /// of type `UUID`, if the table exists.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the table to be searched.
    fn table_has_uuid_pk(&self, name: &str) -> Option<bool> {
        let table = self.table(name)?;
        Some(table.has_uuid_pk())
    }

    /// Returns a reference to a function defined in the schema by its name, if
    /// it exists.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function to be searched.
    fn function(&self, name: &str) -> Option<&CreateFunction>;

    /// Returns a reference to the body of a function defined in the schema by
    /// its name, if it exists.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function to be searched.
    fn function_body(&self, name: &str) -> Option<BeginEndStatements> {
        let function = self.function(name)?;
        let function_body = function.function_body.as_ref()?;
        match function_body {
            CreateFunctionBody::AsBeginEnd(body) => Some(body.clone()),
            CreateFunctionBody::AsBeforeOptions(Expr::Value(ValueWithSpan {
                value: Value::DollarQuotedString(DollarQuotedString { value: maybe_body, tag: _ }),
                span: _,
            })) => {
                // We strip spaces and semicolons from the body.
                let maybe_body = maybe_body.trim().trim_end_matches(';').trim();

                // We strip the `BEGIN` and `END` tokens, as they are not part of the
                // actual body.
                let maybe_body = maybe_body
                    .strip_prefix("BEGIN")
                    .expect("Function body should start with BEGIN")
                    .strip_suffix("END")
                    .expect("Function body should end with END")
                    .trim();

                let mut statements = sqlparser::parser::Parser::parse_sql(
                    &sqlparser::dialect::PostgreSqlDialect {},
                    maybe_body,
                )
                .unwrap_or_else(|_| panic!("Failed to parse function body: {maybe_body}"));

                // The function body may end with a `RETURN NEW;` or `RETURN OLD;` statement.
                // If that's the case, we remove it.
                if let Some(Statement::Return(ReturnStatement {
                    value: Some(ReturnStatementValue::Expr(expr)),
                })) = statements.last()
                {
                    let string_expr = expr.to_string();
                    if string_expr == "NEW" || string_expr == "OLD" {
                        statements.pop();
                    }
                }

                Some(BeginEndStatements {
                    begin_token: AttachedToken(TokenWithSpan::wrap(Token::Word(Word {
                        value: "BEGIN".into(),
                        quote_style: None,
                        keyword: Keyword::BEGIN,
                    }))),
                    statements,
                    end_token: AttachedToken(TokenWithSpan::wrap(Token::Word(Word {
                        value: "END".into(),
                        quote_style: None,
                        keyword: Keyword::END,
                    }))),
                })
            }
            _ => {
                unimplemented!(
                    "Function body extraction for definition `{function_body:?}` is not yet implemented.",
                )
            }
        }
    }

    /// Adds a function to the schema.
    ///
    /// # Arguments
    ///
    /// * `function` - The function to be added.
    fn add_function(&mut self, function: &CreateFunction);

    /// Adds a table to the schema.
    ///
    /// # Arguments
    /// * `table` - The table to be added.
    fn add_table(&mut self, table: &CreateTable);
}
