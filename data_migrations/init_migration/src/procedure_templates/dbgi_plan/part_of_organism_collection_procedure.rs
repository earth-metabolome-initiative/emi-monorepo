//! Submodule defining the Ethanol 70 percent procedure creation.

use core_structures::{
    HarvestingProcedureTemplate, PackagingProcedureTemplate, ProcedureTemplate,
    ProcedureTemplateAssetModel, StorageProcedureTemplate, User,
    tables::insertables::{
        HarvestingProcedureTemplateSettable, PackagingProcedureTemplateSettable,
        ProcedureTemplateSettable, StorageProcedureTemplateSettable,
    },
    traits::AppendProcedureTemplate,
};
use diesel::OptionalExtension;
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::{
    procedure_template_asset_models::{
        coffee_wrapper::coffee_wrapper_builder, conical_tubes::cct_builder,
        conical_tubes_box::cct_box_builder, organism::sample_builder,
    }, procedure_templates::dbgi_plan::organism_observation_procedure::organism_observation_procedure,
};

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
pub(crate) fn part_of_organism_collection(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<(ProcedureTemplate, ProcedureTemplateAssetModel)> {
    let name = "Part of Organism Collection";
    let storage_procedure_name = "Place in conical centrifugal tube";

    if let Some(existing) = ProcedureTemplate::from_name(name, conn).optional()? {
        let storage_procedure = StorageProcedureTemplate::from_name(storage_procedure_name, conn)?;
        let cct = storage_procedure.procedure_template_stored_into_model(conn)?;
        return Ok((existing, cct));
    }

    let (_, organism) = organism_observation_procedure(user, conn)?;

    let collection = ProcedureTemplate::new()
        .name(name)?
        .description(
            "procedure template to collect part of organisms, such as leaves, stems, or roots.",
        )?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    // Remind the user to wear gloves
    let gloves_reminder = ProcedureTemplate::new()
        .name("Wear gloves")?
        .description("Please wear gloves to avoid contamination and protect yourself.")?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    // Remind the user to sterilize / clean the scalpel and gloves with ethanol 70
    // percent
    let sterilization_reminder = ProcedureTemplate::new()
        .name("Sterilize scalpel and gloves")?
        .description(
            "Please sterilize the scalpel and gloves with ethanol 70 percent to avoid contamination.",
        )?
        .created_by(user.id)?
        .insert(user.id, conn)?;


    // Harvest the sample from the sample source
    let sample_harvesting = HarvestingProcedureTemplate::new()
        .name("Harvest sample")?
        .description("Harvest the cut part of the organism as a sample.")?
        .created_by(user.id)?
        .procedure_template_sample_source_model(&organism)?
        .procedure_template_sample_model(sample_builder(user, conn)?)?
        .insert(user.id, conn)?;
    let sample: i32 = sample_harvesting.procedure_template_sample_model;

    // Wrapping procedure with coffee filter paper
    let coffee_filter_wrapping = PackagingProcedureTemplate::new()
        .name("Wrap in coffee filter paper")?
        .description(
            "Wrap the cut part of the organism in a coffee filter paper to protect it during transport.",
        )?
        .created_by(user.id)?
        .procedure_template_packaged_with_model(coffee_wrapper_builder(user, conn)?)?
        .procedure_template_sample_model(sample)?
        .insert(user.id, conn)?;
    let coffee_wrapper: i32 = coffee_filter_wrapping.procedure_template_packaged_with_model;

    // Placing the wrapped sample in the conical centrifugal tube
    let place_in_tube = StorageProcedureTemplate::new()
        .name(storage_procedure_name)?
        .description(
            "Place the wrapped sample in a conical centrifugal tube for storage and transport.",
        )?
        .procedure_template_stored_into_model(cct_builder(user, conn)?)?
        .procedure_template_stored_asset_model(coffee_wrapper)?
        .created_by(user.id)?
        .insert(user.id, conn)?;
    let cct = place_in_tube.procedure_template_stored_into_model(conn)?;

    // Put it in the storage box
    let place_in_storage_box = StorageProcedureTemplate::new()
        .name("Place in storage box")?
        .description(
            "Place the conical centrifugal tube with the sample in a storage box for long-term storage.",
        )?
        .procedure_template_stored_into_model(
            cct_box_builder(user, conn)?
        )?
        .procedure_template_stored_asset_model(cct.id)?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    collection.extend(
        &[
            &gloves_reminder,
            &sterilization_reminder,
            &sample_harvesting.procedure_template(conn)?,
            &coffee_filter_wrapping.procedure_template(conn)?,
            &place_in_tube.procedure_template(conn)?,
            &place_in_storage_box.procedure_template(conn)?,
        ],
        user,
        conn,
    )?;

    Ok((collection, cct))
}
