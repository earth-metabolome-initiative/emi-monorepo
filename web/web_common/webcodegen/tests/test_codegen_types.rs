//! Submodule to test the codegen of diesel types.

mod utils;

use utils::*;
use webcodegen::*;

#[tokio::test]
/// Test generation of diesel schema for types.
async fn test_codegen_types() {
    let (docker, mut conn, database_name) =
        setup_database_with_default_migrations("test_codegen_types").await.unwrap();

    Codegen::default()
        .set_output_directory("tests/codegen_types".as_ref())
        .enable_sql_types()
        .beautify()
        .generate(&mut conn, &database_name, None)
        .await
        .unwrap();

    docker.stop().await.unwrap();

    codegen_test("codegen_types");

    std::fs::remove_dir_all("tests/codegen_types").unwrap();
}
