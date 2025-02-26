//! Submodule to test the codegen of diesel tables.

mod utils;

use utils::*;
use webcodegen::*;

#[tokio::test]
/// Test generation of diesel schema for tables.
async fn test_codegen_tables_structs() {
    let (docker, mut conn, database_name) =
        setup_database_with_default_migrations("test_codegen_tables_structs").await.unwrap();

    let outcome = Codegen::default()
        .set_output_directory("tests/codegen_tables_structs".as_ref())
        .enable_table_structs()
        .beautify()
        .generate(&mut conn, &database_name, None);
    docker.stop().await.unwrap();
    outcome.unwrap();

    codegen_test("codegen_tables_structs");

    std::fs::remove_dir_all("tests/codegen_tables_structs").unwrap();
}
