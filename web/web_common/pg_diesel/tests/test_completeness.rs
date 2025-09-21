//! Submodule testing that all relevant postgres tables exist in the schema.

use diesel::PgConnection;
use pg_diesel::models::Table;
use reference_docker::reference_docker_with_connection;

#[tokio::test]
/// Test retrieval of extensions from a column
async fn test_schema_completeness() {
    let database_name = "test_schema_completeness";
    let (docker, mut conn) = reference_docker_with_connection::<PgConnection>(database_name, 45658)
        .await
        .expect("Failed to start container");

	// We load the schema.rs document
	let diesel_schema = include_str!("../../pg_diesel/src/schema.rs");

    for schema in ["pg_toast", "pg_catalog", "information_schema"] {
        let tables =
            Table::load_all(&mut conn, database_name, "pg_catalog").expect("Failed to load tables");
		for table in tables {
			let needle = format!("{}.{}", table.table_schema, table.table_name);
			assert!(diesel_schema.contains(&needle), "Table {table} not found in schema.rs");
		}
    }

    docker.stop().await.unwrap();
}
