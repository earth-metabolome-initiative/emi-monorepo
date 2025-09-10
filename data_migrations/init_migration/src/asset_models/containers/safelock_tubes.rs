//! Submodule to initialize the

use core_structures::{
    CommercialProduct, User, VolumetricContainerModel,
    tables::insertables::{
        AssetModelSettable, CommercialProductSettable, VolumetricContainerModelSettable,
    },
};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

pub(crate) fn safelock_tubes_2ml(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<VolumetricContainerModel> {
    let name = "Safelock Tube 2ml";

    if let Some(existing_tube) = VolumetricContainerModel::from_name(name, conn).optional()? {
        return Ok(existing_tube);
    }

    Ok(VolumetricContainerModel::new()
        .name(name)?
        .description("Safelock tube of 2ml, used for sample extraction.")?
        .created_by(user)?
        .liters(0.002)?
        .insert(user.id, conn)?)
}

use crate::brands::eppendorf;

/// Returns and possibly creates an Eppendorf Safe-Lock Tube 2ml product.
pub(crate) fn init_eppendorf_safelock_tube(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let eppendorf_safelock_tube = "Eppendorf Safe-Lock Tube 2ml";
    if let Some(eppendorf_safelock_tube) =
        CommercialProduct::from_name(eppendorf_safelock_tube, conn).optional()?
    {
        return Ok(eppendorf_safelock_tube);
    }

    let safelock_tube = safelock_tubes_2ml(user, conn)?;
    let eppendorf = eppendorf(user, conn)?;
    Ok(CommercialProduct::new()
        .name(eppendorf_safelock_tube)?
        .description("Eppendorf Safe-Lock Tube 2ml used for extraction.")?
        .created_by(user)?
        .brand(eppendorf)?
        .parent_model(safelock_tube)?
        .insert(user.id, conn)?)
}
