//! Submodule to test the codegen of a diesel macro to allow joins

mod utils;

use utils::*;
use webcodegen::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};



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

    let username_column = Column::load(
        "fortune",
        "constrained_users",
        "test_check_constraints_column_db",
        &mut conn
    ).unwrap().unwrap();

    let check_constraint = username_column.check_constraints(&mut conn).unwrap();

    docker.stop().await.unwrap();
}
