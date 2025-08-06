//! Submodule defining the Ethanol 70 percent procedure creation.

use core_structures::{
    ContainerModel, PackagingProcedureModel, ProcedureModel, ProcedureModelTrackable,
    StorageProcedureModel, Trackable, User, VolumetricContainerModel,
    traits::{AppendProcedureModel, ChildOptions, ParentProcedureModel},
};
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::trackables::containers::{
    CONICAL_CENTRIFUGAL_TUBE_50ML, POLYSTYRENE_BOX,
    wet_lab_containers::wrappers::COFFEE_FILTER_WRAPPER,
};

/// The name of the part of organism collection procedure model.
const PART_OF_ORGANISM: &str = "Part-of organisms collection procedure";
pub(crate) const CONICAL_TUBE_BOX: &str = "Conical Tube Box";
const SAMPLE_CCT: &str = "Sample Conical Centrifugal Tube";

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
) -> ProcedureModel {
    if let Some(existing) = ProcedureModel::from_name(PART_OF_ORGANISM, conn).unwrap() {
        return existing;
    }

    let cct =
        VolumetricContainerModel::from_name(CONICAL_CENTRIFUGAL_TUBE_50ML, conn).unwrap().unwrap();
    let cct_builder = ProcedureModelTrackable::new()
        .name(SAMPLE_CCT)
        .unwrap()
        .trackable(cct.id)
        .unwrap()
        .created_by(user.id)
        .unwrap();

    let coffee_filter_wrapper =
        ContainerModel::from_name(COFFEE_FILTER_WRAPPER, conn).unwrap().unwrap();

    let coffee_filter_wrapper_builder = ProcedureModelTrackable::new()
        .name("Coffee filter wrapper")
        .unwrap()
        .trackable(coffee_filter_wrapper.id)
        .unwrap()
        .created_by(user.id)
        .unwrap();

    let sample = Trackable::from_name("Sample", conn).unwrap().unwrap();

    let sample_builder = ProcedureModelTrackable::new()
        .name("Sample")
        .unwrap()
        .trackable(sample.id)
        .unwrap()
        .created_by(user.id)
        .unwrap();

    let collection = ProcedureModel::new()
        .name(PART_OF_ORGANISM)
        .unwrap()
        .description(
            "Procedure model to collect part of organisms, such as leaves, stems, or roots.",
        )
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    // Remind the user to wear gloves
    let gloves_reminder = ProcedureModel::new()
        .name("Wear gloves")
        .unwrap()
        .description("Please wear gloves to avoid contamination and protect yourself.")
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    // Remind the user to sterilize / clean the scalpel and gloves with ethanol 70
    // percent
    let sterilization_reminder = ProcedureModel::new()
        .name("Sterilize scalpel and gloves")
        .unwrap()
        .description(
            "Please sterilize the scalpel and gloves with ethanol 70 percent to avoid contamination.",
        )
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    // Cut the part of the organism to be collected with a sterile scalpel
    let cut_part = ProcedureModel::new()
        .name("Cut part of organism")
        .unwrap()
        .description(
            "Use a sterile scalpel to cut the desired part of the organism, such as leaves, stems, or roots.",
        )
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    // Wrapping procedure with coffee filter paper
    let coffee_filter_wrapping = PackagingProcedureModel::new()
        .name("Wrap in coffee filter paper")
        .unwrap()
        .description(
            "Wrap the cut part of the organism in a coffee filter paper to protect it during transport.",
        )
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .procedure_packaged_with(coffee_filter_wrapper_builder.clone())
        .unwrap()
        .procedure_sample(sample_builder)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    // Placing the wrapped sample in the conical centrifugal tube
    let place_in_tube = StorageProcedureModel::new()
        .name("Place in conical centrifugal tube")
        .unwrap()
        .description(
            "Place the wrapped sample in a conical centrifugal tube for storage and transport.",
        )
        .unwrap()
        .procedure_parent_container(cct_builder.clone())
        .unwrap()
        .procedure_child_container(coffee_filter_wrapper_builder)
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    // Put it in the storage box
    let place_in_storage_box = StorageProcedureModel::new()
        .name("Place in storage box")
        .unwrap()
        .description(
            "Place the conical centrifugal tube with the sample in a storage box for long-term storage.",
        )
        .unwrap()
        .procedure_parent_container(
            ProcedureModelTrackable::new()
            .name(CONICAL_TUBE_BOX)
            .unwrap()
            .trackable(ContainerModel::from_name(POLYSTYRENE_BOX, conn).unwrap().unwrap().id)
            .unwrap()
            .created_by(user.id)
            .unwrap()
        )
        .unwrap()
        .procedure_child_container(cct_builder)
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    for procedure in [
        &gloves_reminder,
        &sterilization_reminder,
        &cut_part,
        &coffee_filter_wrapping.procedure_model(conn).unwrap(),
        &place_in_tube.procedure_model(conn).unwrap(),
        &place_in_storage_box.procedure_model(conn).unwrap(),
    ] {
        collection
            .child(procedure, ChildOptions::default().inherit_trackables(), user, conn)
            .unwrap();
    }

    collection
        .extend(
            &[
                &gloves_reminder,
                &sterilization_reminder,
                &cut_part,
                &coffee_filter_wrapping.procedure_model(conn).unwrap(),
                &place_in_tube.procedure_model(conn).unwrap(),
                &place_in_storage_box.procedure_model(conn).unwrap(),
            ],
            user,
            conn,
        )
        .unwrap();

    collection
}
