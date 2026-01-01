//! Implementations for [`GenericDB`] relative to sqlparser structures.

use std::{path::Path, rc::Rc};

use sqlparser::{
    ast::{
        CheckConstraint, ColumnDef, ColumnOption, CreateFunction, CreateTable, Expr,
        ForeignKeyConstraint, IndexColumn, OrderByExpr, OrderByOptions, Statement, TableConstraint,
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
    utils::columns_in_expression,
};

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
    /// Helper function to process check constraints.
    fn process_check_constraint(
        check_expr: &Expr,
        create_table: &Rc<CreateTable>,
        table_metadata: &TableMetadata<CreateTable>,
        builder: &GenericDBBuilder<
            CreateTable,
            TableAttribute<CreateTable, ColumnDef>,
            TableAttribute<CreateTable, UniqueConstraint>,
            TableAttribute<CreateTable, ForeignKeyConstraint>,
            CreateFunction,
            TableAttribute<CreateTable, CheckConstraint>,
        >,
    ) -> Result<
        (Vec<Rc<<Self as DatabaseLike>::Column>>, Vec<Rc<<Self as DatabaseLike>::Function>>),
        crate::errors::Error,
    > {
        let columns_in_expression = columns_in_expression::<Self>(
            check_expr,
            &create_table.name.to_string(),
            table_metadata.column_rc_slice(),
        )?;
        let functions_in_expression = functions_in_expression::functions_in_expression::<Self>(
            check_expr,
            builder.function_rc_vec().as_slice(),
        );
        Ok((columns_in_expression, functions_in_expression))
    }

    /// Helper function to create a unique constraint expression from columns.
    fn create_unique_expression(columns: &[IndexColumn]) -> Expr {
        let expression_string = format!(
            "({})",
            columns.iter().map(|ident| ident.column.to_string()).collect::<Vec<_>>().join(", ")
        );
        Parser::new(&sqlparser::dialect::GenericDialect {})
            .try_with_sql(expression_string.as_str())
            .expect("Failed to parse unique constraint expression")
            .parse_expr()
            .expect("No expression found in parsed unique constraint")
    }

    /// Helper function to process unique constraints.
    fn process_unique_constraint(
        unique_constraint: UniqueConstraint,
        create_table: &Rc<CreateTable>,
    ) -> (
        Rc<TableAttribute<CreateTable, UniqueConstraint>>,
        UniqueIndexMetadata<TableAttribute<CreateTable, UniqueConstraint>>,
    ) {
        let unique_index = Rc::new(TableAttribute::new(create_table.clone(), unique_constraint));
        let expression = Self::create_unique_expression(&unique_index.attribute().columns);
        let unique_index_metadata = UniqueIndexMetadata::new(expression, create_table.clone());
        (unique_index, unique_index_metadata)
    }

    /// Helper function to process column options.
    fn process_column_options(
        column: &Rc<TableAttribute<CreateTable, ColumnDef>>,
        create_table: &Rc<CreateTable>,
        table_metadata: &mut TableMetadata<CreateTable>,
        mut builder: GenericDBBuilder<
            CreateTable,
            TableAttribute<CreateTable, ColumnDef>,
            TableAttribute<CreateTable, UniqueConstraint>,
            TableAttribute<CreateTable, ForeignKeyConstraint>,
            CreateFunction,
            TableAttribute<CreateTable, CheckConstraint>,
        >,
    ) -> Result<
        GenericDBBuilder<
            CreateTable,
            TableAttribute<CreateTable, ColumnDef>,
            TableAttribute<CreateTable, UniqueConstraint>,
            TableAttribute<CreateTable, ForeignKeyConstraint>,
            CreateFunction,
            TableAttribute<CreateTable, CheckConstraint>,
        >,
        crate::errors::Error,
    > {
        for option in &column.attribute().options {
            match option.option.clone() {
                ColumnOption::Check(check_constraint) => {
                    let check_rc = Rc::new(TableAttribute::new(
                        create_table.clone(),
                        check_constraint.clone(),
                    ));
                    table_metadata.add_check_constraint(check_rc.clone());
                    let (columns_in_expression, functions_in_expression) =
                        Self::process_check_constraint(
                            &check_constraint.expr,
                            create_table,
                            table_metadata,
                            &builder,
                        )?;
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
                    let fk = Rc::new(TableAttribute::new(create_table.clone(), foreign_key));
                    table_metadata.add_foreign_key(fk.clone());
                    builder = builder.add_foreign_key(fk, ());
                }
                ColumnOption::Unique(mut unique_constraint) => {
                    unique_constraint.columns.push(IndexColumn {
                        column: OrderByExpr {
                            expr: Expr::Identifier(column.attribute().name.clone()),
                            options: OrderByOptions::default(),
                            with_fill: None,
                        },
                        operator_class: None,
                    });
                    let (unique_index, unique_index_metadata) =
                        Self::process_unique_constraint(unique_constraint, create_table);
                    table_metadata.add_unique_index(unique_index.clone());
                    builder = builder.add_unique_index(unique_index, unique_index_metadata);
                }
                ColumnOption::PrimaryKey(_) => {
                    // From the primary key constraint we also create an associated unique index,
                    // since primary keys also have an associated unique index.
                    let primary_key_unique_constraint = UniqueConstraint {
                        name: None,
                        index_name: None,
                        index_type_display: sqlparser::ast::KeyOrIndexDisplay::None,
                        index_type: None,
                        columns: vec![IndexColumn {
                            column: OrderByExpr {
                                expr: Expr::Identifier(column.attribute().name.clone()),
                                options: OrderByOptions::default(),
                                with_fill: None,
                            },
                            operator_class: None,
                        }],
                        index_options: vec![],
                        characteristics: None,
                        nulls_distinct: sqlparser::ast::NullsDistinctOption::None,
                    };

                    let (unique_index, unique_index_metadata) = Self::process_unique_constraint(
                        primary_key_unique_constraint,
                        create_table,
                    );
                    table_metadata.add_unique_index(unique_index.clone());
                    builder = builder.add_unique_index(unique_index, unique_index_metadata);

                    table_metadata.set_primary_key(vec![column.clone()]);
                }
                _ => {}
            }
        }
        Ok(builder)
    }

    /// Helper function to process table constraints.
    fn process_table_constraints(
        constraints: &[TableConstraint],
        create_table: &Rc<CreateTable>,
        table_metadata: &mut TableMetadata<CreateTable>,
        mut builder: GenericDBBuilder<
            CreateTable,
            TableAttribute<CreateTable, ColumnDef>,
            TableAttribute<CreateTable, UniqueConstraint>,
            TableAttribute<CreateTable, ForeignKeyConstraint>,
            CreateFunction,
            TableAttribute<CreateTable, CheckConstraint>,
        >,
    ) -> Result<
        GenericDBBuilder<
            CreateTable,
            TableAttribute<CreateTable, ColumnDef>,
            TableAttribute<CreateTable, UniqueConstraint>,
            TableAttribute<CreateTable, ForeignKeyConstraint>,
            CreateFunction,
            TableAttribute<CreateTable, CheckConstraint>,
        >,
        crate::errors::Error,
    > {
        for constraint in constraints {
            match constraint {
                TableConstraint::Unique(uc) => {
                    let (unique_index, unique_index_metadata) =
                        Self::process_unique_constraint(uc.clone(), create_table);
                    table_metadata.add_unique_index(unique_index.clone());
                    builder = builder.add_unique_index(unique_index, unique_index_metadata);
                }
                TableConstraint::ForeignKey(fk) => {
                    // Validate host columns exist
                    for col_ident in &fk.columns {
                        let column_exists = table_metadata
                            .column_rcs()
                            .any(|col| col.column_name() == col_ident.value.as_str());

                        if !column_exists {
                            return Err(crate::errors::Error::HostColumnNotFoundForForeignKey {
                                host_column: col_ident.value.clone(),
                                host_table: create_table.name.to_string(),
                            });
                        }
                    }

                    // Validate referenced table exists or is current table (self-referential)
                    let referenced_table_name = fk.foreign_table.to_string();

                    let referenced_table = builder
                        .tables()
                        .iter()
                        .map(|(t, _)| t.as_ref())
                        .chain(std::iter::once(create_table.as_ref()))
                        .find(|t| t.name.to_string() == referenced_table_name);

                    let referenced_table = if let Some(table) = referenced_table {
                        table
                    } else {
                        return Err(crate::errors::Error::ReferencedTableNotFoundForForeignKey {
                            referenced_table: referenced_table_name.clone(),
                            host_table: create_table.name.to_string(),
                        });
                    };

                    // Validate referenced columns exist
                    for ref_col_ident in &fk.referred_columns {
                        let column_exists = referenced_table
                            .columns
                            .iter()
                            .any(|col| col.name.value.as_str() == ref_col_ident.value.as_str());

                        if !column_exists {
                            return Err(
                                crate::errors::Error::ReferencedColumnNotFoundForForeignKey {
                                    referenced_column: ref_col_ident.value.clone(),
                                    referenced_table: referenced_table_name.clone(),
                                    host_table: create_table.name.to_string(),
                                },
                            );
                        }
                    }

                    let fk = Rc::new(TableAttribute::new(create_table.clone(), fk.clone()));
                    table_metadata.add_foreign_key(fk.clone());
                    builder = builder.add_foreign_key(fk, ());
                }
                TableConstraint::Check(check) => {
                    let check_rc =
                        Rc::new(TableAttribute::new(create_table.clone(), check.clone()));
                    table_metadata.add_check_constraint(check_rc.clone());
                    let (columns_in_expression, functions_in_expression) =
                        Self::process_check_constraint(
                            &check.expr,
                            create_table,
                            table_metadata,
                            &builder,
                        )?;
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
                        let Expr::Identifier(column_name) = &col_name.column.expr else {
                            unreachable!(
                                "Unexpected expression in primary key column: {:?}",
                                col_name
                            )
                        };
                        primary_key_columns.extend(
                            table_metadata
                                .column_rcs()
                                .filter(|col: &&Rc<TableAttribute<CreateTable, ColumnDef>>| {
                                    col.column_name() == column_name.value.as_str()
                                })
                                .cloned(),
                        );
                    }

                    // From the primary key constraint we also create an associated unique index,
                    // since primary keys also have an associated unique index.
                    let primary_key_unique_constraint = UniqueConstraint {
                        name: pk.name.clone(),
                        index_name: None,
                        index_type_display: sqlparser::ast::KeyOrIndexDisplay::None,
                        index_type: None,
                        columns: pk.columns.clone(),
                        index_options: vec![],
                        characteristics: pk.characteristics,
                        nulls_distinct: sqlparser::ast::NullsDistinctOption::None,
                    };

                    let (unique_index, unique_index_metadata) = Self::process_unique_constraint(
                        primary_key_unique_constraint,
                        create_table,
                    );
                    table_metadata.add_unique_index(unique_index.clone());
                    builder = builder.add_unique_index(unique_index, unique_index_metadata);

                    table_metadata.set_primary_key(primary_key_columns);
                }
                _ => {}
            }
        }
        Ok(builder)
    }

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
    ///
    /// # Errors
    ///
    /// Returns an error if a check constraint references an unknown column.
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn from_statements(
        statements: Vec<Statement>,
        catalog_name: String,
    ) -> Result<Self, crate::errors::Error> {
        let mut builder = GenericDBBuilder::new(catalog_name);

        for statement in statements {
            match statement {
                Statement::CreateFunction(create_function) => {
                    builder = builder.add_function(Rc::new(create_function), ());
                }
                Statement::CreateTable(create_table) => {
                    let create_table = Rc::new(create_table);
                    let mut table_metadata: TableMetadata<CreateTable> = TableMetadata::default();

                    // Add all columns to metadata
                    for column in create_table.columns.clone() {
                        let column_rc = Rc::new(TableAttribute::new(create_table.clone(), column));
                        table_metadata.add_column(column_rc.clone());
                    }

                    // Process column options and add columns to builder
                    for column in table_metadata.clone().column_rcs() {
                        builder = Self::process_column_options(
                            column,
                            &create_table,
                            &mut table_metadata,
                            builder,
                        )?;
                        builder = builder.add_column(column.clone(), ());
                    }

                    // Process table constraints
                    builder = Self::process_table_constraints(
                        &create_table.constraints,
                        &create_table,
                        &mut table_metadata,
                        builder,
                    )?;

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
                | Statement::CreateOperatorFamily(_)
                | Statement::CreateType { .. }
                | Statement::CreateExtension(_)
                | Statement::CreateIndex(_)
                | Statement::CreateTrigger(_) => {
                    // At the moment, we ignore these CREATE statements.
                }
                _ => {
                    unimplemented!("Unsupported statement found: {statement:?}");
                }
            }
        }

        Ok(builder.into())
    }
}

