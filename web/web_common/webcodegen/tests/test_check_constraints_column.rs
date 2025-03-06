//! Submodule to test the codegen of a diesel macro to allow joins

mod utils;

use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use utils::*;
use webcodegen::*;

const CHECK_CONSTRAINT_TEST_MIGRATIONS: EmbeddedMigrations =
    embed_migrations!("./test_check_constraints_migrations");

#[tokio::test]
/// Test generation of the check_constraints()
async fn test_check_constraints_column() {
    let (docker, mut conn, database_name) = setup_database_with_migrations(
        "test_check_constraints_column",
        CHECK_CONSTRAINT_TEST_MIGRATIONS,
    )
    .await
    .unwrap();

    for (column_name, expected_number_of_check_constraints, expected_number_of_functions) in
        [("fortune", 1, 0), ("id", 0, 0), ("email", 0, 0), ("age", 2, 2), ("created_at", 0, 0)]
    {
        let column =
            Column::load(column_name, "constrained_users", "public", &database_name, &mut conn)
                .expect(&format!("Failed to query database `{database_name}`"))
                .expect(&format!("Failed to retrieve column `{column_name}`"));

        let column_check_constraints =
            column.single_column_check_constraints(&mut conn).expect(&format!(
                "Failed to query check constraints for column `{column_name}`",
                column_name = column_name
            ));

        assert_eq!(
            column_check_constraints.len(),
            expected_number_of_check_constraints,
            "Column `{column_name}` has an unexpected number of check constraints"
        );

        for check_constraint in column_check_constraints {
            let functions = check_constraint
                .functions(&mut conn).expect(&format!(
                    "Failed to query functions for check constraint `{check_constraint_name}`",
                    check_constraint_name = check_constraint.constraint_name
                ));
            assert_eq!(
                functions.len(),
                expected_number_of_functions,
                "Check constraint `{check_constraint_name}` has an unexpected number of functions",
                check_constraint_name = check_constraint.constraint_name
            );

            // None of these functions are expected to be associated with an extension.
            for function in functions {
                assert!(
                    function.extension(&mut conn).unwrap().is_none(),
                    "Function `{function_name}` is associated with an extension",
                    function_name = function.proname
                );
            }
        }
    }

    docker.stop().await.unwrap();
}
