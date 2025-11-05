//! Submodule defining the EMI plan procedure template.
use core_structures::{ProcedureTemplate, User, tables::insertables::ProcedureTemplateSettable};

use core_structures::traits::AppendProcedureTemplate;
use diesel::OptionalExtension;
use web_common_traits::database::{DispatchableInsertableVariant, Insertable};

use crate::procedure_templates::init_data_enrichment_procedure;
use crate::procedure_templates::init_negative_ionization_lcms_procedure;
use crate::procedure_templates::init_positive_ionization_lcms_procedure;
use crate::procedure_templates::organism_observation_procedure;
use crate::procedure_templates::shared_sub_procedure_templates::full_organism_collection_procedure;
use crate::procedure_templates::shared_sub_procedure_templates::part_of_organism_collection;
use crate::procedure_templates::shared_sub_procedure_templates::{
    full_sample_processing_procedures, partial_sample_processing_procedures,
};

/// Initializes the EMI plan procedure template in the database.
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
pub fn emi_procedure_template(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<ProcedureTemplate> {
    const EMI_PT: &str = "EMI Procedure Template";

    if let Some(procedure) = ProcedureTemplate::from_name(EMI_PT, conn).optional()? {
        return Ok(procedure);
    }

    let emi_procedure_template = ProcedureTemplate::new()
        .name(EMI_PT)?
        .description("EMI Plan procedure template")?
        .created_by(user)?
        .insert(user.id, conn)?;

    let (observation_procedure, _organism, _phone) = organism_observation_procedure(user, conn)?;
    let (part_of_organism_collection, cct) = part_of_organism_collection(user, conn)?;
    let sample_processing_procedure = partial_sample_processing_procedures(user, &cct, conn)?;
    let positive_lcms_procedure = init_positive_ionization_lcms_procedure(user, conn)?;
    let negative_lcms_procedure = init_negative_ionization_lcms_procedure(user, conn)?;
    let data_enrichment = init_data_enrichment_procedure(user, conn)?;

    emi_procedure_template.extend(
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

    Ok(emi_procedure_template)
}

/// Initializes the EMI Insect Procedure Template in the database.
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
pub fn emi_insect_procedure_template(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<ProcedureTemplate> {
    const EMI_INSECT_PT: &str = "EMI Insect Procedure Template";

    if let Some(procedure) = ProcedureTemplate::from_name(EMI_INSECT_PT, conn).optional()? {
        return Ok(procedure);
    }

    let emi_insect_procedure_template = ProcedureTemplate::new()
        .name(EMI_INSECT_PT)?
        .description("EMI Insect procedure template")?
        .created_by(user)?
        .insert(user.id, conn)?;

    let (observation_procedure, _organism, _phone) = organism_observation_procedure(user, conn)?;
    let (full_organism_collection_procedure, cct) = full_organism_collection_procedure(user, conn)?;
    let sample_processing_procedure = full_sample_processing_procedures(user, &cct, conn)?;
    let positive_lcms_procedure = init_positive_ionization_lcms_procedure(user, conn)?;
    let negative_lcms_procedure = init_negative_ionization_lcms_procedure(user, conn)?;
    let data_enrichment = init_data_enrichment_procedure(user, conn)?;

    emi_insect_procedure_template.extend(
        &[
            observation_procedure.into(),
            full_organism_collection_procedure.into(),
            sample_processing_procedure.into(),
            positive_lcms_procedure.into(),
            negative_lcms_procedure.into(),
            data_enrichment.into(),
        ],
        user,
        conn,
    )?;

    Ok(emi_insect_procedure_template)
}
