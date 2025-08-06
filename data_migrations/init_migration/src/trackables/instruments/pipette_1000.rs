//! Submodule creating the instrument commercial product model for the Pipette
//! 1000 instrument.

use core_structures::{CommercialProduct, Trackable, User};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::{
    brands::{gilson, sarstedt},
    trackables::instruments::{PIPETTE_TIPS_1000, PIPETTES_1000},
};

/// Returns and possibly initializes the Gilson pipette 1000 instrument
/// trackable in the database.
pub(crate) fn init_gilson_pipette_1000(user: &User, conn: &mut PgConnection) -> CommercialProduct {
    let device_name = "Gilson Pipette 1000";
    if let Some(instrument) = CommercialProduct::from_name(device_name, conn).unwrap() {
        return instrument;
    }

    let pipette =
        Trackable::from_name(PIPETTES_1000, conn).unwrap().expect("Pipette trackable should exist");
    let brand = gilson(user, conn).unwrap();

    CommercialProduct::new()
        .name(Some(device_name.to_owned()))
        .unwrap()
        .description(Some("Gilson Pipette 1000μl to manipulate liquids.".to_owned()))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .parent(Some(pipette.id))
        .unwrap()
        .brand(brand.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap()
}

/// Returns and possibly initializes the Sarstedt Tip for Gilson pipette 1000
/// tool trackable in the database.
pub(crate) fn init_sarstedt_pipette_tip_1000(
    user: &User,
    conn: &mut PgConnection,
) -> CommercialProduct {
    let device_name = "Sarstedt Tip for Gilson Pipette 1000μl";
    if let Some(instrument) = CommercialProduct::from_name(device_name, conn).unwrap() {
        return instrument;
    }

    let pipette_tip =
        Trackable::from_name(PIPETTE_TIPS_1000, conn).unwrap().expect("Tip trackable should exist");
    let brand = sarstedt(user, conn).unwrap();

    CommercialProduct::new()
        .name(Some(device_name.to_owned()))
        .unwrap()
        .description(Some("Sarstedt Tip for Gilson Pipette 1000μl.".to_owned()))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .parent(Some(pipette_tip.id))
        .unwrap()
        .brand(brand.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap()
}
