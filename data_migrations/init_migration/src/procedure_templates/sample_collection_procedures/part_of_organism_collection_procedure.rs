//! Submodule defining the Ethanol 70 percent procedure creation.

use core_structures::{
    PackagingProcedureTemplate, ProcedureTemplate, StorageProcedureTemplate, User,
    tables::insertables::{
        PackagingProcedureTemplateBuildable, ProcedureTemplateBuildable,
        StorageProcedureTemplateBuildable,
    },
    traits::{AppendProcedureTemplate, ChildOptions, ParentProcedureTemplate},
};
use diesel::OptionalExtension;
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::procedure_template_asset_models::{
    coffee_wrapper::coffee_wrapper_builder, conical_tubes::cct_builder,
    conical_tubes_box::cct_box_builder, organism::sample_builder,
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
pub(crate) fn init_part_of_organism_collection(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<ProcedureTemplate> {
    let name = "Part of Organism Collection";

    if let Some(existing) = ProcedureTemplate::from_name(name, conn).optional()? {
        return Ok(existing);
    }

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

    // Cut the part of the organism to be collected with a sterile scalpel
    let cut_part = ProcedureTemplate::new()
        .name("Cut part of organism")?
        .description(
            "Use a sterile scalpel to cut the desired part of the organism, such as leaves, stems, or roots.",
        )?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    // Wrapping procedure with coffee filter paper
    let coffee_filter_wrapping = PackagingProcedureTemplate::new()
        .name("Wrap in coffee filter paper")?
        .description(
            "Wrap the cut part of the organism in a coffee filter paper to protect it during transport.",
        )?
        .created_by(user.id)?
        .procedure_template_packaged_with_model(coffee_wrapper_builder(user, conn)?)?
        .procedure_template_sample_model(sample_builder(user, conn)?)?
        .insert(user.id, conn)?;

    // Placing the wrapped sample in the conical centrifugal tube
    let place_in_tube = StorageProcedureTemplate::new()
        .name("Place in conical centrifugal tube")?
        .description(
            "Place the wrapped sample in a conical centrifugal tube for storage and transport.",
        )?
        .procedure_template_stored_into_model(cct_builder(user, conn)?)?
        .procedure_template_stored_asset_model(coffee_wrapper_builder(user, conn)?)?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    // Put it in the storage box
    let place_in_storage_box = StorageProcedureTemplate::new()
        .name("Place in storage box")?
        .description(
            "Place the conical centrifugal tube with the sample in a storage box for long-term storage.",
        )?
        .procedure_template_stored_into_model(
            cct_box_builder(user, conn)?
        )?
        .procedure_template_stored_asset_model(cct_builder(user, conn)?)?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    for procedure in [
        &gloves_reminder,
        &sterilization_reminder,
        &cut_part,
        &coffee_filter_wrapping.procedure_template(conn)?,
        &place_in_tube.procedure_template(conn)?,
        &place_in_storage_box.procedure_template(conn)?,
    ] {
        collection.child(procedure, ChildOptions::default().inherit_asset_models(), user, conn)?;
    }

    collection.extend(
        &[
            &gloves_reminder,
            &sterilization_reminder,
            &cut_part,
            &coffee_filter_wrapping.procedure_template(conn)?,
            &place_in_tube.procedure_template(conn)?,
            &place_in_storage_box.procedure_template(conn)?,
        ],
        user,
        conn,
    )?;

    Ok(collection)
}
