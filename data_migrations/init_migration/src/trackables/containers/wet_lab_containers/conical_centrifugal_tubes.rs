//! Submodule to initialize the sample containers in the database.

use core_structures::{ContainerModel, User, VolumetricContainerModel, traits::CompatibleWith};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::trackables::containers::{POLYSTYRENE_BOX, SHELF};

const CONICAL_CENTRIFUGAL_TUBE: &str = "Conical Tube";
pub const CONICAL_CENTRIFUGAL_TUBE_50ML: &str = "Conical Tube 50ml";
pub const CONICAL_CENTRIFUGAL_TUBE_50ML_RACK: &str = "Conical Tube Rack 50ml";

pub(super) fn init_conical_centrifugal_tubes(
    user: &User,
    container: &ContainerModel,
    wet_lab_container: &ContainerModel,
    conn: &mut PgConnection,
) -> anyhow::Result<()> {
    let conical_tube = ContainerModel::new()
        .name(CONICAL_CENTRIFUGAL_TUBE.to_owned())?
        .description("Conical tube, a common container for samples".to_owned())?
        .parent(Some(wet_lab_container.id))?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let conical_tube_50ml = VolumetricContainerModel::new()
        .name(CONICAL_CENTRIFUGAL_TUBE_50ML.to_owned())?
        .description("Conical tube of 50ml, used for sample collection.".to_owned())?
        .parent(Some(conical_tube.id))?
        .created_by(user.id)?
        .liters(0.05)?
        .insert(user.id, conn)?;

    let conical_tube_rack = ContainerModel::new()
        .name(CONICAL_CENTRIFUGAL_TUBE_50ML_RACK.to_owned())?
        .description("Conical tube rack, a common container for conical tubes".to_owned())?
        .parent(Some(container.id))?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    conical_tube_rack.compatible_with_quantity(&conical_tube_50ml, 24, user, conn)?;

    ContainerModel::from_name(POLYSTYRENE_BOX, conn)?.compatible_with(
        &conical_tube_50ml,
        user,
        conn,
    )?;

    ContainerModel::from_name(SHELF, conn)?.compatible_with(&conical_tube_rack, user, conn)?;

    Ok(())
}
