//! Submodule defining the init migrations for the step models.

use core_structures::User;
use diesel_async::AsyncPgConnection;

mod aliquoting_step_model;
mod cleaning_step_model_95;
mod sterilizing_step_model_70;

pub(crate) async fn init_step_models(
    darwin: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    sterilizing_step_model_70::init_sterilizing_step_model_70(darwin, portal_conn).await?;
    cleaning_step_model_95::init_cleaning_step_model_95(darwin, portal_conn).await?;
    aliquoting_step_model::init_aliquoting_step_model(darwin, portal_conn).await?;

    Ok(())
}
