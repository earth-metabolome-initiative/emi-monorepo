//! Test that the proper error is raised when the SQL migrations are wrong.

use diesel_migrations_utils::prelude::*;

#[test]
/// Test to check that the wrong SQL migrations causes the expected error.
fn test_wrong_sql_migrations() {
    assert_eq!(
        MigrationDirectory::try_from("wrong_sql_migrations").unwrap_err(),
        Error::InvalidSql(
            1,
            MigrationKind::Up,
            "sql parser error: Expected: ',' or ')' after column definition, found: name at Line: 4, Column: 2".to_owned()
        )
    );
}
