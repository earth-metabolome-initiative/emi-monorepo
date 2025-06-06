//! Submodule providing an `AnyConnection` enum for multi-database support.

use diesel::QueryResult;

#[derive(diesel::MultiConnection)]
/// An enum representing a connection to either a PostgreSQL or SQLite database.
pub enum AnyConnection {
    /// A connection to a PostgreSQL database.
    Postgresql(diesel::PgConnection),
    /// A connection to a SQLite database.
    Sqlite(diesel::SqliteConnection),
}

impl From<diesel::PgConnection> for AnyConnection {
    fn from(conn: diesel::PgConnection) -> Self {
        AnyConnection::Postgresql(conn)
    }
}

impl From<diesel::SqliteConnection> for AnyConnection {
    fn from(conn: diesel::SqliteConnection) -> Self {
        AnyConnection::Sqlite(conn)
    }
}
