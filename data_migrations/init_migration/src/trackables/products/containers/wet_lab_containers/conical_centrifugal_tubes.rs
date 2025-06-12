//! Submodule for initializing the conical centrifugal tube and its rack
//! products.

use core_structures::{CommercialProduct, Trackable, User};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::{
    fisherbrand, greiner_bio_one,
    trackables::containers::{
        CONICAL_CENTRIFUGAL_TUBE_50ML, wet_lab_containers::CONICAL_CENTRIFUGAL_TUBE_50ML_RACK,
    },
};

/// Returns and possibly creates a Greiner Conical Centrifugal Tube 50ml
/// product.
pub(crate) fn init_greiner_cct(user: &User, portal_conn: &mut PgConnection) -> CommercialProduct {
    let conical_tube = "Greiner Conical Centrifugal Tube 50ml";
    if let Some(conical_tube) = CommercialProduct::from_name(conical_tube, portal_conn).unwrap() {
        return conical_tube;
    }

    let conical_tube_50ml = Trackable::from_name(CONICAL_CENTRIFUGAL_TUBE_50ML, portal_conn)
        .unwrap()
        .expect("Sample container should exist");
    let greiner = greiner_bio_one(user, portal_conn).unwrap();
    CommercialProduct::new()
        .name(Some(conical_tube.to_owned()))
        .unwrap()
        .description(Some("Conical sample tube of 50ml, used for sample collection.".to_owned()))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .brand_id(greiner.id)
        .unwrap()
        .parent_id(conical_tube_50ml.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap()
}

/// Returns and possibly creates a Fisherbrand Delrin Conical Centrifugal Tube
/// Rack 50ml product.
pub(crate) fn init_fisherbrand_cct_rack(
    user: &User,
    portal_conn: &mut PgConnection,
) -> CommercialProduct {
    let conical_tube = "Fisherbrand Rack for Conical Centrifugal Tube 50ml";
    if let Some(conical_tube) = CommercialProduct::from_name(conical_tube, portal_conn).unwrap() {
        return conical_tube;
    }

    let conical_tube_rack_50ml =
        Trackable::from_name(CONICAL_CENTRIFUGAL_TUBE_50ML_RACK, portal_conn)
            .unwrap()
            .expect("Rack should exist");
    let fisherbrand = fisherbrand(user, portal_conn).unwrap();
    CommercialProduct::new()
        .name(Some(conical_tube.to_owned()))
        .unwrap()
        .description(Some(
            "Rack for Conical sample tube of 50ml, used to store dried samples library.".to_owned(),
        ))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .brand_id(fisherbrand.id)
        .unwrap()
        .parent_id(conical_tube_rack_50ml.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap()
}
