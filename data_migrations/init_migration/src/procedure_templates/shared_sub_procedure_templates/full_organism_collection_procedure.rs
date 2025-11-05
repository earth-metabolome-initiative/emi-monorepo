//! Submodule defining the Full Organism collection procedure.

use core_structures::{
    CleaningProcedureTemplate, HarvestingProcedureTemplate,
    PpeReminderProcedureTemplate, ProcedureTemplate,
    ProcedureTemplateAssetModel, StorageProcedureTemplate, User,
    tables::insertables::{
        CleaningProcedureTemplateSettable, HarvestingProcedureTemplateSettable,
        PpeReminderProcedureTemplateSettable,
        ProcedureTemplateSettable, StorageProcedureTemplateSettable,
    },
    traits::AppendProcedureTemplate,
};
use diesel::OptionalExtension;
use web_common_traits::database::{DispatchableInsertableVariant, Insertable};

use crate::{
    procedure_template_asset_models::{
        coffee_wrapper::coffee_wrapper_builder, conical_tubes::cct_builder,
        conical_tubes_box::cct_box_builder, organism::sample_builder,
        photograph::photograph_builder, ppe::glove_model_builder, tools::scalpel_model_builder,
    },
    procedure_templates::{
        collection_preparation_procedures::ethanol_70_percent_procedure,
        organism_observation_procedure,
    },
};
use crate::procedure_templates::shared_sub_procedure_templates::gloves_procedure_template;
use core_structures::tables::insertables::ProcedureTemplateAssetModelSettable;

/// Initializes the part of organism collection procedure template in the
/// database.
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
pub fn full_organism_collection_procedure(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<(ProcedureTemplate, ProcedureTemplateAssetModel)> {
    let name = "Full Organism Collection";
    const CCT: &str = "Insect Conical Centrifugal Tube";
    let storage_procedure_name = "Place insect in conical centrifugal tube";

    if let Some(existing) = ProcedureTemplate::from_name(name, conn).optional()? {
        let cct = ProcedureTemplateAssetModel::from_name_and_procedure_template(
            CCT,
            existing.procedure_template,
            conn,
        )?;
        return Ok((existing, cct));
    }

    let (_, organism, _phone) = organism_observation_procedure(user, conn)?;

    let collection = ProcedureTemplate::new()
        .name(name)?
        .description(
            "procedure template to collect full organisms, such as insects",
        )?
        .created_by(user)?
        .insert(user.id, conn)?;

    // Remind the user to wear gloves
    let gloves_procedure = gloves_procedure_template(user, conn)?;

    // Harvest the sample from the sample source
    let sample_harvesting = HarvestingProcedureTemplate::new()
        .name("Harvest insect")?
        .description("Harvest the organism as a sample.")?
        .created_by(user)?
        .procedure_template_sample_source_model(&organism)?
        .procedure_template_sample_model(sample_builder(user, conn)?)?
        .insert(user.id, conn)?;
    let sample: i32 = sample_harvesting.procedure_template_sample_model;

    // Placing the sample in the conical centrifugal tube
    let place_in_tube = StorageProcedureTemplate::new()
        .name(storage_procedure_name)?
        .description(
            "Place the insect in a conical centrifugal tube for storage and transport.",
        )?
        .procedure_template_stored_into_model(cct_builder(user, conn)?.name(CCT)?)?
        .procedure_template_stored_asset_model(sample)?
        .created_by(user)?
        .insert(user.id, conn)?;
    let cct = place_in_tube.procedure_template_stored_into_model(conn)?;

    // Put it in the storage box
    let place_in_storage_box = StorageProcedureTemplate::new()
        .name("Place insect in storage box")?
        .description(
            "Place the conical centrifugal tube with the sample in a storage box for long-term storage.",
        )?
        .procedure_template_stored_into_model(
            cct_box_builder(user, conn)?
        )?
        .procedure_template_stored_asset_model(&cct)?
        .created_by(user)?
        .insert(user.id, conn)?;

    collection.extend(
        &[
            gloves_procedure.into(),
            sample_harvesting.into(),
            place_in_tube.into(),
            place_in_storage_box.into(),
        ],
        user,
        conn,
    )?;

    Ok((collection, cct))
}
