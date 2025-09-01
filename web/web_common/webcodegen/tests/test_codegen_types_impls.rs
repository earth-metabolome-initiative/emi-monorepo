//! Submodule to test the codegen of type structs.

mod utils;

use utils::*;
use webcodegen::*;

#[tokio::test]
/// Test generation of type structs.
async fn test_codegen_types_impls() {
    let (docker, mut conn, database_name) =
        setup_database_with_default_migrations("test_codegen_types_impls").await.unwrap();

    let outcome = Codegen::default()
        .set_output_directory("tests/codegen_types_impls".as_ref())
        .enable_type_impls()
        .beautify()
        .add_schema("public")
        .generate(&mut conn, &database_name);
    docker.stop().await.unwrap();
    outcome.unwrap();

    codegen_test("codegen_types_impls");

    std::fs::remove_dir_all("tests/codegen_types_impls").unwrap();
}
