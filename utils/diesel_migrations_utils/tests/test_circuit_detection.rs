//! Test submodule to verify whether circuits are detected correctly.

use diesel_migrations_utils::prelude::*;

mod utils;

#[test]
/// Test to check that the out-of-order migrations are recognized as such.
pub fn test_out_of_order_circular_migrations() {
    assert_eq!(
        MigrationDirectory::try_from("out_of_order_circular_migrations").unwrap_err(),
        MigrationError::CircularDependency(vec![
            (2, "second_migration".to_owned()),
            (3, "third_migration_looped".to_owned())
        ])
    );
}
