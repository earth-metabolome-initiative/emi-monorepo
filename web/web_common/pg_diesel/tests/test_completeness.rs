//! Submodule testing that all relevant postgres tables exist in the schema.

use std::path::PathBuf;

use common_traits::builder::Builder;
use diesel::PgConnection;
use pg_diesel::database::PgDatabaseBuilder;
use reference_docker::reference_docker_with_connection;
use sql_traits::traits::{ColumnLike, TableLike, database::DatabaseLike};

#[tokio::test]
/// Test retrieval of extensions from a column
async fn test_schema_completeness() {
    let database_name = "test_schema_completeness";
    let (docker, mut conn) = reference_docker_with_connection::<PgConnection>(database_name, 45658)
        .await
        .expect("Failed to start container");

    // We load the schema.rs document
    // Deny-listed Postgres types for the generated schema
    //
    // The following Postgres types are intentionally excluded from Diesel code
    // generation because they are internal/system or otherwise do not have a
    // clear, stable mapping to Diesel's type system. It may be possible in the
    // future to add explicit custom structs or parsers to support these, but
    // for now they are deny-listed.
    //
    // - `anyarray` A Postgres polymorphic pseudo-type used to represent "any array"
    //   of some element type. Because the element type is not fixed, there is no
    //   single Diesel type that safely represents every `anyarray` column. Mapping
    //   would require knowing the element type per-column (or introducing a
    //   generic/custom wrapper), so these columns are omitted until an explicit,
    //   well-defined representation is added.
    //
    // - `pg_ndistinct` An internal statistical type used by the planner to store
    //   NDISTINCT estimates in system catalogs. This is not a regular user-facing
    //   data type and does not have a documented, stable binary/text representation
    //   suitable for Diesel. As such, it is skipped; a future custom struct could
    //   be provided if you need to surface this information in Rust.
    //
    // - `pg_dependencies` An internal type that encodes dependency information for
    //   database objects (used in system catalogs). Its format and semantics are
    //   implementation-specific and not covered by Diesel's built-in types. Columns
    //   using this type are therefore deny-listed; a tailored parser or wrapper
    //   type would be required to support them.
    //
    // - `pg_mcv_list` A planner/statistics-internal type representing a
    //   most-common-values list. This form is an internal structure rather than a
    //   regular user type and may use a compact/opaque representation. Diesel does
    //   not provide a native mapping, so these columns are omitted until a clear
    //   custom representation is defined.
    //
    // - `_pg_statistic` An internal/system composite/array type related to the
    //   `pg_statistic` catalog (statistics metadata). System catalog composites and
    //   their array forms are not directly mappable to Diesel primitives because
    //   their layout and usage are special-purpose. They are deny-listed to avoid
    //   generating fragile bindings; a bespoke composite mapping could be added
    //   later if needed.
    //
    // Note: Deny-listing these types avoids generating incorrect or non-portable
    // Diesel bindings. If you need to work with any of these values from Rust,
    // consider adding an explicit custom type, a textual/JSON representation at
    // the SQL level, or a targeted parser to convert the internal format into a
    // stable Rust struct.
    let db = PgDatabaseBuilder::default()
        .connection(&mut conn)
        .catalog(database_name)
        .denylist_types([
            "anyarray",
            "pg_ndistinct",
            "pg_dependencies",
            "pg_mcv_list",
            "_pg_statistic",
        ])
        .unwrap()
        .schemas(vec![
            "pg_toast".to_owned(),
            "pg_catalog".to_owned(),
            "information_schema".to_owned(),
        ])
        .build()
        .expect("Failed to build database");

    let crate_root_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    for table in db.table_dag() {
        if table.table_name().starts_with("_") {
            // Skip internal tables
            continue;
        }

        let expected_schema_path = crate_root_path
            .join("src")
            .join("schema")
            .join(table.table_schema().unwrap())
            .join(format!("{}.rs", table.table_name()));

        if !expected_schema_path.exists() {
            panic!("Table `{}` not found in src/schema/", expected_schema_path.display(),);
        }

        // We read the content of the expected schema file
        let expected_schema_content = std::fs::read_to_string(&expected_schema_path)
            .expect("Unable to read expected schema file");

        // We check that all columns are found in the expected schema file
        for column in table.columns(&db) {
            assert!(
                expected_schema_content.contains(column.column_name()),
                "Column `{}.{}` not found in expected schema file",
                table.table_name(),
                column.column_name(),
            );
        }

        // We check that the number of columns in the table matches the number of "->"
        // in the expected schema file, as each column is represented by a line
        // containing "->"
        let column_count = table.number_of_columns(&db);
        let arrow_count = expected_schema_content.matches("->").count();
        assert_eq!(
            column_count,
            arrow_count,
            "Table `{}` has {} columns, but expected schema file has {} columns, and the unexpected columns should be removed. The expected columns are:\n{}",
            table.table_name(),
            column_count,
            arrow_count,
            table
                .columns(&db)
                .map(|c| format!("- {}", c.column_name()))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }

    docker.stop().await.unwrap();
}
