//! Test suite for table metadata loading
mod utils;

use diesel_async::AsyncPgConnection;
use utils::*;
use webcodegen::{errors::WebCodeGenError, *};

async fn test_check_constraints(
    database_name: &str,
    conn: &mut AsyncPgConnection,
) -> Result<(), WebCodeGenError> {
    let users = Table::load(conn, "users", None, database_name).await.unwrap();

    let table_check_constraint = users.check_constraints(conn).await?;

    assert_eq!(
        table_check_constraint.len(),
        5,
        "Expected 5 check constraint, got: {table_check_constraint:?}"
    );

    let user_name_column = users.column_by_name(conn, "username").await?;

    assert_eq!(user_name_column.column_name, "username");

    Ok(())
}

#[tokio::test]
async fn test_user_table() {
    let (docker, mut conn, database_name) =
        setup_database_with_default_migrations("test_table_properties").await.unwrap();

    // We try to load all elements of each type, so to ensure
    // that the structs are actually compatible with the schema
    // of PostgreSQL
    let all_tables = Table::load_all(&mut conn, &database_name, None).await.unwrap();
    assert!(!all_tables.is_empty());

    let _all_columns = Column::load_all(&mut conn).await;

    let _all_table_constraints = TableConstraint::load_all(&mut conn).await;
    let _all_key_column_usage = KeyColumnUsage::load_all_key_column_usages(&mut conn).await;
    let _all_referential_constraints =
        ReferentialConstraint::load_all_referential_constraints(&mut conn).await;
    let _all_constraint_column_usage =
        ConstraintColumnUsage::load_all_constraint_column_usages(&mut conn).await;
    let _all_constraint_table_usage = ConstraintTableUsage::load_all(&mut conn).await;
    let _all_domain_constraint = DomainConstraint::load_all_domain_constraints(&mut conn).await;

    let users = Table::load(&mut conn, "users", None, &database_name).await.unwrap();

    test_check_constraints(&database_name, &mut conn).await.unwrap();

    let original_user_id_column = users.column_by_name(&mut conn, "id").await.unwrap();

    let columns: Result<Vec<Column>, WebCodeGenError> = users.columns(&mut conn).await;

    assert!(columns.is_ok());
    let columns = columns.unwrap();
    assert_eq!(columns.len(), 4);

    let primary_key_columns: Result<Vec<Column>, WebCodeGenError> =
        users.primary_key_columns(&mut conn).await;

    assert!(primary_key_columns.is_ok());
    let primary_key_columns = primary_key_columns.unwrap();
    assert_eq!(primary_key_columns.len(), 1);

    let unique_columns: Result<Vec<Vec<Column>>, WebCodeGenError> = users.unique_columns(&mut conn).await;

    assert!(unique_columns.is_ok());
    let unique_columns = unique_columns.unwrap();

    assert_eq!(unique_columns.len(), 3);
    assert_eq!(unique_columns[0].len(), 1);
    assert_eq!(unique_columns[1].len(), 2);
    assert_eq!(unique_columns[2].len(), 1);

    let composite_users = Table::load(&mut conn, "composite_users", None, &database_name).await.unwrap();

    let columns: Result<Vec<Column>, WebCodeGenError> = composite_users.columns(&mut conn).await;
    let primary_key_columns: Result<Vec<Column>, WebCodeGenError> =
        composite_users.primary_key_columns(&mut conn).await;

    assert!(columns.is_ok());
    let columns = columns.unwrap();
    assert_eq!(columns.len(), 8);

    assert!(primary_key_columns.is_ok());
    let primary_key_columns = primary_key_columns.unwrap();
    assert_eq!(primary_key_columns.len(), 2);

    let primary_id_column = composite_users.column_by_name(&mut conn, "primary_id").await.unwrap();
    assert_eq!(primary_id_column.column_name, "primary_id");
    assert!(primary_id_column.is_foreign_key(&mut conn).await);

    let (foreign_table, user_id_column) =
        primary_id_column.foreign_table(&mut conn).await.unwrap().unwrap();
    assert_eq!(foreign_table, users);
    assert_eq!(user_id_column, original_user_id_column);

    let secondary_id_column = composite_users.column_by_name(&mut conn, "secondary_id").await.unwrap();
    assert_eq!(secondary_id_column.column_name, "secondary_id");
    assert!(secondary_id_column.is_foreign_key(&mut conn).await);

    let (foreign_table, user_id_column) =
        secondary_id_column.foreign_table(&mut conn).await.unwrap().unwrap();
    assert_eq!(foreign_table, users);
    assert_eq!(user_id_column, original_user_id_column);

    let username_column = composite_users.column_by_name(&mut conn, "username").await.unwrap();
    assert_eq!(username_column.column_name, "username");
    assert!(!username_column.is_foreign_key(&mut conn).await);

    assert!(username_column.foreign_table(&mut conn).await.unwrap().is_none());

    docker.stop().await.unwrap();
}
