//! Submodule defining the init migrations for the procedure models.

use core_structures::User;
use diesel::PgConnection;

// mod aliquoting_mass_spec_extracts_procedure;
// mod collection_procedure;
// mod data_enrichment_procedure;
// mod emi_solvent_procedure;
// mod ethanol_70_percent;
// mod extraction_procedure;
// mod freeze_drying_procedure;
// mod freezing_procedure;
// mod mass_spec_procedure;
// mod precollection_procedure;
// mod shared_step_models;
// mod weighing_procedure;
mod dbgi_plan;
pub use dbgi_plan::DBGI_PLAN;
mod analysis_procedures;
mod collection_preparation_procedures;
mod data_enrichment_procedures;
pub(crate) use data_enrichment_procedures::init_data_enrichment_procedure;
mod observation_procedures;
pub(crate) use observation_procedures::init_organism_observation_procedure;
mod sample_collection_procedures;
pub(crate) use analysis_procedures::{
    init_negative_ionization_lcms_procedure, init_positive_ionization_lcms_procedure,
};
pub(crate) use collection_preparation_procedures::{
    init_ethanol_70_percent, init_sample_extraction_solvent_procedure,
};
pub(crate) use sample_collection_procedures::{
    init_full_organism_collection, init_part_of_organism_collection,
};

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

pub(crate) fn init_procedure_models(user: &User, conn: &mut PgConnection) {
    // emi_solvent_procedure::init_emi_solvent_procedure_models(user, conn)?;
    // ethanol_70_percent::init_ethanol_70_percent(user, conn)?;
    dbgi_plan::init_dbgi_plan(user, conn);

    // precollection_procedure::init_precollection_procedure_model(user,
    // conn)?;
    // collection_procedure::init_collection_procedure_model(user, conn)?;

    // freezing_procedure::init_freezing_procedure(user, conn)?;
    // freeze_drying_procedure::init_freeze_drying_procedure(user, conn)?;
    // weighing_procedure::init_weighing_procedure(user, conn)?;
    // extraction_procedure::init_extraction_procedure(user, conn)?;
    // aliquoting_mass_spec_extracts_procedure::init_aliquoting_mass_spec_extracts_procedure(
    //     user,
    //     conn,
    // )?;
    // mass_spec_procedure::init_mass_spec_procedure(user, conn)?;
    // data_enrichment_procedure::init_data_enrichment_procedure(user,
    // conn)?;
}
