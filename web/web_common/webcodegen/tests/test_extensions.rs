//! Submodule to test code relative to retrieving metadata of Extensions from
//! postgres

mod utils;

use diesel_migrations_utils::prelude::MigrationDirectory;
use utils::*;
use webcodegen::*;

#[tokio::test]
/// Test retrieval of extensions from a column
async fn test_extensions_column() {
    let (docker, mut conn, _database_name) = setup_database_with_migrations(
        "test_extensions_column",
        MigrationDirectory::try_from("./test_extensions_migrations").unwrap(),
    )
    .await
    .unwrap();

    let uuid_extension = PgExtension::load("uuid-ossp", "public", &mut conn)
        .expect("Unable to query the database")
        .expect("Extension `uuid-ossp` not found");

    let pg_trgm_extension = PgExtension::load("pg_trgm", "public", &mut conn)
        .expect("Unable to query the database")
        .expect("Extension `pg_trgm` not found");

    let pgrx_validation_extension = PgExtension::load("pgrx_validation", "public", &mut conn)
        .expect("Unable to query the database")
        .expect("Extension `pgrx_validation` not found");

    // We print all of the available extensions.
    let all_extensions = PgExtension::load_all(&mut conn).expect("Unable to query the database");

    // We check that the loaded extensions appear in the set of aLL extensions
    for extension in &[&uuid_extension, &pg_trgm_extension, &pgrx_validation_extension] {
        assert!(all_extensions.contains(extension), "Extension {extension:?} not found");
    }

    let uuid_functions = uuid_extension
        .functions(&mut conn)
        .expect("Failed to query the database for `uuid-ossp` functions");

    let pg_trgm_functions = pg_trgm_extension
        .functions(&mut conn)
        .expect("Failed to query the database for pg_trgm functions");

    let pgrx_validation_functions = pgrx_validation_extension
        .functions(&mut conn)
        .expect("Failed to query the database for pgrx_validation functions");

    // We check that, for each of the loaded functions, the extension name is the
    // same as the one we loaded
    for (all_functions, expected_extension) in [
        (&uuid_functions, &uuid_extension),
        (&pg_trgm_functions, &pg_trgm_extension),
        (&pgrx_validation_functions, &pgrx_validation_extension),
    ] {
        for function in all_functions {
            assert_eq!(
                &function
                    .extension(&mut conn)
                    .expect("Failed to query the database")
                    .expect("Extension not found"),
                expected_extension
            );
        }
    }

    docker.stop().await.unwrap();
}
