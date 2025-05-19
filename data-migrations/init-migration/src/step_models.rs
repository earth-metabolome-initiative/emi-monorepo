//! Submodule defining the init migrations for the step models.

use core_structures::User;
use diesel_async::AsyncPgConnection;

pub(crate) async fn init_step_models(
    darwin: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    Ok(())
}
