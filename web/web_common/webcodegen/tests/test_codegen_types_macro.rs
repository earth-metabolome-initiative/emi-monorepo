//! Submodule to test the codegen of type macro.

mod utils;

use utils::*;
use webcodegen::*;

#[tokio::test]
/// Test generation of type macro.
async fn test_codegen_types_macro() {
    let (docker, mut conn, database_name) =
        setup_database_with_default_migrations("test_codegen_types_macro").await.unwrap();

    let outcome = Codegen::default()
        .set_output_directory("tests/codegen_types_macro".as_ref())
        .enable_sql_types()
        .beautify()
        .generate(&mut conn, &database_name, None);

    docker.stop().await.unwrap();
    outcome.unwrap();

    codegen_test("codegen_types_macro");

    std::fs::remove_dir_all("tests/codegen_types_macro").unwrap();
}
