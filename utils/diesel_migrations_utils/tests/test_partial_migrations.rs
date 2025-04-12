//! Test to check that the partial migrations causes the expected error.
use diesel_migrations_utils::prelude::*;

#[test]
/// Test to check that the partial migrations causes the expected error.
fn test_partial_migrations() {
    assert_eq!(
        MigrationDirectory::try_from("partial_up_migrations").unwrap_err(),
        MigrationError::MissingUpMigration(1)
    );

    assert_eq!(
        MigrationDirectory::try_from("partial_down_migrations").unwrap_err(),
        MigrationError::MissingDownMigration(1)
    );
}
