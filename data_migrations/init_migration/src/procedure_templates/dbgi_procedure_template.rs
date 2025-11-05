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
mod panel_observation_procedure;
mod cct_and_panel_picture_procedure;
use crate::procedure_templates::organism_observation_procedure;
use panel_observation_procedure::panel_observation_procedure;
use cct_and_panel_picture_procedure::cct_and_panel_picture_procedure;
use crate::procedure_templates::shared_sub_procedure_templates::part_of_organism_collection;
use crate::procedure_templates::shared_sub_procedure_templates::partial_sample_processing_procedures;
use diesel::OptionalExtension;

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
pub fn dbgi_procedure_template(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<ProcedureTemplate> {
    /// The name of the DBGI plan procedure template.
    const DBGI_PT: &str = "DBGI Procedure Template";

    if let Some(procedure) = ProcedureTemplate::from_name(DBGI_PT, conn).optional()? {
        return Ok(procedure);
    }

    let dbgi_procedure_template = ProcedureTemplate::new()
        .name(DBGI_PT)?
        .description("DBGI Plan procedure template")?
        .created_by(user)?
        .insert(user.id, conn)?;

    let (observation_procedure, _organism, _phone) = organism_observation_procedure(user, conn)?;
    let panel_observation_procedure = panel_observation_procedure(user, conn)?;
    let (part_of_organism_collection, cct) = part_of_organism_collection(user, conn)?;
    let cct_and_panel_picture_procedure =
        cct_and_panel_picture_procedure(user, conn)?;
    let sample_processing_procedure =
        partial_sample_processing_procedures(user, &cct, conn)?;
    let positive_lcms_procedure = init_positive_ionization_lcms_procedure(user, conn)?;
    let negative_lcms_procedure = init_negative_ionization_lcms_procedure(user, conn)?;
    let data_enrichment = init_data_enrichment_procedure(user, conn)?;

    dbgi_procedure_template.extend(
        &[
            observation_procedure.into(),
            panel_observation_procedure.into(),
            part_of_organism_collection.into(),
            cct_and_panel_picture_procedure.into(),
            sample_processing_procedure.into(),
            positive_lcms_procedure.into(),
            negative_lcms_procedure.into(),
            data_enrichment.into(),
        ],
        user,
        conn,
    )?;

    Ok(dbgi_procedure_template)
}
