use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel_migrations::{embed_migrations, MigrationHarness};

fn establish_test_connection(db_name: &str) -> PgConnection {
    let database_url = format!("postgres://localhost/{}", db_name);
    PgConnection::establish(&database_url).expect("Error connecting to test database")
}

fn setup_test_database(db_name: &str) {
    let conn = establish_test_connection("postgres");
    diesel::sql_query(format!("CREATE DATABASE {}", db_name)).execute(&conn).unwrap();
}

fn teardown_test_database(db_name: &str) {
    let conn = establish_test_connection("postgres");
    diesel::sql_query(format!("DROP DATABASE IF EXISTS {}", db_name)).execute(&conn).unwrap();
}

#[test]
fn test_example() {
    let db_name = "test_db";
    setup_test_database(db_name);

    let conn = establish_test_connection(db_name);
    conn.run_pending_migrations(embed_migrations!("./test_migrations")).unwrap();

    // Run your tests with `conn`

    teardown_test_database(db_name);
}