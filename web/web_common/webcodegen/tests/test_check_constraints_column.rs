//! Submodule to test the codegen of a diesel macro to allow joins

mod utils;

use diesel_migrations_utils::prelude::MigrationDirectory;
use utils::*;
use webcodegen::*;

#[tokio::test]
/// Test generation of the check_constraints()
async fn test_check_constraints_column() {
    let (docker, mut conn, database_name) = setup_database_with_migrations(
        "test_check_constraints_column",
        MigrationDirectory::try_from("./test_check_constraints_migrations").unwrap(),
    )
    .await
    .unwrap();

    for (column_name, expected_number_of_check_constraints, expected_number_of_functions) in
        [("fortune", 1, 0), ("id", 0, 0), ("email", 1, 1), ("age", 2, 2), ("created_at", 0, 0)]
    {
        let column =
            Column::load(column_name, "constrained_users", "public", &database_name, &mut conn)
                .await
                .unwrap_or_else(|_| panic!("Failed to query database `{database_name}`"))
                .unwrap_or_else(|| panic!("Failed to retrieve column `{column_name}`"));

        let column_check_constraints =
            column.check_constraints(&mut conn).await.unwrap_or_else(|_| {
                panic!("Failed to query check constraints for column `{column_name}`")
            });

        assert_eq!(
            column_check_constraints.len(),
            expected_number_of_check_constraints,
            "Column `{column_name}` has an unexpected number of check constraints: {column_check_constraints:?}"
        );

        for check_constraint in column_check_constraints {
            let functions = check_constraint.functions(&mut conn).await.unwrap_or_else(|_| {
                panic!(
                    "Failed to query functions for check constraint `{check_constraint_name}`",
                    check_constraint_name = check_constraint.constraint_name
                )
            });
            assert_eq!(
                functions.len(),
                expected_number_of_functions,
                "Check constraint `{check_constraint_name}` has an unexpected number of functions",
                check_constraint_name = check_constraint.constraint_name
            );

            // None of these functions are expected to be associated with an extension.
            for function in functions {
                assert!(
                    function.extension(&mut conn).await.unwrap().is_none(),
                    "Function `{function_name}` is associated with an extension",
                    function_name = function.proname
                );
            }
        }
    }

    docker.stop().await.unwrap();
}
