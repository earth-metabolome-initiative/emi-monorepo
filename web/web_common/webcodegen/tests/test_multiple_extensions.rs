//! Submodule testing the detection of columns requiring a partial builder.

mod utils;

use diesel_migrations_utils::prelude::MigrationDirectory;
use utils::*;
use webcodegen::*;

#[tokio::test]
/// Test retrieval of extensions from a column
async fn test_multiple_extensions() {
    let (docker, mut conn, database_name) = setup_database_with_migrations(
        "test_multiple_extensions",
        MigrationDirectory::try_from("./test_multiple_extensions").unwrap(),
    )
    .await
    .unwrap();

    let users = Table::load(&mut conn, "users", "public", &database_name)
        .expect("Failed to load `users` table");
    let projects = Table::load(&mut conn, "projects", "public", &database_name)
        .expect("Failed to load `projects` table");
    let team_members = Table::load(&mut conn, "team_members", "public", &database_name)
        .expect("Failed to load `team_members` table");
    let team_projects = Table::load(&mut conn, "team_projects", "public", &database_name)
        .expect("Failed to load `team_projects` table");

    let mut codegen = Codegen::default()
        .users(&users)
        .projects(&projects)
        .team_members(&team_members)
        .team_projects(&team_projects)
        .set_output_directory("tests/codegen_multiple_extensions".as_ref())
        .enable_insertable_trait()
        .beautify()
        .add_schema("public");
    let outcome = codegen.generate(&mut conn, &database_name);
    let network = codegen.table_extension_network().unwrap();
    let dot = network.to_dot();
    std::fs::write("test_multiple_extensions.dot", dot).expect("Failed to write DOT file");
    docker.stop().await.unwrap();
    outcome.expect("Failed to generate code for multiple extensions");

    codegen_test("codegen_multiple_extensions");
}
