#![doc = include_str!("../README.md")]
use diesel::{Connection, PgConnection};

pub mod asset_models;
mod brands;
mod login_providers;
mod procedure_template_asset_models;
mod procedure_templates;
mod users;

use asset_models::init_compatibility_rules;
pub(crate) use brands::{fisherbrand, greiner_bio_one};
use login_providers::init_login_providers;
pub use procedure_templates::init_dbgi_plan;
pub use users::init_root_user;

/// Executes the init migration.
///
/// # Arguments
///
/// * `conn` - The connection to the database.
///
/// # Errors
///
/// * If the connection to the database fails.
pub fn init_migration(conn: &mut PgConnection) -> anyhow::Result<()> {
    conn.transaction(|conn| {
        let darwin = init_root_user(conn)?;
        init_login_providers(&darwin, conn)?;
        init_compatibility_rules(&darwin, conn)?;
        Ok(())
    })
}
