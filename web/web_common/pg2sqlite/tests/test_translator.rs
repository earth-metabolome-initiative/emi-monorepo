//! Test translating the core migrations used in the `core_structures` crate.

use diesel::{Connection, SqliteConnection, connection::SimpleConnection};
use pg2sqlite::prelude::Pg2Sqlite;

#[test]
/// Test translating the core migrations used in the `core_structures` crate.
fn test_translator() {
    let translated_migrations = Pg2Sqlite::default()
        .remove_unsupported_check_constraints()
        .verbose()
        .ups("../../../data_migrations/init_db/migrations")
        .expect("Failed to load the migrations")
        .translate()
        .expect("Failed to translate the migrations");

    assert_eq!(translated_migrations.len(), 60);

    // We try to parse the translated migrations using the `sqlparser` crate,
    // for the `SQLite` dialect.
    for translated_migration in &translated_migrations {
        sqlparser::parser::Parser::parse_sql(
            &sqlparser::dialect::SQLiteDialect {},
            &translated_migration.to_string(),
        )
        .expect("Failed to parse the translated migration");
    }

    // We create a testcontainer `Docker` for `SQLite` and run the translated
    // migrations, considering the severe limitations of our target use case
    // which is `WASM + SQLite`.
    let mut connection = SqliteConnection::establish(":memory:")
        .expect("Failed to establish a connection to the SQLite database");

    for (i, translated_migration) in translated_migrations.iter().enumerate() {
        connection.batch_execute(&translated_migration.to_string()).unwrap_or_else(|_| {
            panic!("Failed to run the migration {i}/{}", translated_migrations.len())
        });
    }
}
