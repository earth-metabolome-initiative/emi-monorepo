//! Submodule for initializing the

use core_structures::{CommercialProduct, Trackable, User};
use diesel::{OptionalExtension, PgConnection};
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::{
    brands::{advion_interchim, fisherbrand, macherey_nagel, vici_schweiz},
    trackables::containers::{
        VIAL_1_5ML, VIAL_1_5ML_CAP_SPLITTED, VIAL_1_5ML_SEALED_CAP, VIAL_INSERT_200UL,
        wet_lab_containers::vials::VIAL_BOX,
    },
};

/// Returns and possibly creates a Machinery Nagel Vial 1.5ml product.
pub(crate) fn init_macherey_nagel_vial(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let vial = "Macherey Nagel Vial 1.5ml";
    if let Some(vial) = CommercialProduct::from_name(vial, conn).optional()? {
        return Ok(vial);
    }

    let vial_1_5ml = Trackable::from_name(VIAL_1_5ML, conn)?;
    let macherey_nagel = macherey_nagel(user, conn)?;
    Ok(CommercialProduct::new()
        .name(Some(vial.to_owned()))?
        .description(Some(
            "Macherey Nagel Vial 1.5ml, used for extract library and mass spectrometry analysis."
                .to_owned(),
        ))?
        .created_by(user.id)?
        .brand(macherey_nagel.id)?
        .parent(Some(vial_1_5ml.id))?
        .insert(user.id, conn)?)
}

/// Returns and possibly creates a Fisherbrand Kryobox Vial Box product.
///
/// # Arguments
///
/// * `user` - The user for whom the product is being created.
/// * `conn` - The database connection.
pub(crate) fn init_fisherbrand_kryobox_vial_box(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let vial_box = "Fisherbrand Kryobox Vial Box";
    if let Some(vial_box) = CommercialProduct::from_name(vial_box, conn).optional()? {
        return Ok(vial_box);
    }

    let vial_box_trackable = Trackable::from_name(VIAL_BOX, conn)?;
    let fisherbrand = fisherbrand(user, conn)?;
    Ok(CommercialProduct::new()
        .name(Some(vial_box.to_owned()))?
        .description(Some("Fisherbrand Kryobox Vial Box, used to store vials.".to_owned()))?
        .created_by(user.id)?
        .brand(fisherbrand.id)?
        .parent(Some(vial_box_trackable.id))?
        .insert(user.id, conn)?)
}

/// Returns and possibly creates a Splitted cap
pub(crate) fn init_macherey_nagel_splitted_cap(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let splitted_cap = "Machinery Nagel Splitted Cap 1.5ml";
    if let Some(splitted_cap) = CommercialProduct::from_name(splitted_cap, conn).optional()? {
        return Ok(splitted_cap);
    }

    let splitted_cap_trackable = Trackable::from_name(VIAL_1_5ML_CAP_SPLITTED, conn)?;
    let macherey_nagel = macherey_nagel(user, conn)?;
    Ok(CommercialProduct::new()
        .name(Some(splitted_cap.to_owned()))
        ?
        .description(Some("Machinery Nagel Splitted Cap 1.5ml, used to partially seal Vial 1.5ml and allows mass spectrometry analysis.".to_owned()))
        ?
        .created_by(user.id)
        ?
        .brand(macherey_nagel.id)
        ?
        .parent(Some(splitted_cap_trackable.id))
        ?
        .insert(user.id, conn)
        ?)
}

/// Returns and possibly creates a sealed cap
pub(crate) fn init_advion_interchim_sealed_cap(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let sealed_cap = "Avion Interchim Sealed Cap 1.5ml";
    if let Some(sealed_cap) = CommercialProduct::from_name(sealed_cap, conn).optional()? {
        return Ok(sealed_cap);
    }

    let sealed_cap_trackable = Trackable::from_name(VIAL_1_5ML_SEALED_CAP, conn)?;
    let advion_interchim = advion_interchim(user, conn)?;
    Ok(CommercialProduct::new()
        .name(Some(sealed_cap.to_owned()))?
        .description(Some(
            "Avion Interchim Sealed Cap 1.5ml, used to seal Vial 1.5ml for storage.".to_owned(),
        ))?
        .created_by(user.id)?
        .brand(advion_interchim.id)?
        .parent(Some(sealed_cap_trackable.id))?
        .insert(user.id, conn)?)
}

/// Returns and possibly creates an insert
pub(crate) fn init_vici_schweiz_insert(
    user: &User,
    conn: &mut PgConnection,
) -> anyhow::Result<CommercialProduct> {
    let insert = "VICI Schweiz Insert 200μl";
    if let Some(insert) = CommercialProduct::from_name(insert, conn).optional()? {
        return Ok(insert);
    }

    let insert_trackable = Trackable::from_name(VIAL_INSERT_200UL, conn)?;
    let vici_schweiz = vici_schweiz(user, conn)?;
    Ok(CommercialProduct::new()
        .name(Some(insert.to_owned()))
        ?
        .description(Some("VICI Schweiz insert, used to decrease needed amount of extract for mass spectrometry analysis.".to_owned()))
        ?
        .created_by(user.id)
        ?
        .brand(vici_schweiz.id)
        ?
        .parent(Some(insert_trackable.id))
        ?
        .insert(user.id, conn)
        ?)
}
