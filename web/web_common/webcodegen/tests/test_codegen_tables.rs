//! Submodule to test the codegen of diesel tables.

mod utils;

use utils::*;

use webcodegen::*;

#[tokio::test]
/// Test generation of diesel schema for tables.
async fn test_codegen_tables() {
    let (docker, mut conn, database_name) =
        setup_database_with_default_migrations("test_table_codegen").await.unwrap();

    Codegen::default()
        .set_output_directory("tests/test_table_codegen".as_ref())
        .generate(&mut conn, &database_name, None)
        .unwrap();

    docker.stop().await.unwrap();

    // TODO! ACTUALLY TEST!
    // let builder = trybuild::TestCases::new();
    // add_main_to_file("tests/test_table_codegen/codegen.rs");
    // builder.pass("tests/test_table_codegen/codegen.rs");
}
