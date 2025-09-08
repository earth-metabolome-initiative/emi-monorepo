//! Submodule defining the init migrations for the procedure templates.

use core_structures::User;
use diesel::PgConnection;

mod dbgi_plan;
pub use dbgi_plan::init_dbgi_plan;
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
    init_ethanol_70_percent, sample_extraction_solvent_procedure,
};
pub(crate) use sample_collection_procedures::init_part_of_organism_collection;

pub(crate) fn init_procedure_templates(user: &User, conn: &mut PgConnection) -> anyhow::Result<()> {
    // emi_solvent_procedure::init_emi_solvent_procedure_templates(user, conn)?;
    // ethanol_70_percent::init_ethanol_70_percent(user, conn)?;
    dbgi_plan::init_dbgi_plan(user, conn)?;

    // precollection_procedure::init_precollection_procedure_template(user,
    // conn)?;
    // collection_procedure::init_collection_procedure_template(user, conn)?;

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

    Ok(())
}
