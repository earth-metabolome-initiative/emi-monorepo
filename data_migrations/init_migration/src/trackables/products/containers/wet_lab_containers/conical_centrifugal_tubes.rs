//! Submodule for initializing the conical centrifugal tube and its rack
//! products.

use core_structures::{CommercialProduct, Trackable, User};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::{
    fisherbrand, greiner_bio_one,
    trackables::containers::{
        CONICAL_CENTRIFUGAL_TUBE_50ML, wet_lab_containers::CONICAL_CENTRIFUGAL_TUBE_50ML_RACK,
    },
};

/// Returns and possibly creates a Greiner Conical Centrifugal Tube 50ml
/// product.
pub(crate) fn init_greiner_cct(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let conical_tube = "Greiner Conical Centrifugal Tube 50ml";
    if let Some(conical_tube) = CommercialProduct::from_name(conical_tube, conn).optional()? {
        return Ok(conical_tube);
    }

    let conical_tube_50ml = Trackable::from_name(CONICAL_CENTRIFUGAL_TUBE_50ML, conn)?;
    let greiner = greiner_bio_one(user, conn)?;
    Ok(CommercialProduct::new()
        .name(Some(conical_tube.to_owned()))?
        .description(Some("Conical sample tube of 50ml, used for sample collection.".to_owned()))?
        .created_by(user.id)?
        .brand(greiner.id)?
        .parent(Some(conical_tube_50ml.id))?
        .insert(user.id, conn)?)
}

/// Returns and possibly creates a Fisherbrand Delrin Conical Centrifugal Tube
/// Rack 50ml product.
pub(crate) fn init_fisherbrand_cct_rack(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let conical_tube = "Fisherbrand Rack for Conical Centrifugal Tube 50ml";
    if let Some(conical_tube) = CommercialProduct::from_name(conical_tube, conn).optional()? {
        return Ok(conical_tube);
    }

    let conical_tube_rack_50ml = Trackable::from_name(CONICAL_CENTRIFUGAL_TUBE_50ML_RACK, conn)?;
    let fisherbrand = fisherbrand(user, conn)?;
    Ok(CommercialProduct::new()
        .name(Some(conical_tube.to_owned()))?
        .description(Some(
            "Rack for Conical sample tube of 50ml, used to store dried samples library.".to_owned(),
        ))?
        .created_by(user.id)?
        .brand(fisherbrand.id)?
        .parent(Some(conical_tube_rack_50ml.id))?
        .insert(user.id, conn)?)
}
