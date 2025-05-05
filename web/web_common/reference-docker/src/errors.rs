//! Error enumeration for the `reference-docker` crate.

use testcontainers::TestcontainersError;

#[derive(Debug)]
/// Error enumeration for the `reference-docker` crate.
pub enum Error {
    /// Something failed while setting up the docker container.
    Container(TestcontainersError),
    /// Something failed while trying to connect to the database.
    Diesel(diesel::ConnectionError),
}

impl From<TestcontainersError> for Error {
    fn from(err: TestcontainersError) -> Self {
        Error::Container(err)
    }
}

impl From<diesel::ConnectionError> for Error {
    fn from(err: diesel::ConnectionError) -> Self {
        Error::Diesel(err)
    }
}
