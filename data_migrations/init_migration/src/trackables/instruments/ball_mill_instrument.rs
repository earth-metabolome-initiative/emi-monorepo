//! Submodule creating the instrument commercial product model for the Ball Mill
//! instrument.

use core_structures::{CommercialProduct, Trackable, User};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::{brands::retsch, trackables::instruments::BALL_MILL_MACHINE};

/// Returns and possibly initializes the Retsch MM 400 Ball Mill instrument
/// trackable in the database.
pub(crate) fn init_retsch_mm400(user: &User, portal_conn: &mut PgConnection) -> CommercialProduct {
    let device_name = "Retsch MM 400 Ball Mill";
    if let Some(instrument) = CommercialProduct::from_name(device_name, portal_conn).unwrap() {
        return instrument;
    }

    let ball_mill = Trackable::from_name(BALL_MILL_MACHINE, portal_conn)
        .unwrap()
        .expect("Ball Mill Machine trackable should exist");
    let brand = retsch(user, portal_conn).unwrap();

    CommercialProduct::new()
        .name(Some(device_name.to_owned()))
        .unwrap()
        .description(Some(
            "Retsch MM 400 Ball Mill, used to reduce solid material into powder.".to_owned(),
        ))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .parent_id(Some(ball_mill.id))
        .unwrap()
        .brand_id(brand.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap()
}
