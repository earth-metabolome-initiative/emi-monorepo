//! Submodule to test the codegen to generate diesel joinables.

mod utils;

use utils::*;

use webcodegen::*;

#[tokio::test]
/// Test generation of diesel macro to define joinables.
async fn test_codegen_joinables() {
    let (docker, mut conn, database_name) =
        setup_database_with_default_migrations("test_codegen_joinables").await.unwrap();

    Codegen::default()
        .set_output_directory("tests/codegen_joinables".as_ref())
		.enable_joinables()
        .enable_tables()
        .beautify()
        .generate(&mut conn, &database_name, None)
        .unwrap();

    docker.stop().await.unwrap();

    codegen_test("codegen_joinables");

    std::fs::remove_dir_all("tests/codegen_joinables").unwrap();
}
