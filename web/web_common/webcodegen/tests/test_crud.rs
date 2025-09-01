//! Submodule to test the generation of the CRUD-related traits for tables.

mod utils;

use utils::*;
use webcodegen::*;

#[tokio::test]
/// Test generation of CRUD traits for tables.
async fn test_codegen_tables_crud_traits() {
    let (docker, mut conn, database_name) =
        setup_database_with_default_migrations("test_codegen_tables_crud_traits").await.unwrap();

    let outcome = Codegen::default()
        .set_output_directory("tests/codegen_tables_crud_traits".as_ref())
        .enable_crud_operations()
        .beautify()
        .add_schema("public")
        .generate(&mut conn, &database_name);
    docker.stop().await.unwrap();
    outcome.unwrap();

    codegen_test("codegen_tables_crud_traits");

    std::fs::remove_dir_all("tests/codegen_tables_crud_traits").unwrap();
}
