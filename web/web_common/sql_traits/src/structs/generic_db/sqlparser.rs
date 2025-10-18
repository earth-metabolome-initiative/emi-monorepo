//! Implementations for [`GenericDB`] relative to sqlparser structures.

use std::rc::Rc;

use common_traits::prelude::Builder;
use sqlparser::{
    ast::{ColumnDef, ColumnOption, CreateFunction, CreateTable, Expr, Statement, TableConstraint},
    parser::Parser,
};

use crate::{
    structs::{
        GenericDB, TableAttribute, TableMetadata, generic_db::GenericDBBuilder,
        metadata::UniqueIndexMetadata,
    },
    traits::column::ColumnLike,
};

/// A type alias for a `GenericDB` specialized for `sqlparser`'s `CreateTable`.
pub type ParserDB = GenericDB<CreateTable, CreateFunction>;

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
                    builder = builder.add_function(create_function, ());
                }
                Statement::CreateTable(create_table) => {
                    let create_table = Rc::new(create_table);
                    let mut table_metadata = TableMetadata::default();
                    for column in create_table.columns.clone() {
                        let column_rc =
                            Rc::new(TableAttribute::new(create_table.clone(), column.clone()));
                        table_metadata.add_column(column_rc.clone());
                        for option in column.options.iter() {
                            match &option.option {
                                ColumnOption::Check(check_constraint) => {
                                    table_metadata.add_check_constraint(Rc::new(
                                        TableAttribute::new(
                                            create_table.clone(),
                                            check_constraint.clone(),
                                        ),
                                    ));
                                }
                                ColumnOption::ForeignKey(foreign_key) => {
                                    let fk = Rc::new(TableAttribute::new(
                                        create_table.clone(),
                                        foreign_key.clone(),
                                    ));
                                    table_metadata.add_foreign_key(fk.clone());
                                    builder = builder.add_foreign_key(fk, ());
                                }
                                ColumnOption::Unique(unique_constraint) => {
                                    let unique_index = Rc::new(TableAttribute::new(
                                        create_table.clone(),
                                        unique_constraint.clone(),
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
                                            .expect(
                                                "No expression found in parsed unique constraint",
                                            );
                                    let unique_index_metadata =
                                        UniqueIndexMetadata::new(expression, create_table.clone());
                                    table_metadata.add_unique_index(unique_index.clone());
                                    builder = builder
                                        .add_unique_index(unique_index, unique_index_metadata);
                                }
                                ColumnOption::PrimaryKey(_) => {
                                    table_metadata.set_primary_key(vec![column_rc.clone()]);
                                }
                                _ => {}
                            }
                        }
                        builder = builder.add_column(column_rc, ());
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
                                table_metadata.add_check_constraint(Rc::new(TableAttribute::new(
                                    create_table.clone(),
                                    check.clone(),
                                )));
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
                _ => {
                    unimplemented!(
                        "Only CREATE TABLE and CREATE FUNCTION statements are supported, found: {statement:?}"
                    );
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