impl TryFrom<&str> for ParserDB {
    type Error = crate::errors::Error;

    fn try_from(sql: &str) -> Result<Self, Self::Error> {
        let dialect = sqlparser::dialect::GenericDialect {};
        let mut parser = sqlparser::parser::Parser::new(&dialect).try_with_sql(sql)?;
        let statements = parser.parse_statements()?;
        Self::from_statements(statements, "unknown_catalog".to_string())
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
            } else if let Some(extension) = path.extension()
                && extension == "sql"
                && path.file_name().unwrap() != "down.sql"
            {
                sql_files.push(path);
            }
        }
    } else if let Some(extension) = path.extension()
        && extension == "sql"
    {
        sql_files.push(path.to_path_buf());
    }
    sql_files
}

impl TryFrom<&Path> for ParserDB {
    type Error = crate::errors::Error;

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
    type Error = crate::errors::Error;

    fn try_from(paths: &[&Path]) -> Result<Self, Self::Error> {
        let mut statements = Vec::new();
        for path in paths {
            if !path.exists() {
                return Err(ParserError::TokenizerError(format!(
                    "Path does not exist: {}",
                    path.display()
                ))
                .into());
            }

            let mut sql_paths = search_sql_documents(path);
            sql_paths.sort_unstable();

            for sql_path in sql_paths {
                let sql_content = std::fs::read_to_string(&sql_path)
                    .map_err(|e| ParserError::TokenizerError(e.to_string()))?;
                let mut parser = sqlparser::parser::Parser::new(&PostgreSqlDialect {})
                    .try_with_sql(&sql_content)?;
                statements.extend(parser.parse_statements()?);
            }
        }

        Self::from_statements(statements, "unknown_catalog".to_string())
    }
}
