//! Submodule creating the instrument commercial product model for the Pipette
//! 200 instrument.

use core_structures::{CommercialProduct, PipetteModel, PipetteTipModel, Trackable, User};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::{
    brands::{gilson, sarstedt},
    trackables::instruments::{PIPETTE_TIPS_200, PIPETTES_200},
};

/// Returns and possibly initializes the Gilson pipettes 200
/// trackable in the database.
pub(crate) fn init_gilson_pipette_200(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let device_name = "Gilson pipette 200";
    if let Some(instrument) = CommercialProduct::from_name(device_name, conn).optional()? {
        return Ok(instrument);
    }

    let pipette = PipetteModel::from_name(PIPETTES_200, conn)?;
    let brand = gilson(user, conn)?;

    Ok(CommercialProduct::new()
        .name(device_name.to_owned())?
        .description("Gilson pipette 200, used to precipitate solid material.".to_owned())?
        .created_by(user.id)?
        .parent(Some(pipette.id))?
        .brand(brand.id)?
        .insert(user.id, conn)?)
}

/// Returns and possibly initializes the Gilson pipette 200 tip tool
/// trackable in the database.
pub(crate) fn init_sarstedt_pipette_tip_200(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let device_name = "Sarstedt Tip for Gilson Pipette 200μl";
    if let Some(instrument) = CommercialProduct::from_name(device_name, conn).optional()? {
        return Ok(instrument);
    }

    let pipette_tip = PipetteTipModel::from_name(PIPETTE_TIPS_200, conn)?;
    let brand = sarstedt(user, conn)?;

    Ok(CommercialProduct::new()
        .name(device_name.to_owned())?
        .description("Tip for Gilson Pipette 200µl to manipulate liquids".to_owned())?
        .created_by(user.id)?
        .parent(Some(pipette_tip.id))?
        .brand(brand.id)?
        .insert(user.id, conn)?)
}
