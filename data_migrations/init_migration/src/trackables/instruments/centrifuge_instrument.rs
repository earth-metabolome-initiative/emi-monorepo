//! Submodule creating the instrument commercial product model for the
//! Centrifuge instrument.

use core_structures::{CommercialProduct, Trackable, User};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::{brands::eppendorf, trackables::instruments::SAFELOCK_CENTRIFUGE};

/// Returns and possibly initializes the Eppendorf centrifuge instrument
/// trackable in the database.
pub(crate) fn init_eppendorf_centrifuge(user: &User, conn: &mut PgConnection) -> CommercialProduct {
    let device_name = "Eppendorf centrifuge";
    if let Some(instrument) = CommercialProduct::from_name(device_name, conn).unwrap() {
        return instrument;
    }

    let centrifuge = Trackable::from_name(SAFELOCK_CENTRIFUGE, conn)
        .unwrap()
        .expect("Centrifuge trackable should exist");
    let brand = eppendorf(user, conn).unwrap();

    CommercialProduct::new()
        .name(Some(device_name.to_owned()))
        .unwrap()
        .description(Some("Eppendorf centrifuge, used to precipitate solid material.".to_owned()))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .parent_id(Some(centrifuge.id))
        .unwrap()
        .brand_id(brand.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap()
}
