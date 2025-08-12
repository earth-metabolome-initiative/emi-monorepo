//! Submodule defining the Ethanol 70 percent procedure creation.

use core_structures::{
    PackagingProcedureModel, ProcedureModel, ProcedureModelTrackable, StorageProcedureModel,
    Trackable, User,
    traits::{AppendProcedureModel, ChildOptions, ParentProcedureModel},
};
use diesel::OptionalExtension;
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::procedure_model_trackables::{
    coffee_wrapper::coffee_wrapper_builder, conical_tubes::cct_builder,
    conical_tubes_box::cct_box_builder,
};

/// Initializes the part of organism collection procedure model in the database.
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
pub(crate) fn init_part_of_organism_collection(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<ProcedureModel> {
    let name = "Part of Organism Collection";

    if let Some(existing) = ProcedureModel::from_name(name, conn).optional()? {
        return Ok(existing);
    }

    let sample = Trackable::from_name("Sample", conn)?;

    let sample_builder =
        ProcedureModelTrackable::new().name("Sample")?.trackable(sample.id)?.created_by(user.id)?;

    let collection = ProcedureModel::new()
        .name(name)?
        .description(
            "Procedure model to collect part of organisms, such as leaves, stems, or roots.",
        )?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    // Remind the user to wear gloves
    let gloves_reminder = ProcedureModel::new()
        .name("Wear gloves")?
        .description("Please wear gloves to avoid contamination and protect yourself.")?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    // Remind the user to sterilize / clean the scalpel and gloves with ethanol 70
    // percent
    let sterilization_reminder = ProcedureModel::new()
        .name("Sterilize scalpel and gloves")?
        .description(
            "Please sterilize the scalpel and gloves with ethanol 70 percent to avoid contamination.",
        )?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    // Cut the part of the organism to be collected with a sterile scalpel
    let cut_part = ProcedureModel::new()
        .name("Cut part of organism")?
        .description(
            "Use a sterile scalpel to cut the desired part of the organism, such as leaves, stems, or roots.",
        )?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    // Wrapping procedure with coffee filter paper
    let coffee_filter_wrapping = PackagingProcedureModel::new()
        .name("Wrap in coffee filter paper")?
        .description(
            "Wrap the cut part of the organism in a coffee filter paper to protect it during transport.",
        )?
        .created_by(user.id)?
        .procedure_packaged_with(coffee_wrapper_builder(user, conn)?)?
        .procedure_sample(sample_builder)?
        .insert(user.id, conn)?;

    // Placing the wrapped sample in the conical centrifugal tube
    let place_in_tube = StorageProcedureModel::new()
        .name("Place in conical centrifugal tube")?
        .description(
            "Place the wrapped sample in a conical centrifugal tube for storage and transport.",
        )?
        .procedure_parent_container(cct_builder(user, conn)?)?
        .procedure_child_container(coffee_wrapper_builder(user, conn)?)?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    // Put it in the storage box
    let place_in_storage_box = StorageProcedureModel::new()
        .name("Place in storage box")?
        .description(
            "Place the conical centrifugal tube with the sample in a storage box for long-term storage.",
        )?
        .procedure_parent_container(
            cct_box_builder(user, conn)?
        )?
        .procedure_child_container(cct_builder(user, conn)?)?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    for procedure in [
        &gloves_reminder,
        &sterilization_reminder,
        &cut_part,
        &coffee_filter_wrapping.procedure_model(conn)?,
        &place_in_tube.procedure_model(conn)?,
        &place_in_storage_box.procedure_model(conn)?,
    ] {
        collection.child(procedure, ChildOptions::default().inherit_trackables(), user, conn)?;
    }

    collection.extend(
        &[
            &gloves_reminder,
            &sterilization_reminder,
            &cut_part,
            &coffee_filter_wrapping.procedure_model(conn)?,
            &place_in_tube.procedure_model(conn)?,
            &place_in_storage_box.procedure_model(conn)?,
        ],
        user,
        conn,
    )?;

    Ok(collection)
}
