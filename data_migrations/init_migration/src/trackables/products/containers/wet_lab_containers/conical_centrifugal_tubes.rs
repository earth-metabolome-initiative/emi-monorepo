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
pub(crate) fn init_greiner_cct(user: &User, conn: &mut PgConnection) -> CommercialProduct {
    let conical_tube = "Greiner Conical Centrifugal Tube 50ml";
    if let Some(conical_tube) = CommercialProduct::from_name(conical_tube, conn).unwrap() {
        return conical_tube;
    }

    let conical_tube_50ml = Trackable::from_name(CONICAL_CENTRIFUGAL_TUBE_50ML, conn)
        .unwrap()
        .expect("Sample container should exist");
    let greiner = greiner_bio_one(user, conn).unwrap();
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
        .insert(user.id, conn)
        .unwrap()
}

/// Returns and possibly creates a Fisherbrand Delrin Conical Centrifugal Tube
/// Rack 50ml product.
pub(crate) fn init_fisherbrand_cct_rack(user: &User, conn: &mut PgConnection) -> CommercialProduct {
    let conical_tube = "Fisherbrand Rack for Conical Centrifugal Tube 50ml";
    if let Some(conical_tube) = CommercialProduct::from_name(conical_tube, conn).unwrap() {
        return conical_tube;
    }

    let conical_tube_rack_50ml = Trackable::from_name(CONICAL_CENTRIFUGAL_TUBE_50ML_RACK, conn)
        .unwrap()
        .expect("Rack should exist");
    let fisherbrand = fisherbrand(user, conn).unwrap();
    let rack = CommercialProduct::new()
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
        .insert(user.id, conn)
        .unwrap();

    rack
}
