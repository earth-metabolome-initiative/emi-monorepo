//! Submodule to test the generation of the Foreign traits implementation for
//! tables.

mod utils;

use utils::*;
use webcodegen::*;

#[tokio::test]
/// Test generation of `Foreign` traits implementation for tables.
async fn test_codegen_tables_foreign_traits() {
    let (docker, mut conn, database_name) =
        setup_database_with_default_migrations("test_codegen_tables_foreign_traits").await.unwrap();

    let outcome = Codegen::default()
        .set_output_directory("tests/codegen_tables_foreign_traits".as_ref())
        .enable_foreign_trait()
        .beautify()
        .generate(&mut conn, &database_name, None);
    docker.stop().await.unwrap();
    outcome.unwrap();

    codegen_test("codegen_tables_foreign_traits");

    std::fs::remove_dir_all("tests/codegen_tables_foreign_traits").unwrap();
}
