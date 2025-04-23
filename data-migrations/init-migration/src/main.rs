#![doc = include_str!("../README.md")]
use diesel_async::{AsyncConnection, AsyncPgConnection};

mod error;

const PORTAL_DATABASE_NAME: &str = "development.db";
const PORTAL_DATABASE_PASSWORD: &str = "password";
const PORTAL_DATABASE_USER: &str = "user";
const PORTAL_DATABASE_PORT: u16 = 15032;
const PORTAL_DATABASE_URL: &str = const_format::formatcp!(
    "postgres://{PORTAL_DATABASE_USER}:{PORTAL_DATABASE_PASSWORD}@localhost:{PORTAL_DATABASE_PORT}/{PORTAL_DATABASE_NAME}",
);

/// Executes the init migration.
async fn transact_migration(_portal_conn: &mut AsyncPgConnection) -> Result<(), error::Error> {
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let mut portal_conn = AsyncPgConnection::establish(PORTAL_DATABASE_URL).await?;

    portal_conn
        .transaction(|portal_conn| Box::pin(async move { transact_migration(portal_conn).await }))
        .await?;

    Ok(())
}
