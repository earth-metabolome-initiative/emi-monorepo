//! Submodule to test the codegen of diesel tables.

mod utils;

use utils::*;

use webcodegen::*;

#[tokio::test]
/// Test generation of diesel schema for tables.
async fn test_codegen_tables() {
    let (docker, mut conn, database_name) =
        setup_database_with_default_migrations("test_codegen_tables").await.unwrap();

    Codegen::default()
        .set_output_directory("tests/codegen_tables".as_ref())
        .enable_tables()
        .beautify()
        .generate(&mut conn, &database_name, None)
        .unwrap();

    docker.stop().await.unwrap();

    codegen_test("codegen_tables");

    std::fs::remove_dir_all("tests/codegen_tables").unwrap();
}
