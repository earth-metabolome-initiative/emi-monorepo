//! Submodule to test the generation of the [`Deletable`] traits for tables.

mod utils;

use utils::*;
use webcodegen::*;

#[tokio::test]
/// Test generation of [`Deletable`] traits for tables.
async fn test_codegen_tables_deletable_traits() {
    let (docker, mut conn, database_name) =
        setup_database_with_default_migrations("test_codegen_tables_deletable_traits")
            .await
            .unwrap();

    let users = Table::load(&mut conn, "users", None, &database_name)
        .expect("Failed to load `users` table");
    let projects = Table::load(&mut conn, "projects", None, &database_name)
        .expect("Failed to load `projects` table");
    let team_members = Table::load(&mut conn, "team_members", None, &database_name)
        .expect("Failed to load `team_members` table");
    let team_projects = Table::load(&mut conn, "team_projects", None, &database_name)
        .expect("Failed to load `team_projects` table");

    let outcome = Codegen::default()
        .users(&users)
        .projects(&projects)
        .team_members(&team_members)
        .team_projects(&team_projects)
        .set_output_directory("tests/codegen_tables_deletable_traits".as_ref())
        .enable_deletable_trait()
        .beautify()
        .generate(&mut conn, &database_name, None);
    docker.stop().await.unwrap();
    outcome.unwrap();

    codegen_test("codegen_tables_deletable_traits");

    std::fs::remove_dir_all("tests/codegen_tables_deletable_traits").unwrap();
}
