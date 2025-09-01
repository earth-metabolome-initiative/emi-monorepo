//! Submodule to test the same-as network generation.

mod utils;

use diesel_migrations_utils::prelude::MigrationDirectory;
use utils::*;
use webcodegen::{errors::WebCodeGenError, *};

fn inner_test_same_as_indices(
    conn: &mut diesel::pg::PgConnection,
    database_name: &str,
) -> Result<(), WebCodeGenError> {
    let cousins = Table::load(conn, "cousins", "public", &database_name)?;
    let great_grandchildren = Table::load(conn, "great_grandchildren", "public", &database_name)?;
    let grandchildren = Table::load(conn, "grandchildren", "public", &database_name)?;

    assert!(grandchildren.must_be_inserted_alongside_with(&great_grandchildren, conn)?);
    assert!(!cousins.must_be_inserted_alongside_with(&great_grandchildren, conn)?);
    assert!(!cousins.must_be_inserted_alongside_with(&grandchildren, conn)?);

    let mut codegen = Codegen::default()
        .set_output_directory("tests/same_as_indices".as_ref())
        .enable_joinables()
        .enable_tables_schema()
        .add_schema("public");
    codegen.generate(conn, &database_name)?;
    codegen.print_same_as_network(conn, &database_name, &format!("{}.dot", database_name))?;

    Ok(())
}

#[tokio::test]
/// Test same-as network generation.
async fn test_same_as_indices() {
    let (docker, mut conn, database_name) = setup_database_with_migrations(
        "test_same_as_indices",
        MigrationDirectory::try_from("./test_same_as_indices").unwrap(),
    )
    .await
    .unwrap();

    let outcome = inner_test_same_as_indices(&mut conn, &database_name);

    docker.stop().await.unwrap();

    outcome.unwrap();

    codegen_test("same_as_indices");

    std::fs::remove_dir_all("tests/same_as_indices").unwrap();
}
