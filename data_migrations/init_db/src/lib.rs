#![doc = include_str!("../README.md")]

mod consistency_constraints;
mod csv_migrations;
mod errors;
mod init;
mod migrations;
pub use init::init_database;

/// Returns the number of tables in the given database.
///
/// # Arguments
///
/// * `connection`: A mutable reference to an asynchronous `PostgreSQL`
///   connection.
///
/// # Errors
///
/// * If the query fails, an error of type `errors::Error` is returned.
pub fn number_of_tables(connection: &mut diesel::PgConnection) -> Result<i64, errors::Error> {
    use diesel::{QueryableByName, RunQueryDsl, sql_query};

    #[derive(Debug, QueryableByName)]
    struct TableCount {
        #[diesel(sql_type = diesel::sql_types::BigInt)]
        count: i64,
    }

    Ok(sql_query("SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public'")
        .load::<TableCount>(connection)?
        .pop()
        .map_or(0, |tc| tc.count))
}

/// Returns whether the given database is empty (i.e., has no tables).
///
///  # Arguments
///
/// * `connection`: A mutable reference to an asynchronous `PostgreSQL`
///   connection.
///
///  # Errors
///
/// * If the query fails, an error of type `errors::Error` is returned.
pub fn is_database_empty(connection: &mut diesel::PgConnection) -> Result<bool, errors::Error> {
    Ok(number_of_tables(connection)? == 0)
}
