//! Submodule creating the instrument commercial product model for the Pipette
//! 200 instrument.

use core_structures::{CommercialProduct, Trackable, User};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::{
    brands::{gilson, sarstedt},
    trackables::instruments::{PIPETTE_TIPS_200, PIPETTES_200},
};

/// Returns and possibly initializes the Gilson pipettes 200
/// trackable in the database.
pub(crate) fn init_gilson_pipette_200(user: &User, conn: &mut PgConnection) -> CommercialProduct {
    let device_name = "Gilson pipette 200";
    if let Some(instrument) = CommercialProduct::from_name(device_name, conn).unwrap() {
        return instrument;
    }

    let pipette = Trackable::from_name(PIPETTES_200, conn)
        .unwrap()
        .expect("Centrifuge trackable should exist");
    let brand = gilson(user, conn).unwrap();

    CommercialProduct::new()
        .name(Some(device_name.to_owned()))
        .unwrap()
        .description(Some("Gilson pipette 200, used to precipitate solid material.".to_owned()))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .parent_id(Some(pipette.id))
        .unwrap()
        .brand_id(brand.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap()
}

/// Returns and possibly initializes the Gilson pipette 200 tip tool
/// trackable in the database.
pub(crate) fn init_sarstedt_pipette_tip_200(
    user: &User,
    conn: &mut PgConnection,
) -> CommercialProduct {
    let device_name = "Sarstedt Tip for Gilson Pipette 200μl";
    if let Some(instrument) = CommercialProduct::from_name(device_name, conn).unwrap() {
        return instrument;
    }

    let pipette_tip = Trackable::from_name(PIPETTE_TIPS_200, conn)
        .unwrap()
        .expect("Pipette tips trackable should exist");
    let brand = sarstedt(user, conn).unwrap();

    CommercialProduct::new()
        .name(Some(device_name.to_owned()))
        .unwrap()
        .description(Some("Tip for Gilson Pipette 200µl to manipulate liquids".to_owned()))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .parent_id(Some(pipette_tip.id))
        .unwrap()
        .brand_id(brand.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap()
}
