//! Submodule creating the instrument commercial product model for the Pipette
//! 1000 instrument.

use core_structures::{CommercialProduct, PipetteModel, PipetteTipModel, User};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::{
    brands::{gilson, sarstedt},
    trackables::instruments::{PIPETTE_TIPS_1000, PIPETTES_1000},
};

/// Returns and possibly initializes the Gilson pipette 1000 instrument
/// trackable in the database.
pub(crate) fn init_gilson_pipette_1000(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let device_name = "Gilson Pipette 1000";
    if let Some(instrument) = CommercialProduct::from_name(device_name, conn).optional()? {
        return Ok(instrument);
    }

    let pipette = PipetteModel::from_name(PIPETTES_1000, conn)?;
    let brand = gilson(user, conn)?;

    Ok(CommercialProduct::new()
        .name(device_name.to_owned())?
        .description("Gilson Pipette 1000μl to manipulate liquids.".to_owned())?
        .created_by(user.id)?
        .parent(Some(pipette.id))?
        .brand(brand.id)?
        .insert(user.id, conn)?)
}

/// Returns and possibly initializes the Sarstedt Tip for Gilson pipette 1000
/// tool trackable in the database.
pub(crate) fn init_sarstedt_pipette_tip_1000(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let device_name = "Sarstedt Tip for Gilson Pipette 1000μl";
    if let Some(instrument) = CommercialProduct::from_name(device_name, conn).optional()? {
        return Ok(instrument);
    }

    let pipette_tip = PipetteTipModel::from_name(PIPETTE_TIPS_1000, conn)?;
    let brand = sarstedt(user, conn)?;

    Ok(CommercialProduct::new()
        .name(device_name.to_owned())?
        .description("Sarstedt Tip for Gilson Pipette 1000μl.".to_owned())?
        .created_by(user.id)?
        .parent(Some(pipette_tip.id))?
        .brand(brand.id)?
        .insert(user.id, conn)?)
}
