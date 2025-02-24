//! Submodule to test the codegen of a diesel macro to allow joins

mod utils;

use utils::*;

use webcodegen::*;

#[tokio::test]
/// Test generation of diesel macro to allow joins.
async fn test_codegen_allow_join() {
    let (docker, mut conn, database_name) =
        setup_database_with_default_migrations("test_codegen_allow_join").await.unwrap();

    Codegen::default()
        .set_output_directory("tests/codegen_allow_join".as_ref())
		.enable_allow_tables_to_appear_in_same_query()
        .enable_tables_schema()
        .beautify()
        .generate(&mut conn, &database_name, None)
        .unwrap();

    docker.stop().await.unwrap();

    codegen_test("codegen_allow_join");

    std::fs::remove_dir_all("tests/codegen_allow_join").unwrap();
}
