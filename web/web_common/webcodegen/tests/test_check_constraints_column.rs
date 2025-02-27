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

    for (column_name, expected_number_of_check_constraints) in
        [("fortune", 1), ("id", 0), ("email", 0), ("age", 2), ("created_at", 0)]
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
    }

    docker.stop().await.unwrap();
}
