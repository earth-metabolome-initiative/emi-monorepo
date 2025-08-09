//! Submodule to initialize the

use core_structures::{ContainerModel, User, VolumetricContainerModel};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

const SAFELOCK_TUBE: &str = "Safelock Tube";
pub const SAFELOCK_TUBE_2ML: &str = "Safelock Tube 2ml";

pub(super) fn init_safelock_tubes(
    user: &User,
    wet_lab_container: &ContainerModel,
    conn: &mut PgConnection,
) -> anyhow::Result<()> {
    let safelock_tube = ContainerModel::new()
        .name(SAFELOCK_TUBE.to_owned())?
        .description("Safelock Tube, used to perform extractions".to_owned())?
        .parent(Some(wet_lab_container.id))?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let _safelock_tube_2ml = VolumetricContainerModel::new()
        .name(SAFELOCK_TUBE_2ML.to_owned())?
        .description("Safelock tube of 2ml, used for sample extraction.".to_owned())?
        .parent(Some(safelock_tube.id))?
        .created_by(user.id)?
        .liters(0.002)?
        .insert(user.id, conn)?;

    Ok(())
}
