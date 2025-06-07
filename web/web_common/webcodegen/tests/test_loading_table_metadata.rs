//! Test suite for table metadata loading
mod utils;

use diesel::PgConnection;
use utils::*;
use webcodegen::{errors::WebCodeGenError, *};

fn test_check_constraints(
    database_name: &str,
    conn: &mut PgConnection,
) -> Result<(), WebCodeGenError> {
    let users = Table::load(conn, "users", None, database_name).unwrap();

    let table_check_constraint = users.check_constraints(conn)?;

    assert_eq!(
        table_check_constraint.len(),
        5,
        "Expected 5 check constraint, got: {table_check_constraint:?}"
    );

    let user_name_column = users.column_by_name(conn, "username")?;

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
    let all_tables = Table::load_all(&mut conn, &database_name, None).unwrap();
    assert!(!all_tables.is_empty());

    let _all_columns = Column::load_all(&mut conn);

    let _all_table_constraints = TableConstraint::load_all(&mut conn);
    let _all_key_column_usage = KeyColumnUsage::load_all_key_column_usages(&mut conn);
    let _all_referential_constraints =
        ReferentialConstraint::load_all_referential_constraints(&mut conn);
    let _all_constraint_column_usage =
        ConstraintColumnUsage::load_all_constraint_column_usages(&mut conn);
    let _all_constraint_table_usage = ConstraintTableUsage::load_all(&mut conn);
    let _all_domain_constraint = DomainConstraint::load_all_domain_constraints(&mut conn);

    let users = Table::load(&mut conn, "users", None, &database_name).unwrap();

    test_check_constraints(&database_name, &mut conn).unwrap();

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
    assert_eq!(columns.len(), 5);

    assert!(primary_key_columns.is_ok());
    let primary_key_columns = primary_key_columns.unwrap();
    assert_eq!(primary_key_columns.len(), 2);

    let primary_id_column = composite_users.column_by_name(&mut conn, "primary_id").unwrap();
    assert_eq!(primary_id_column.column_name, "primary_id");
    assert!(primary_id_column.is_foreign_key(&mut conn).unwrap());

    let secondary_id_column = composite_users.column_by_name(&mut conn, "secondary_id").unwrap();
    assert_eq!(secondary_id_column.column_name, "secondary_id");
    assert!(secondary_id_column.is_foreign_key(&mut conn).unwrap());

    let username_column = composite_users.column_by_name(&mut conn, "username").unwrap();
    assert_eq!(username_column.column_name, "username");
    assert!(!username_column.is_foreign_key(&mut conn).unwrap());

    docker.stop().await.unwrap();
}
