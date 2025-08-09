//! Submodule creating the instrument commercial product model for the Ball Mill
//! instrument.

use core_structures::{CommercialProduct, Trackable, User};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::{brands::retsch, trackables::instruments::BALL_MILL_MACHINE};

/// Returns and possibly initializes the Retsch MM 400 Ball Mill instrument
/// trackable in the database.
pub(crate) fn init_retsch_mm400(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let device_name = "Retsch MM 400 Ball Mill";
    if let Some(instrument) = CommercialProduct::from_name(device_name, conn).optional()? {
        return Ok(instrument);
    }

    let ball_mill = Trackable::from_name(BALL_MILL_MACHINE, conn)?;
    let brand = retsch(user, conn)?;

    Ok(CommercialProduct::new()
        .name(Some(device_name.to_owned()))?
        .description(Some(
            "Retsch MM 400 Ball Mill, used to reduce solid material into powder.".to_owned(),
        ))?
        .created_by(user.id)?
        .parent(Some(ball_mill.id))?
        .brand(brand.id)?
        .insert(user.id, conn)?)
}
