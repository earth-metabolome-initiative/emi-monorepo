//! Submodule testing the methods associated with the `KeyColumnUsage` struct.

mod utils;

use utils::*;
use webcodegen::Table;

#[tokio::test]
/// Test methods associated with the `KeyColumnUsage` struct.
async fn test_key_column_usage() {
    let (docker, mut conn, database_name) =
        setup_database_with_default_migrations("test_key_column_usage").await.unwrap();

    let composite_user_profiles =
        match Table::load(&mut conn, "composite_user_profiles", "public", &database_name) {
            Ok(table) => table,
            Err(err) => {
                docker.stop().await.unwrap();
                panic!("Error loading composite_user_profiles: {err}");
            }
        };

    let composite_users = match Table::load(&mut conn, "composite_users", "public", &database_name)
    {
        Ok(table) => table,
        Err(err) => {
            docker.stop().await.unwrap();
            panic!("Error loading composite_users: {err}");
        }
    };

    let users = match Table::load(&mut conn, "users", "public", &database_name) {
        Ok(table) => table,
        Err(err) => {
            docker.stop().await.unwrap();
            panic!("Error loading users: {err}");
        }
    };

    let key_column_usage = match composite_user_profiles.foreign_keys(&mut conn) {
        Ok(usage) => usage,
        Err(err) => {
            docker.stop().await.unwrap();
            panic!("Error getting key column usage: {err}");
        }
    };

    assert_eq!(key_column_usage.len(), 1, "Expected 1 foreign key, found {key_column_usage:?}");

    let first_usage = &key_column_usage[0];

    assert_eq!(first_usage.constraint_name, "fk_composite_user_profiles");
    assert_eq!(first_usage.table_name, "composite_user_profiles");
    assert_eq!(first_usage.column_name, "primary_id");

    let Ok(columns) = first_usage.columns(&mut conn) else {
        docker.stop().await.unwrap();
        panic!("Error getting columns for key column usage: {first_usage:?}");
    };
    assert_eq!(columns.len(), 2, "Expected 2 columns, found {columns:?}");

    let Ok(expected_first_column) = composite_user_profiles.column_by_name(&mut conn, "primary_id")
    else {
        docker.stop().await.unwrap();
        panic!("Error getting expected first column for key column usage: {first_usage:?}");
    };

    let Ok(expected_second_column) =
        composite_user_profiles.column_by_name(&mut conn, "secondary_id")
    else {
        docker.stop().await.unwrap();
        panic!("Error getting expected second column for key column usage: {first_usage:?}");
    };

    assert_eq!(columns[0], expected_first_column, "First column mismatch: {columns:?}");
    assert_eq!(columns[1], expected_second_column, "Second column mismatch: {columns:?}");

    let foreign_table = match first_usage.foreign_table(&mut conn) {
        Ok(Some(table)) => table,
        Ok(None) => {
            docker.stop().await.unwrap();
            panic!(
                "Expected foreign table to be Some, found None for key column usage: {first_usage:?}"
            );
        }
        Err(err) => {
            docker.stop().await.unwrap();
            panic!("Error getting foreign table for key column usage: {err}");
        }
    };

    assert_eq!(foreign_table, composite_users, "Foreign table mismatch: {foreign_table:?}");

    let foreign_columns = match first_usage.foreign_columns(&mut conn) {
        Ok(columns) => columns,
        Err(err) => {
            docker.stop().await.unwrap();
            panic!("Error getting foreign columns for key column usage: {err}");
        }
    };

    assert_eq!(foreign_columns.len(), 2, "Expected 2 foreign columns, found {foreign_columns:?}");

    let first_foreign_column = match foreign_table.column_by_name(&mut conn, "primary_id") {
        Ok(column) => column,
        Err(err) => {
            docker.stop().await.unwrap();
            panic!("Error getting first foreign column for key column usage: {err}");
        }
    };
    let second_foreign_column = match foreign_table.column_by_name(&mut conn, "secondary_id") {
        Ok(column) => column,
        Err(err) => {
            docker.stop().await.unwrap();
            panic!("Error getting second foreign column for key column usage: {err}");
        }
    };

    assert_eq!(
        foreign_columns[0], first_foreign_column,
        "First foreign column mismatch: {foreign_columns:?}"
    );
    assert_eq!(
        foreign_columns[1], second_foreign_column,
        "Second foreign column mismatch: {foreign_columns:?}"
    );

    let composite_users_foreign_keys = match composite_users.foreign_keys(&mut conn) {
        Ok(foreign_keys) => foreign_keys,
        Err(err) => {
            docker.stop().await.unwrap();
            panic!("Error getting foreign keys for composite_users: {err}");
        }
    };

    assert_eq!(
        composite_users_foreign_keys.len(),
        2,
        "Expected 2 foreign keys, found {composite_users_foreign_keys:?}"
    );

    let first_composite_users_fk = &composite_users_foreign_keys[0];
    let first_composite_users_fk_columns = match first_composite_users_fk.columns(&mut conn) {
        Ok(columns) => columns,
        Err(err) => {
            docker.stop().await.unwrap();
            panic!("Error getting columns for first composite_users foreign key: {err}");
        }
    };

    assert_eq!(
        first_composite_users_fk_columns.len(),
        1,
        "Expected 1 column for first composite_users foreign key, found {first_composite_users_fk_columns:?}"
    );

    assert_eq!(
        first_composite_users_fk_columns[0], first_foreign_column,
        "First column of first composite_users foreign key mismatch: {first_composite_users_fk_columns:?}"
    );

    let second_composite_users_fk = &composite_users_foreign_keys[1];
    let second_composite_users_fk_columns = match second_composite_users_fk.columns(&mut conn) {
        Ok(columns) => columns,
        Err(err) => {
            docker.stop().await.unwrap();
            panic!("Error getting columns for second composite_users foreign key: {err}");
        }
    };

    assert_eq!(
        second_composite_users_fk_columns.len(),
        1,
        "Expected 1 column for second composite_users foreign key, found {second_composite_users_fk_columns:?}"
    );

    assert_eq!(
        second_composite_users_fk_columns[0], second_foreign_column,
        "First column of second composite_users foreign key mismatch: {second_composite_users_fk_columns:?}"
    );

    // We check that for both foreign key constraints in `composite_users`, the
    // foreign table is `users`
    for fk in &composite_users_foreign_keys {
        let foreign_table = match fk.foreign_table(&mut conn) {
            Ok(Some(table)) => table,
            Ok(None) => {
                docker.stop().await.unwrap();
                panic!("Expected foreign table to be Some, found None for foreign key: {fk:?}");
            }
            Err(err) => {
                docker.stop().await.unwrap();
                panic!("Error getting foreign table for foreign key: {err}");
            }
        };
        assert_eq!(foreign_table, users, "Foreign table mismatch for foreign key");

        // We check that this column has only a single foreign key column.
        let foreign_columns = match fk.foreign_columns(&mut conn) {
            Ok(columns) => columns,
            Err(err) => {
                docker.stop().await.unwrap();
                panic!("Error getting foreign columns for foreign key: {err}");
            }
        };
        assert_eq!(
            foreign_columns.len(),
            1,
            "Expected 1 foreign column, found {foreign_columns:?} for foreign key: {fk:?}"
        );
        let first_foreign_column = match foreign_table.column_by_name(&mut conn, "id") {
            Ok(column) => column,
            Err(err) => {
                docker.stop().await.unwrap();
                panic!("Error getting first foreign column for foreign key: {err}");
            }
        };
        assert_eq!(
            foreign_columns[0], first_foreign_column,
            "First foreign column mismatch: {foreign_columns:?} for foreign key: {fk:?}"
        );
    }

    // Cleanup
    docker.stop().await.unwrap();
}
