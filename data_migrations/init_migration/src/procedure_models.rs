//! Submodule defining the init migrations for the procedure models.

use core_structures::User;
use diesel_async::AsyncPgConnection;

mod emi_solvent_procedure;
mod ethanol_70_percent;

pub(crate) async fn init_procedure_models(
    user: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    ethanol_70_percent::init_ethanol_70_percent(user, portal_conn).await?;
    emi_solvent_procedure::init_emi_solvent_procedure_models(user, portal_conn).await?;
    Ok(())
}
