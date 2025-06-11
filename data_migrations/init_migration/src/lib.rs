#![doc = include_str!("../README.md")]
use diesel::{Connection, PgConnection};

mod brands;
mod error;
mod login_providers;
mod procedure_models;
mod trackables;
mod users;

pub(crate) use brands::{acros_organics, fisher_scientific, greiner_bio_one};
use login_providers::init_login_providers;
pub use procedure_models::DBGI_PLAN;
use procedure_models::init_procedure_models;
use trackables::init_trackables;
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
pub fn init_migration(portal_conn: &mut PgConnection) -> Result<(), error::Error> {
    portal_conn.transaction(|portal_conn| {
        init_login_providers(portal_conn)?;
        let darwin = init_root_user(portal_conn)?;
        init_trackables(&darwin, portal_conn);
        init_procedure_models(&darwin, portal_conn);
        Ok(())
    })
}
