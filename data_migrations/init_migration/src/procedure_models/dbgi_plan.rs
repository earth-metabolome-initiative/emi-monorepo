//! Submodule defining the DBGI plan procedure model.

use core_structures::{
    ProcedureModel, User,
    traits::{AppendProcedureModel, ChildOptions, ParentProcedureModel},
};
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::procedure_models::{
    init_data_enrichment_procedure, init_full_organism_collection,
    init_negative_ionization_lcms_procedure, init_organism_observation_procedure,
    init_part_of_organism_collection, init_positive_ionization_lcms_procedure,
};
mod dbgi_collection_preparation;
mod sample_processing_procedures;

/// The name of the DBGI plan procedure model.
pub const DBGI_PLAN: &str = "DBGI Plan";

/// Initializes the DBGI plan procedure model in the database.
///
/// # Arguments
///
/// * `user` - The user who is creating the procedure model.
/// * `conn` - The database connection to use for the insertion.
///
/// # Panics
///
/// * If the connection fails to insert the procedure model.
/// * If the procedure model building fails.
pub(super) fn init_dbgi_plan(user: &User, conn: &mut diesel::PgConnection) -> anyhow::Result<()> {
    let dbgi_plan = ProcedureModel::new()
        .name(DBGI_PLAN)?
        .description("DBGI Plan procedure model")?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let collection_preparation =
        dbgi_collection_preparation::init_dbgi_collection_preparation(user, conn)?;
    let sample_processing_procedure =
        sample_processing_procedures::init_dbgi_sample_processing_procedures(user, conn)?;
    let positive_lcms_procedure = init_positive_ionization_lcms_procedure(user, conn)?;
    let negative_lcms_procedure = init_negative_ionization_lcms_procedure(user, conn)?;
    let observation_procedure = init_organism_observation_procedure(user, conn)?;
    let part_of_organism_collection = init_part_of_organism_collection(user, conn)?;
    let data_enrichment = init_data_enrichment_procedure(user, conn)?;

    for procedure in [
        &collection_preparation,
        &observation_procedure,
        &part_of_organism_collection,
        &sample_processing_procedure,
        &positive_lcms_procedure,
        &negative_lcms_procedure,
        &data_enrichment,
    ] {
        dbgi_plan.child(procedure, ChildOptions::default().inherit_trackables(), user, conn)?;
    }

    dbgi_plan.extend(
        &[
            &collection_preparation,
            &observation_procedure,
            &part_of_organism_collection,
            &sample_processing_procedure,
            &positive_lcms_procedure,
            &negative_lcms_procedure,
            &data_enrichment,
        ],
        user,
        conn,
    )?;
    Ok(())
}
