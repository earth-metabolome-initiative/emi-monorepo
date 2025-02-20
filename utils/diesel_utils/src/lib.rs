//! Utility functions for working with Diesel.

/// Convert a `QueryResult` into a `Result<Option<T>, diesel::result::Error>`.
///
/// # Arguments
///
/// * `query_result` - The `QueryResult` to convert.
///
/// # Errors
///
/// If the `QueryResult` is an error other than `NotFound`, the error is
/// returned.
pub fn optional<T>(
    query_result: diesel::QueryResult<T>,
) -> Result<Option<T>, diesel::result::Error> {
    match query_result {
        Ok(record) => Ok(Some(record)),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(e) => Err(e),
    }
}

const DATABASE_NAME: &str = "development.db";
const DATABASE_PASSWORD: &str = "password";
const DATABASE_USER: &str = "user";
const DATABASE_PORT: u16 = 15032;

/// Establish a connection to a PostgreSQL database.
pub async fn pg_connect() -> Result<diesel_async::AsyncPgConnection, diesel::ConnectionError> {
    use diesel_async::AsyncConnection;
    let database_url = format!(
        "postgres://{DATABASE_USER}:{DATABASE_PASSWORD}@localhost:{DATABASE_PORT}/{DATABASE_NAME}",
    );

    let mut number_of_attempts = 0;

    while let Err(e) = diesel_async::AsyncPgConnection::establish(&database_url).await {
        eprintln!("Failed to establish connection: {:?}", e);
        std::thread::sleep(std::time::Duration::from_secs(1));
        if number_of_attempts > 10 {
            eprintln!("Failed to establish connection after 10 attempts");
            std::process::exit(1);
        }
        number_of_attempts += 1;
    }

    diesel_async::AsyncPgConnection::establish(&database_url).await
}
