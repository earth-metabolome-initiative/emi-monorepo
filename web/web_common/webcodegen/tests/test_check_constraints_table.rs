//! Submodule to test for the check constraints at the table level

mod utils;

use diesel_migrations_utils::prelude::MigrationDirectory;
use utils::*;
use webcodegen::*;

#[tokio::test]
/// Test generation of the check_constraints()
async fn test_check_constraints_table() {
    let (docker, mut conn, database_name) = setup_database_with_migrations(
        "test_check_constraints_column",
        MigrationDirectory::try_from("./test_check_constraints_migrations").unwrap(),
    )
    .await
    .unwrap();

    for (table_name, expected_number_of_check_constraints) in
        [("constrained_users", 1), ("constrained_samples", 2), ("unconstrained_samples", 0)]
    {
        let table = Table::load(&mut conn, table_name, Some("public"), &database_name)
            .unwrap_or_else(|_| panic!("Failed to retrieve table `{table_name}`"));

        let table_check_constraints =
            table.multi_column_check_constraints(&mut conn).unwrap_or_else(|_| {
                panic!("Failed to query check constraints for table `{table_name}`")
            });

        assert_eq!(
            table_check_constraints.len(),
            expected_number_of_check_constraints,
            "Table `{table_name}` has an unexpected number of check constraints"
        );
    }

    docker.stop().await.unwrap();
}
