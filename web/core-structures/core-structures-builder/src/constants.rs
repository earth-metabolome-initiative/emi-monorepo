//! Submodule providing the constants used in the database connection.

pub(crate) const DATABASE_NAME: &str = "development.db";
pub(crate) const DATABASE_PASSWORD: &str = "password";
pub(crate) const DATABASE_USER: &str = "user";
pub(crate) const DATABASE_PORT: u16 = 15032;
pub(crate) const DATABASE_URL: &str = const_format::formatcp!(
    "postgres://{DATABASE_USER}:{DATABASE_PASSWORD}@localhost:{DATABASE_PORT}/{DATABASE_NAME}",
);
