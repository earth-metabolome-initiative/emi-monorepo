//! Submodule defining the init migrations for the procedure models.

use core_structures::User;
use diesel_async::AsyncPgConnection;

mod aliquoting_mass_spec_extracts_procedure;
mod collection_procedure;
mod data_enrichment_procedure;
mod emi_solvent_procedure;
mod ethanol_70_percent;
mod extraction_procedure;
mod freeze_drying_procedure;
mod freezing_procedure;
mod mass_spec_procedure;
mod precollection_procedure;
mod shared_step_models;
mod weighing_procedure;

pub(crate) use aliquoting_mass_spec_extracts_procedure::ALIQUOTING_MASS_SPEC_EXTRACTS;
pub(crate) use collection_procedure::COLLECTION_PROCEDURE;
pub(crate) use data_enrichment_procedure::DATA_ENRICHMENT_PROCEDURE;
pub(crate) use extraction_procedure::EXTRACTION_PROCEDURE;
pub(crate) use freeze_drying_procedure::FREEZE_DRYING;
pub(crate) use freezing_procedure::FREEZING_PROCEDURE;
pub(crate) use mass_spec_procedure::MASS_SPEC_PROCEDURE;
pub(crate) use precollection_procedure::PRECOLLECTION_PROCEDURE;
pub(crate) use weighing_procedure::WEIGHING_PROCEDURE;

pub(crate) async fn init_procedure_models(
    user: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    ethanol_70_percent::init_ethanol_70_percent(user, portal_conn).await?;
    emi_solvent_procedure::init_emi_solvent_procedure_models(user, portal_conn).await?;
    precollection_procedure::init_precollection_procedure_model(user, portal_conn).await?;
    collection_procedure::init_collection_procedure_model(user, portal_conn).await?;

    freezing_procedure::init_freezing_procedure(user, portal_conn).await?;
    freeze_drying_procedure::init_freeze_drying_procedure(user, portal_conn).await?;
    weighing_procedure::init_weighing_procedure(user, portal_conn).await?;
    extraction_procedure::init_extraction_procedure(user, portal_conn).await?;
    aliquoting_mass_spec_extracts_procedure::init_aliquoting_mass_spec_extracts_procedure(
        user,
        portal_conn,
    )
    .await?;
    mass_spec_procedure::init_mass_spec_procedure(user, portal_conn).await?;
    data_enrichment_procedure::init_data_enrichment_procedure(user, portal_conn).await?;

    Ok(())
}
