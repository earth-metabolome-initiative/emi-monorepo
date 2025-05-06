#![doc = include_str!("../README.md")]
use diesel_async::{AsyncConnection, AsyncPgConnection};

mod brands;
mod commercial_products;
mod error;
mod login_providers;
mod procedure_models;
mod step_models;
mod users;

use brands::init_brands;
use commercial_products::init_commercial_products;
use login_providers::init_login_providers;
use procedure_models::init_procedure_models;
use step_models::init_step_models;
use users::init_root_user;

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
        .transaction(|portal_conn| {
            Box::pin(async move {
                init_login_providers(portal_conn).await?;
                let darwin = init_root_user(portal_conn).await?;
                init_brands(&darwin, portal_conn).await?;
                init_commercial_products(&darwin, portal_conn).await?;
                init_step_models(&darwin, portal_conn).await?;
                init_procedure_models(&darwin, portal_conn).await?;
                Ok(())
            })
        })
        .await
}
