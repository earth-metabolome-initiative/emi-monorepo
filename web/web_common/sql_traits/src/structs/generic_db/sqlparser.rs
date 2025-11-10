//! Implementations for [`GenericDB`] relative to sqlparser structures.

use std::{path::Path, rc::Rc};

use common_traits::prelude::Builder;
use csqlv::{CSVSchema, CSVSchemaBuilder, SQLGenerationOptions};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sqlparser::{
    ast::{
        CheckConstraint, ColumnDef, ColumnOption, CreateFunction, CreateTable, Expr,
        ForeignKeyConstraint, IndexColumn, OrderByExpr, Statement, TableConstraint,
        UniqueConstraint, Value, ValueWithSpan,
    },
    dialect::PostgreSqlDialect,
    parser::{Parser, ParserError},
};

use crate::{
    structs::{
        GenericDB, TableAttribute, TableMetadata,
        generic_db::GenericDBBuilder,
        metadata::{CheckMetadata, UniqueIndexMetadata},
    },
    traits::{DatabaseLike, column::ColumnLike},
};

mod columns_in_expression;
mod functions_in_expression;

/// A type alias for a `GenericDB` specialized for `sqlparser`'s `CreateTable`.
pub type ParserDB = GenericDB<
    CreateTable,
    TableAttribute<CreateTable, ColumnDef>,
    TableAttribute<CreateTable, UniqueConstraint>,
    TableAttribute<CreateTable, ForeignKeyConstraint>,
    CreateFunction,
    TableAttribute<CreateTable, CheckConstraint>,
>;

