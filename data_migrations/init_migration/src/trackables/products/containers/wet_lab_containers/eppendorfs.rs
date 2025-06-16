//! Submodule for initializing the Eppendorf product.

use core_structures::{CommercialProduct, Trackable, User};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::{brands::eppendorf, trackables::containers::SAFELOCK_TUBE_2ML};

/// Returns and possibly creates an Eppendorf Safe-Lock Tube 2ml product.
pub(crate) fn init_eppendorf_safelock_tube(
    user: &User,
    conn: &mut PgConnection,
) -> CommercialProduct {
    let eppendorf_safelock_tube = "Eppendorf Safe-Lock Tube 2ml";
    if let Some(eppendorf_safelock_tube) =
        CommercialProduct::from_name(eppendorf_safelock_tube, conn).unwrap()
    {
        return eppendorf_safelock_tube;
    }

    let safelock_tube = Trackable::from_name(SAFELOCK_TUBE_2ML, conn)
        .unwrap()
        .expect("Sample container should exist");
    let eppendorf = eppendorf(user, conn).unwrap();
    CommercialProduct::new()
        .name(Some(eppendorf_safelock_tube.to_owned()))
        .unwrap()
        .description(Some("Eppendorf Safe-Lock Tube 2ml used for extraction.".to_owned()))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .brand_id(eppendorf.id)
        .unwrap()
        .parent_id(safelock_tube.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap()
}
