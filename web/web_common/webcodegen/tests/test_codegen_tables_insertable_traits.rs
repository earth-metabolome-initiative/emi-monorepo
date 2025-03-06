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

    let users = Table::load(&mut conn, "users", None, &database_name).unwrap();
    let outcome = Codegen::default()
        .users(&users)
        .set_output_directory("tests/codegen_tables_insertable_traits".as_ref())
        .enable_insertable_trait()
        .beautify()
        .generate(&mut conn, &database_name, None);
    docker.stop().await.unwrap();
    outcome.unwrap();

    codegen_test("codegen_tables_insertable_traits");

    std::fs::remove_dir_all("tests/codegen_tables_insertable_traits").unwrap();
}
