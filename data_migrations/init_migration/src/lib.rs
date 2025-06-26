#![doc = include_str!("../README.md")]
use diesel::{Connection, PgConnection};

mod brands;
mod error;
mod login_providers;
mod procedure_models;
mod trackables;
mod users;

pub(crate) use brands::{acros_organics, fisher_scientific, fisherbrand, greiner_bio_one};
use login_providers::init_login_providers;
pub use procedure_models::DBGI_PLAN;
use procedure_models::init_procedure_models;
use trackables::init_trackables;
use users::init_root_user;

/// Executes the init migration.
///
/// # Arguments
///
/// * `conn` - The connection to the database.
///
/// # Errors
///
/// * If the connection to the database fails.
pub fn init_migration(conn: &mut PgConnection) -> Result<(), error::Error> {
    conn.transaction(|conn| {
        init_login_providers(conn)?;
        let darwin = init_root_user(conn)?;
        init_trackables(&darwin, conn);
        init_procedure_models(&darwin, conn);
        Ok(())
    })
}
