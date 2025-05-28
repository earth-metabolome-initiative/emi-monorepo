#![doc = include_str!("../README.md")]
use diesel_async::{AsyncConnection, AsyncPgConnection};

mod brands;
mod error;
mod login_providers;
mod plans;
mod procedure_models;
mod reagents;
mod trackable_categories;
mod users;

use brands::init_brands;
use login_providers::init_login_providers;
use plans::init_plans;
use procedure_models::init_procedure_models;
use reagents::init_reagents;
use trackable_categories::init_trackable_categories;
use users::init_root_user;

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
                init_trackable_categories(&darwin, portal_conn).await?;
                init_reagents(&darwin, portal_conn).await?;
                init_procedure_models(&darwin, portal_conn).await?;
                init_plans(&darwin, portal_conn).await?;
                Ok(())
            })
        })
        .await
}