impl ParserDB {
    /// Creates a new `ParserDB` from a vector of SQL statements and a catalog
    /// name.
    ///
    /// # Arguments
    ///
    /// * `statements` - A vector of SQL statements to parse.
    /// * `catalog_name` - The name of the database catalog.
    ///
    /// # Panics
    ///
    /// Panics if a statement other than `CREATE TABLE` or `CREATE FUNCTION` is
    /// encountered, or if the builder fails to build the database.
    pub fn from_statements(statements: Vec<Statement>, catalog_name: String) -> Self {
        let mut builder = GenericDBBuilder::new().catalog_name(catalog_name);

        for statement in statements {
            match statement {
                Statement::CreateFunction(create_function) => {
                    builder = builder.add_function(Rc::new(create_function), ());
                }
                Statement::CreateTable(create_table) => {
                    let create_table = Rc::new(create_table);
                    let mut table_metadata: TableMetadata<CreateTable> = TableMetadata::default();
                    for column in create_table.columns.clone() {
                        let column_rc = Rc::new(TableAttribute::new(create_table.clone(), column));
                        table_metadata.add_column(column_rc.clone());
                    }
                    for column in table_metadata.clone().column_rcs() {
                        for option in column.attribute().options.iter() {
                            match option.option.clone() {
                                ColumnOption::Check(check_constraint) => {
                                    let check_rc = Rc::new(TableAttribute::new(
                                        create_table.clone(),
                                        check_constraint.clone(),
                                    ));
                                    table_metadata.add_check_constraint(check_rc.clone());
                                    let columns_in_expression: Vec<
                                        Rc<<Self as DatabaseLike>::Column>,
                                    > = columns_in_expression::columns_in_expression::<Self>(
                                        &check_constraint.expr,
                                        table_metadata.column_rc_slice(),
                                    );
                                    let functions_in_expression: Vec<
                                        Rc<<Self as DatabaseLike>::Function>,
                                    > = functions_in_expression::functions_in_expression::<Self>(
                                        &check_constraint.expr,
                                        builder.function_rc_vec().as_slice(),
                                    );
                                    builder = builder.add_check_constraint(
                                        check_rc,
                                        CheckMetadata::new(
                                            *check_constraint.expr.clone(),
                                            create_table.clone(),
                                            columns_in_expression,
                                            functions_in_expression,
                                        ),
                                    );
                                }
                                ColumnOption::ForeignKey(mut foreign_key) => {
                                    foreign_key.columns.push(column.attribute().name.clone());
                                    let fk = Rc::new(TableAttribute::new(
                                        create_table.clone(),
                                        foreign_key,
                                    ));
                                    table_metadata.add_foreign_key(fk.clone());
                                    builder = builder.add_foreign_key(fk, ());
                                }
                                ColumnOption::Unique(mut unique_constraint) => {
                                    unique_constraint.columns.push(IndexColumn {
                                        column: OrderByExpr {
                                            expr: Expr::Identifier(column.attribute().name.clone()),
                                            options: Default::default(),
                                            with_fill: None,
                                        },
                                        operator_class: None,
                                    });
                                    let unique_index = Rc::new(TableAttribute::new(
                                        create_table.clone(),
                                        unique_constraint,
                                    ));
                                    let expression_string = format!(
                                        "({})",
                                        unique_index
                                            .attribute()
                                            .columns
                                            .iter()
                                            .map(|ident| ident.column.to_string())
                                            .collect::<Vec<_>>()
                                            .join(", ")
                                    );
                                    let expression =
                                        Parser::new(&sqlparser::dialect::GenericDialect {})
                                            .try_with_sql(expression_string.as_str())
                                            .expect("Failed to parse unique constraint expression")
                                            .parse_expr()
                                            .unwrap();
                                    let unique_index_metadata =
                                        UniqueIndexMetadata::new(expression, create_table.clone());
                                    table_metadata.add_unique_index(unique_index.clone());
                                    builder = builder
                                        .add_unique_index(unique_index, unique_index_metadata);
                                }
                                ColumnOption::PrimaryKey(_) => {
                                    table_metadata.set_primary_key(vec![column.clone()]);
                                }
                                _ => {}
                            }
                        }
                        builder = builder.add_column(column.clone(), ());
                    }

                    for constraint in &create_table.constraints {
                        match constraint {
                            TableConstraint::Unique(uc) => {
                                let unique_index =
                                    Rc::new(TableAttribute::new(create_table.clone(), uc.clone()));
                                let expression_string = format!(
                                    "({})",
                                    unique_index
                                        .attribute()
                                        .columns
                                        .iter()
                                        .map(|ident| ident.column.to_string())
                                        .collect::<Vec<_>>()
                                        .join(", ")
                                );
                                let expression =
                                    Parser::new(&sqlparser::dialect::GenericDialect {})
                                        .try_with_sql(expression_string.as_str())
                                        .expect("Failed to parse unique constraint expression")
                                        .parse_expr()
                                        .expect("No expression found in parsed unique constraint");
                                let unique_index_metadata =
                                    UniqueIndexMetadata::new(expression, create_table.clone());
                                table_metadata.add_unique_index(unique_index.clone());
                                builder =
                                    builder.add_unique_index(unique_index, unique_index_metadata);
                            }
                            TableConstraint::ForeignKey(fk) => {
                                let fk =
                                    Rc::new(TableAttribute::new(create_table.clone(), fk.clone()));
                                table_metadata.add_foreign_key(fk.clone());
                                builder = builder.add_foreign_key(fk, ());
                            }
                            TableConstraint::Check(check) => {
                                let check_rc = Rc::new(TableAttribute::new(
                                    create_table.clone(),
                                    check.clone(),
                                ));
                                table_metadata.add_check_constraint(check_rc.clone());
                                let columns_in_expression: Vec<Rc<<Self as DatabaseLike>::Column>> =
                                    columns_in_expression::columns_in_expression::<Self>(
                                        &check.expr,
                                        table_metadata.column_rc_slice(),
                                    );
                                let functions_in_expression: Vec<
                                    Rc<<Self as DatabaseLike>::Function>,
                                > = functions_in_expression::functions_in_expression::<Self>(
                                    &check.expr,
                                    builder.function_rc_vec().as_slice(),
                                );
                                builder = builder.add_check_constraint(
                                    check_rc,
                                    CheckMetadata::new(
                                        *check.expr.clone(),
                                        create_table.clone(),
                                        columns_in_expression,
                                        functions_in_expression,
                                    ),
                                );
                            }
                            TableConstraint::PrimaryKey(pk) => {
                                let mut primary_key_columns = Vec::new();
                                for col_name in &pk.columns {
                                    let column_name = match &col_name.column.expr {
                                        Expr::Identifier(ident) => ident,
                                        _ => {
                                            unreachable!(
                                                "Unexpected expression in primary key column: {:?}",
                                                col_name
                                            )
                                        }
                                    };
                                    primary_key_columns.extend(
                                        table_metadata
                                            .column_rcs()
                                            .filter(
                                                |col: &&Rc<
                                                    TableAttribute<CreateTable, ColumnDef>,
                                                >| {
                                                    col.column_name() == column_name.value.as_str()
                                                },
                                            )
                                            .cloned(),
                                    );
                                }
                                table_metadata.set_primary_key(primary_key_columns);
                            }
                            _ => {}
                        }
                    }

                    builder = builder.add_table(create_table, table_metadata);
                }
                Statement::Set(sqlparser::ast::Set::SetTimeZone { local, value }) => {
                    // We currently ignore SET TIME ZONE statements.
                    if local {
                        builder = builder.timezone("LOCAL".to_string());
                    } else if let Expr::Value(ValueWithSpan {
                        value: Value::SingleQuotedString(lit),
                        ..
                    }) = value
                    {
                        builder = builder.timezone(lit);
                    } else {
                        unimplemented!(
                            "Only string literals are supported for SET TIME ZONE, found: {value:?}"
                        );
                    }
                }
                Statement::CreateOperator(_)
                | Statement::CreateOperatorClass(_)
                | Statement::CreateOperatorFamily(_) => {
                    // At the moment, we ignore CREATE OPERATOR statements.
                }
                Statement::CreateType { .. } => {
                    // At the moment, we ignore CREATE TYPE statements.
                }
                Statement::CreateExtension(_) => {
                    // At the moment, we ignore CREATE EXTENSION statements.
                }
                Statement::CreateIndex(_) => {
                    // At the moment, we ignore CREATE INDEX statements.
                }
                Statement::CreateTrigger(_) => {
                    // At the moment, we ignore CREATE TRIGGER statements.
                }
                _ => {
                    unimplemented!("Unsupported statement found: {statement:?}");
                }
            }
        }

        builder.build().expect("Failed to build ParserDB")
    }
}

