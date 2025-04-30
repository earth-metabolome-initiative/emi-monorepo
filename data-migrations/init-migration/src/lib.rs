#![doc = include_str!("../README.md")]
use diesel_async::{AsyncConnection, AsyncPgConnection};

mod error;
mod login_providers;
use login_providers::init_login_providers;

#[allow(clippy::unused_async)]
/// Executes the init migration.
///
/// # Arguments
///
/// * `portal_conn` - The connection to the database.
///
/// # Errors
///
/// * If the connection to the database fails.
pub async fn init_migration(portal_conn: &mut AsyncPgConnection) -> Result<(), error::Error> {
    portal_conn
        .transaction(|portal_conn| Box::pin(async move { init_login_providers(portal_conn).await }))
        .await
}
