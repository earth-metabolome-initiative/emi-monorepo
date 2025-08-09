//! Submodule to initialize the `reagents` in the database.

use core_structures::{CompatibilityRule, ContainerModel, FreezerModel, User};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

pub mod wet_lab_containers;
pub(crate) use wet_lab_containers::{
    CONICAL_CENTRIFUGAL_TUBE_50ML, SAFELOCK_TUBE_2ML, VIAL_1_5ML, VIAL_1_5ML_CAP_SPLITTED,
    VIAL_1_5ML_SEALED_CAP, VIAL_INSERT_200UL,
};

use crate::trackables::instruments::FREEZER;

pub const SHELF: &str = "Shelf";
pub const BOTTLE: &str = "Bottle";
pub const BOX: &str = "Box";
pub const SAMPLE_CONTAINER: &str = "Sample Container";
pub const SPRAYER: &str = "Sprayer";
pub const POLYSTYRENE_BOX: &str = "Polystyrene Box";

pub(crate) fn init_containers(user: &User, conn: &mut PgConnection) -> anyhow::Result<()> {
    let container = ContainerModel::new()
        .name(Some("Container".to_owned()))?
        .description(Some("Containers used in laboratory procedures".to_owned()))?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let _bottle = ContainerModel::new()
        .name(Some(BOTTLE.to_owned()))?
        .description(Some("Bottle, a common container for liquids".to_owned()))?
        .parent(Some(container.id))?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let sample_container = ContainerModel::new()
        .name(Some(SAMPLE_CONTAINER.to_owned()))?
        .description(Some("Sample container, a common container for samples".to_owned()))?
        .parent(Some(container.id))?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let r#box = ContainerModel::new()
        .name(Some(BOX.to_owned()))?
        .description(Some("Box, a common container for samples".to_owned()))?
        .parent(Some(container.id))?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let polystyrene_box = ContainerModel::new()
        .name(Some(POLYSTYRENE_BOX.to_owned()))?
        .description(Some(
            "Polystyrene box, a container typically used for liquid nitrogen".to_owned(),
        ))?
        .parent(Some(r#box.id))?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let _shelf = ContainerModel::new()
        .name(Some(SHELF.to_owned()))?
        .description(Some("Shelf, a common container for storing other containers".to_owned()))?
        .parent(Some(container.id))?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let _sprayer = ContainerModel::new()
        .name(Some(SPRAYER.to_owned()))
        ?
        .description(Some(
            "Sprayer, a container for liquids, usually Ethanol, used in laboratory or field procedures".to_owned(),
        ))
        ?
        .parent(Some(container.id))
        ?
        .created_by(user.id)
        ?
        .insert(user.id, conn)
        ?;

    wet_lab_containers::init_wet_lab_containers(user, &container, &sample_container, conn)?;

    Ok(())
}
