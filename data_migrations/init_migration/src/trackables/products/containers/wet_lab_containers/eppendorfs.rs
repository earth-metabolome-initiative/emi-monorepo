//! Submodule for initializing the Eppendorf product.

use core_structures::{CommercialProduct, Trackable, User};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::{brands::eppendorf, trackables::containers::SAFELOCK_TUBE_2ML};

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

    let safelock_tube = Trackable::from_name(SAFELOCK_TUBE_2ML, conn)?;
    let eppendorf = eppendorf(user, conn)?;
    Ok(CommercialProduct::new()
        .name(eppendorf_safelock_tube.to_owned())?
        .description("Eppendorf Safe-Lock Tube 2ml used for extraction.".to_owned())?
        .created_by(user.id)?
        .brand(eppendorf.id)?
        .parent(Some(safelock_tube.id))?
        .insert(user.id, conn)?)
}
