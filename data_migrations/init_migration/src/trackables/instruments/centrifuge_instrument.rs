//! Submodule creating the instrument commercial product model for the
//! Centrifuge instrument.

use core_structures::{CommercialProduct, Trackable, User};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::{brands::eppendorf, trackables::instruments::SAFELOCK_CENTRIFUGE};

/// Returns and possibly initializes the Eppendorf centrifuge instrument
/// trackable in the database.
pub(crate) fn init_eppendorf_centrifuge(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let device_name = "Eppendorf centrifuge";
    if let Some(instrument) = CommercialProduct::from_name(device_name, conn).optional()? {
        return Ok(instrument);
    }

    let centrifuge = Trackable::from_name(SAFELOCK_CENTRIFUGE, conn)?;
    let brand = eppendorf(user, conn)?;

    Ok(CommercialProduct::new()
        .name(Some(device_name.to_owned()))?
        .description(Some("Eppendorf centrifuge, used to precipitate solid material.".to_owned()))?
        .created_by(user.id)?
        .parent(Some(centrifuge.id))?
        .brand(brand.id)?
        .insert(user.id, conn)?)
}
