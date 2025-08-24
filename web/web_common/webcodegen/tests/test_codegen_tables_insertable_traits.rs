//! Submodule to test the generation of the [`Insertable`] traits for tables.

mod utils;

use utils::*;
use webcodegen::*;

#[tokio::test]
/// Test generation of [`Insertable`] traits for tables.
async fn test_codegen_tables_insertable_traits() {
    let (docker, mut conn, database_name) =
        setup_database_with_default_migrations("test_codegen_tables_insertable_traits")
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
    let pgrx_validation = PgExtension::load("pgrx_validation", "public", &mut conn)
        .expect("Failed to query the database")
        .expect("Failed to load `pgrx_validation` extension, maybe it is not installed");
    let outcome = Codegen::default()
        .users(&users)
        .projects(&projects)
        .team_members(&team_members)
        .team_projects(&team_projects)
        .add_check_constraint_extension(&pgrx_validation)
        .set_output_directory("tests/codegen_tables_insertable_traits".as_ref())
        .enable_insertable_trait()
        .beautify()
        .generate(&mut conn, &database_name);
    docker.stop().await.unwrap();
    outcome.unwrap();

    codegen_test("codegen_tables_insertable_traits");

    std::fs::remove_dir_all("tests/codegen_tables_insertable_traits").unwrap();
}
