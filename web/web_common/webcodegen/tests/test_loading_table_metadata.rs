//! Test suite for table metadata loading
use diesel::pg::PgConnection;

mod utils;

use utils::*;
use webcodegen::{errors::WebCodeGenError, *};

async fn test_code_generation_methods(conn: &mut PgConnection) -> Result<(), WebCodeGenError> {
    // We create the `./tests/ui` directory if it does not exist
    std::fs::create_dir_all("tests/ui")?;

    let builder = trybuild::TestCases::new();
    SQLFunction::write_all(conn, "tests/ui/sql_functions.rs")?;
    add_main_to_file("tests/ui/sql_functions.rs");
    builder.pass("tests/ui/sql_functions.rs");

    // SQLOperator::write_all(conn, "tests/ui/sql_operators.rs")?;
    // add_main_to_file("tests/ui/sql_operators.rs");
    // builder.pass("tests/ui/sql_operators.rs");

    Ok(())
}

async fn test_check_constraints(
    database_name: &str,
    conn: &mut PgConnection,
) -> Result<(), WebCodeGenError> {
    let users = Table::load(conn, "users", None, &database_name).unwrap();

    let table_check_constraint = users.check_constraints(conn)?;

    assert_eq!(
        table_check_constraint.len(),
        6,
        "Expected 6 check constraint, got: {:?}",
        table_check_constraint
    );

    let user_name_column = users.column_by_name(conn, "username")?;

    assert_eq!(user_name_column.column_name, "username");

    Ok(())
}

async fn test_create_roles_tables(
    database_name: &str,
    conn: &mut PgConnection,
) -> Result<(), WebCodeGenError> {
    let query_result = Table::create_roles_tables(conn, &database_name, None);

    assert!(
        query_result.is_ok(),
        "Failed to create roles tables using SQL, got error: {query_result:?}"
    );
    Ok(())
}

#[tokio::test]
async fn test_user_table() {
    let (docker, mut conn, database_name) =
        setup_database_with_default_migrations("test_table_properties").await.unwrap();

    test_create_roles_tables(&database_name, &mut conn).await.unwrap();

    // We attempt to create the update triggers for the tables
    // that have an `updated_at` column

    Table::create_update_triggers(&mut conn, &database_name, None).unwrap();

    AuthorizationFunctionBuilder::default()
        .add_childless_table(Table::load(&mut conn, "users", None, &database_name).unwrap())
        .create_authorization_functions_and_triggers(&mut conn, &database_name, None)
        .unwrap();

    test_code_generation_methods(&mut conn).await.unwrap();

    // We try to load all elements of each type, so to ensure
    // that the structs are actually compatible with the schema
    // of PostgreSQL
    let all_tables = Table::load_all(&mut conn, &database_name, None).unwrap();
    assert!(!all_tables.is_empty());

    // We check that all tables that have the `updated_at` column also have the
    // trigger to update the column
    all_tables.iter().for_each(|table| {
        if table.has_updated_at_column(&mut conn).unwrap() {
            assert!(
                table.updated_at_trigger_exists(&mut conn).unwrap(),
                "Table {table_name} does not have the updated_at trigger",
                table_name = table.table_name
            );
        }
    });

    // We check specifically that the `teams` table has the `updated_at` column
    // and the trigger to update it
    let teams = Table::load(&mut conn, "teams", None, &database_name).unwrap();
    assert!(
        teams.has_updated_at_column(&mut conn).unwrap(),
        "Table teams does not have the updated_at column, but has columns: {:?}",
        teams.columns(&mut conn).unwrap()
    );
    assert!(teams.updated_at_trigger_exists(&mut conn).unwrap());

    let _all_columns = Column::load_all(&mut conn);

    let _all_table_constraints = TableConstraint::load_all(&mut conn);
    let _all_key_column_usage = KeyColumnUsage::load_all_key_column_usages(&mut conn);
    let _all_referential_constraints =
        ReferentialConstraint::load_all_referential_constraints(&mut conn);
    let _all_constraint_column_usage =
        ConstraintColumnUsage::load_all_constraint_column_usages(&mut conn);
    let _all_check_constraint = CheckConstraint::load_all_check_constraints(&mut conn);
    let _all_constraint_table_usage = ConstraintTableUsage::load_all(&mut conn);
    let _all_domain_constraint = DomainConstraint::load_all_domain_constraints(&mut conn);

    let users = Table::load(&mut conn, "users", None, &database_name).unwrap();

    test_check_constraints(&database_name, &mut conn).await.unwrap();

    let original_user_id_column = users.column_by_name(&mut conn, "id").unwrap();

    let columns: Result<Vec<Column>, WebCodeGenError> = users.columns(&mut conn);

    assert!(columns.is_ok());
    let columns = columns.unwrap();
    assert_eq!(columns.len(), 4);

    let primary_key_columns: Result<Vec<Column>, WebCodeGenError> =
        users.primary_key_columns(&mut conn);

    assert!(primary_key_columns.is_ok());
    let primary_key_columns = primary_key_columns.unwrap();
    assert_eq!(primary_key_columns.len(), 1);

    let unique_columns: Result<Vec<Vec<Column>>, WebCodeGenError> = users.unique_columns(&mut conn);

    assert!(unique_columns.is_ok());
    let unique_columns = unique_columns.unwrap();

    assert_eq!(unique_columns.len(), 3);
    assert_eq!(unique_columns[0].len(), 1);
    assert_eq!(unique_columns[1].len(), 2);
    assert_eq!(unique_columns[2].len(), 1);

    let composite_users = Table::load(&mut conn, "composite_users", None, &database_name).unwrap();

    let columns: Result<Vec<Column>, WebCodeGenError> = composite_users.columns(&mut conn);
    let primary_key_columns: Result<Vec<Column>, WebCodeGenError> =
        composite_users.primary_key_columns(&mut conn);

    assert!(columns.is_ok());
    let columns = columns.unwrap();
    assert_eq!(columns.len(), 8);

    assert!(primary_key_columns.is_ok());
    let primary_key_columns = primary_key_columns.unwrap();
    assert_eq!(primary_key_columns.len(), 2);

    let primary_id_column = composite_users.column_by_name(&mut conn, "primary_id").unwrap();
    assert_eq!(primary_id_column.column_name, "primary_id");
    assert!(primary_id_column.is_foreign_key(&mut conn));

    let (foreign_table, user_id_column) =
        primary_id_column.foreign_table(&mut conn).unwrap().unwrap();
    assert_eq!(foreign_table, users);
    assert_eq!(user_id_column, original_user_id_column);

    let secondary_id_column = composite_users.column_by_name(&mut conn, "secondary_id").unwrap();
    assert_eq!(secondary_id_column.column_name, "secondary_id");
    assert!(secondary_id_column.is_foreign_key(&mut conn));

    let (foreign_table, user_id_column) =
        secondary_id_column.foreign_table(&mut conn).unwrap().unwrap();
    assert_eq!(foreign_table, users);
    assert_eq!(user_id_column, original_user_id_column);

    let username_column = composite_users.column_by_name(&mut conn, "username").unwrap();
    assert_eq!(username_column.column_name, "username");
    assert!(!username_column.is_foreign_key(&mut conn));

    assert!(username_column.foreign_table(&mut conn).unwrap().is_none());

    docker.stop().await.unwrap();
}
