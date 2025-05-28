//! Submodule defining the project plans.

use diesel_async::AsyncPgConnection;

pub mod dbgi_plan;

/// Initialized all plans.
pub(super) async fn init_plans(
    user: &core_structures::User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    dbgi_plan::init_dbgi_plan(user, portal_conn).await?;
    Ok(())
}
