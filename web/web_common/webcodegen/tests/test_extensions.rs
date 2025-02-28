//! Submodule to test code relative to retrieving metadata of Extensions from postgres

mod utils;

use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use utils::*;
use webcodegen::*;

const CHECK_CONSTRAINT_TEST_MIGRATIONS: EmbeddedMigrations =
    embed_migrations!("./test_extensions_migrations");

#[tokio::test]
/// Test retrieval of extensions from a column
async fn test_extensions_column() {
    let (docker, mut conn, _database_name) =
        setup_database_with_migrations("test_extensions_column", CHECK_CONSTRAINT_TEST_MIGRATIONS)
            .await
            .unwrap();

    let uuid_extension = PgExtension::load("uuid-ossp", "public", &mut conn)
        .expect("Unable to query the database")
        .expect("Extension `uuid-ossp` not found");

    let pg_trgm_extension = PgExtension::load("pg_trgm", "public", &mut conn)
        .expect("Unable to query the database")
        .expect("Extension `pg_trgm` not found");

    // We print all of the available extensions.
    let all_extensions = PgExtension::load_all(&mut conn).expect("Unable to query the database");

    // We check that the loaded extensions appear in the set of aLL extensions
    for extension in [&uuid_extension, &pg_trgm_extension].iter() {
        assert!(all_extensions.contains(extension), "Extension {:?} not found", extension);
    }

    let uuid_functions = uuid_extension
        .functions(&mut conn)
        .expect("Failed to query the database for `uuid-ossp` functions");

    let pg_trgm_functions = pg_trgm_extension
        .functions(&mut conn)
        .expect("Failed to query the database for pg_trgm functions");

    // We check that, for each of the loaded functions, the extension name is the same as the one we
    // loaded
    for (all_functions, expected_extension) in
        [(&uuid_functions, &uuid_extension), (&pg_trgm_functions, &pg_trgm_extension)]
    {
        for function in all_functions.iter() {
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
