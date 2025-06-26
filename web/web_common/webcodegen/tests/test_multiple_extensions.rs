//! Submodule testing the detection of columns requiring a partial builder.

mod utils;

use diesel_migrations_utils::prelude::MigrationDirectory;
use utils::*;
use webcodegen::*;

#[tokio::test]
/// Test retrieval of extensions from a column
async fn test_multiple_extensions() {
    let (docker, conn, database_name) = setup_database_with_migrations(
        "test_multiple_extensions",
        MigrationDirectory::try_from("./test_multiple_extensions").unwrap(),
    )
    .await
    .unwrap();

    // let mut conn = pool.get().expect("Failed to get connection from pool");
    // let users = Table::load(&mut conn, "users", None, &database_name)
    //     .expect("Failed to load `users` table");
    // let projects = Table::load(&mut conn, "projects", None, &database_name)
    //     .expect("Failed to load `projects` table");
    // let team_members = Table::load(&mut conn, "team_members", None,
    // &database_name)     .expect("Failed to load `team_members` table");
    // let team_projects = Table::load(&mut conn, "team_projects", None,
    // &database_name)     .expect("Failed to load `team_projects` table");

    // let outcome = Codegen::default()
    //     .users(&users)
    //     .projects(&projects)
    //     .team_members(&team_members)
    //     .team_projects(&team_projects)
    //     .set_output_directory("tests/codegen_tables_structs".as_ref())
    //     .enable_deletable_trait()
    //     .enable_insertable_trait()
    //     .enable_foreign_trait()
    //     .enable_updatable_trait()
    //     .enable_upsertable_trait()
    //     .enable_crud_operations()
    //     .enable_yew()
    //     .beautify()
    //     .generate(pool, &database_name, None);
    docker.stop().await.unwrap();
    // outcome.expect("Failed to generate code for multiple extensions");

    // codegen_test("codegen_tables_structs");
}
