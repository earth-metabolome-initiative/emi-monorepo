//! Submodule defining the init migrations for the procedure models.

use core_structures::User;
use diesel::PgConnection;

// mod aliquoting_mass_spec_extracts_procedure;
// mod collection_procedure;
// mod data_enrichment_procedure;
mod emi_solvent_procedure;
mod ethanol_70_percent;
// mod extraction_procedure;
// mod freeze_drying_procedure;
// mod freezing_procedure;
// mod mass_spec_procedure;
// mod precollection_procedure;
// mod shared_step_models;
// mod weighing_procedure;
mod dbgi_plan;
pub use dbgi_plan::DBGI_PLAN;
// pub(crate) use
// aliquoting_mass_spec_extracts_procedure::ALIQUOTING_MASS_SPEC_EXTRACTS;
// pub(crate) use collection_procedure::COLLECTION_PROCEDURE;
// pub(crate) use data_enrichment_procedure::DATA_ENRICHMENT_PROCEDURE;
// pub(crate) use extraction_procedure::EXTRACTION_PROCEDURE;
// pub(crate) use freeze_drying_procedure::FREEZE_DRYING;
// pub(crate) use freezing_procedure::FREEZING_PROCEDURE;
// pub(crate) use mass_spec_procedure::MASS_SPEC_PROCEDURE;
// pub(crate) use precollection_procedure::{PRECOLLECTION_PROCEDURE,
// REFRIGERATED_SAMPLE_BOX}; pub(crate) use
// weighing_procedure::WEIGHING_PROCEDURE;

pub(crate) fn init_procedure_models(
    user: &User,
    portal_conn: &mut PgConnection,
) -> Result<(), crate::error::Error> {
    emi_solvent_procedure::init_emi_solvent_procedure_models(user, portal_conn)?;
    ethanol_70_percent::init_ethanol_70_percent(user, portal_conn)?;
    dbgi_plan::init_dbgi_plan(user, portal_conn)?;

    // precollection_procedure::init_precollection_procedure_model(user,
    // portal_conn)?;
    // collection_procedure::init_collection_procedure_model(user, portal_conn)?;

    // freezing_procedure::init_freezing_procedure(user, portal_conn)?;
    // freeze_drying_procedure::init_freeze_drying_procedure(user, portal_conn)?;
    // weighing_procedure::init_weighing_procedure(user, portal_conn)?;
    // extraction_procedure::init_extraction_procedure(user, portal_conn)?;
    // aliquoting_mass_spec_extracts_procedure::init_aliquoting_mass_spec_extracts_procedure(
    //     user,
    //     portal_conn,
    // )?;
    // mass_spec_procedure::init_mass_spec_procedure(user, portal_conn)?;
    // data_enrichment_procedure::init_data_enrichment_procedure(user,
    // portal_conn)?;

    Ok(())
}
