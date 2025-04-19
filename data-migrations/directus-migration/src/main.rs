mod codegen;
mod error;
mod migrations;
use diesel_async::{AsyncConnection, AsyncPgConnection};
use migrations::{
    ensure_instrument_categories_compatibility, insert_missing_brands,
    insert_missing_instrument_models, insert_missing_instruments, insert_missing_users,
};

const DIRECTUS_DATABASE_NAME: &str = "directus";
const DIRECTUS_DATABASE_PASSWORD: &str = "directus_dbgi";
const DIRECTUS_DATABASE_USER: &str = "directus";
const DIRECTUS_DATABASE_PORT: u16 = 5434;
const DIRECTUS_HOSTNAME: &str = "134.21.20.118";
const DIRECTUS_DATABASE_URL: &str = const_format::formatcp!(
    "postgres://{DIRECTUS_DATABASE_USER}:{DIRECTUS_DATABASE_PASSWORD}@{DIRECTUS_HOSTNAME}:{DIRECTUS_DATABASE_PORT}/{DIRECTUS_DATABASE_NAME}",
);

const PORTAL_DATABASE_NAME: &str = "development.db";
const PORTAL_DATABASE_PASSWORD: &str = "password";
const PORTAL_DATABASE_USER: &str = "user";
const PORTAL_DATABASE_PORT: u16 = 15032;
const PORTAL_DATABASE_URL: &str = const_format::formatcp!(
    "postgres://{PORTAL_DATABASE_USER}:{PORTAL_DATABASE_PASSWORD}@localhost:{PORTAL_DATABASE_PORT}/{PORTAL_DATABASE_NAME}",
);

/// Executes the data migration from the Directus database to the portal
/// database.
async fn transact_migration(
    directus_conn: &mut AsyncPgConnection,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), error::Error> {
    insert_missing_users(directus_conn, portal_conn).await?;
    insert_missing_brands(directus_conn, portal_conn).await?;
    ensure_instrument_categories_compatibility(directus_conn, portal_conn).await?;
    insert_missing_instrument_models(directus_conn, portal_conn).await?;
    insert_missing_instruments(directus_conn, portal_conn).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let mut directus_conn = AsyncPgConnection::establish(DIRECTUS_DATABASE_URL).await?;
    let mut portal_conn = AsyncPgConnection::establish(PORTAL_DATABASE_URL).await?;

    portal_conn
        .transaction(|portal_conn| {
            Box::pin(async move { transact_migration(&mut directus_conn, portal_conn).await })
        })
        .await?;

    Ok(())
}