impl TryFrom<&str> for ParserDB {
    type Error = sqlparser::parser::ParserError;

    fn try_from(sql: &str) -> Result<Self, Self::Error> {
        let dialect = sqlparser::dialect::GenericDialect {};
        let mut parser = sqlparser::parser::Parser::new(&dialect).try_with_sql(sql)?;
        let statements = parser.parse_statements()?;
        Ok(Self::from_statements(statements, "unknown_catalog".to_string()))
    }
}

fn search_sql_documents(path: &Path) -> Vec<std::path::PathBuf> {
    let mut sql_files = Vec::new();
    if path.is_dir() {
        for entry in std::fs::read_dir(path).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            if path.is_dir() {
                sql_files.extend(search_sql_documents(&path));
            } else if let Some(extension) = path.extension() {
                if extension == "sql" && path.file_name().unwrap() != "down.sql" {
                    sql_files.push(path);
                }
            }
        }
    } else if let Some(extension) = path.extension() {
        if extension == "sql" {
            sql_files.push(path.to_path_buf());
        }
    }
    sql_files
}

impl TryFrom<&Path> for ParserDB {
    type Error = sqlparser::parser::ParserError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let mut comulative_sql = String::new();
        let mut sql_files = search_sql_documents(path);
        sql_files.sort_unstable();
        for sql_file in sql_files {
            let sql_content = std::fs::read_to_string(&sql_file)
                .map_err(|e| ParserError::TokenizerError(e.to_string()))?;
            comulative_sql.push_str(&sql_content);
        }
        Self::try_from(comulative_sql.as_str())
    }
}

impl TryFrom<&[&Path]> for ParserDB {
    type Error = sqlparser::parser::ParserError;

    fn try_from(paths: &[&Path]) -> Result<Self, Self::Error> {
        let mut sql_documents = Vec::new();

        let mut sql_files = Vec::new();
        for path in paths {
            if !path.exists() {
                return Err(ParserError::TokenizerError(format!(
                    "Path does not exist: {}",
                    path.display()
                )));
            }

            let mut sql_paths = search_sql_documents(path);
            sql_paths.sort_unstable();
            sql_files.extend(sql_paths);

            let schema: CSVSchema =
                CSVSchemaBuilder::default().include_gz().from_dir(path).map_err(|e| {
                    ParserError::TokenizerError(format!(
                        "Failed to build CSV schema from path {}: {}",
                        path.display(),
                        e
                    ))
                })?;

            sql_documents.push(schema.to_sql(&SQLGenerationOptions::default()).map_err(|e| {
                ParserError::TokenizerError(format!(
                    "Failed to convert CSV schema to SQL from path {}: {}",
                    path.display(),
                    e
                ))
            })?);
        }

        // First, we handle the IO part sequentially, as it is a I/O-bound task.
        for sql_document in sql_files {
            let sql_content = std::fs::read_to_string(&sql_document)
                .map_err(|e| ParserError::TokenizerError(e.to_string()))?;
            sql_documents.push(sql_content);
        }

        let statements = sql_documents
            .into_par_iter()
            .map(|sql_file: String| {
                let mut parser = sqlparser::parser::Parser::new(&PostgreSqlDialect {})
                    .try_with_sql(&sql_file)?;
                parser.parse_statements()
            })
            .collect::<Result<Vec<Vec<Statement>>, ParserError>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<Statement>>();

        Ok(Self::from_statements(statements, "unknown_catalog".to_string()))
    }
}
