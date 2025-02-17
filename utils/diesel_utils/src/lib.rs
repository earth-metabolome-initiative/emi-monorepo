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
