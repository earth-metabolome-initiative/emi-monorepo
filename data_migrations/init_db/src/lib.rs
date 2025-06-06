#![doc = include_str!("../README.md")]

mod consistency_constraints;
mod csv_migrations;
mod errors;
mod init;
mod migrations;

pub use init::init_database;

/// Returns whether a database with the given name exists.
///
/// # Arguments
///
/// * `database_name`: The name of the database to check.
/// * `connection`: A mutable reference to an asynchronous `PostgreSQL`
///   connection.
///
/// # Errors
///
/// * If the query fails, an error of type `errors::Error` is returned.
pub fn database_exists(
    database_name: &str,
    connection: &mut diesel::PgConnection,
) -> Result<bool, errors::Error> {
    use diesel::{QueryableByName, RunQueryDsl, sql_query, sql_types::Integer};

    #[derive(QueryableByName)]
    struct Exists {
        #[diesel(sql_type = Integer)]
        exists: i32,
    }

    Ok(sql_query(format!("SELECT 1 as exists FROM pg_database WHERE datname = '{database_name}'",))
        .get_result::<Exists>(connection)?
        .exists
        == 1)
}
