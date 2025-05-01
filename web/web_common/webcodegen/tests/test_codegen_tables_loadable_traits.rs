//! Submodule to test the generation of the [`Loadable`] traits for tables.

mod utils;

use utils::*;
use webcodegen::*;

#[tokio::test]
/// Test generation of [`Loadable`] traits for tables.
async fn test_codegen_tables_loadable_traits() {
    let (docker, mut conn, database_name) =
        setup_database_with_default_migrations("test_codegen_tables_loadable_traits")
            .await
            .unwrap();

    let outcome = Codegen::default()
        .set_output_directory("tests/codegen_tables_loadable_traits".as_ref())
        .enable_loadable_trait()
        .beautify()
        .generate(&mut conn, &database_name, None)
        .await;
    docker.stop().await.unwrap();
    outcome.unwrap();

    codegen_test("codegen_tables_loadable_traits");

    std::fs::remove_dir_all("tests/codegen_tables_loadable_traits").unwrap();
}
