//! Submodule defining the DBGI plan procedure template.

use core_structures::{
    ProcedureTemplate, User, tables::insertables::ProcedureTemplateSettable,
    traits::AppendProcedureTemplate,
};
use web_common_traits::database::{DispatchableInsertableVariant, Insertable};

use crate::procedure_templates::{
    init_data_enrichment_procedure, init_negative_ionization_lcms_procedure,
    init_positive_ionization_lcms_procedure,
};
mod organism_observation_procedure;
mod part_of_organism_collection_procedure;
mod sample_processing_procedures;
use organism_observation_procedure::organism_observation_procedure;
use part_of_organism_collection_procedure::part_of_organism_collection;

/// The name of the DBGI plan procedure template.
pub const DBGI_PLAN: &str = "DBGI Plan";

/// Initializes the DBGI plan procedure template in the database.
///
/// # Arguments
///
/// * `user` - The user who is creating the procedure template.
/// * `conn` - The database connection to use for the insertion.
///
/// # Panics
///
/// * If the connection fails to insert the procedure template.
/// * If the procedure template building fails.
///
/// # Errors
///
/// * If the connection to the database fails.
pub fn dbgi_plan(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<ProcedureTemplate> {
    let dbgi_plan = ProcedureTemplate::new()
        .name(DBGI_PLAN)?
        .description("DBGI Plan procedure template")?
        .created_by(user)?
        .insert(user.id, conn)?;

    let (observation_procedure, _organism) = organism_observation_procedure(user, conn)?;
    let (part_of_organism_collection, cct) = part_of_organism_collection(user, conn)?;
    let sample_processing_procedure =
        sample_processing_procedures::init_dbgi_sample_processing_procedures(user, &cct, conn)?;
    let positive_lcms_procedure = init_positive_ionization_lcms_procedure(user, conn)?;
    let negative_lcms_procedure = init_negative_ionization_lcms_procedure(user, conn)?;
    let data_enrichment = init_data_enrichment_procedure(user, conn)?;

    dbgi_plan.extend(
        &[
            observation_procedure.into(),
            part_of_organism_collection.into(),
            sample_processing_procedure.into(),
            positive_lcms_procedure.into(),
            negative_lcms_procedure.into(),
            data_enrichment.into(),
        ],
        user,
        conn,
    )?;

    Ok(dbgi_plan)
}
