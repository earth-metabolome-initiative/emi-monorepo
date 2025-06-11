//! Submodule for initializing the

use core_structures::{CommercialProduct, Trackable, User};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::{
    brands::{advion_interchim, machinery_nagel, vici_schweiz},
    trackables::containers::{
        VIAL_1_5ML, VIAL_1_5ML_CAP_SPLITTED, VIAL_1_5ML_SEALED_CAP, VIAL_INSERT_200UL,
    },
};

/// Returns and possibly creates a Machinery Nagel Vial 1.5ml product.
pub(crate) fn init_machinery_nagel_vial(
    user: &User,
    portal_conn: &mut PgConnection,
) -> CommercialProduct {
    let vial = "Machinery Nagel Vial 1.5ml";
    if let Some(vial) = CommercialProduct::from_name(vial, portal_conn).unwrap() {
        return vial;
    }

    let vial_1_5ml =
        Trackable::from_name(VIAL_1_5ML, portal_conn).unwrap().expect("Vial should exist");
    let machinery_nagel = machinery_nagel(user, portal_conn).unwrap();
    CommercialProduct::new()
        .name(Some(vial.to_owned()))
        .unwrap()
        .description(Some(
            "Machinery Nagel Vial 1.5ml, used for extract library and mass spectrometry analysis."
                .to_owned(),
        ))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .brand_id(machinery_nagel.id)
        .unwrap()
        .parent_id(vial_1_5ml.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap()
}

/// Returns and possibly creates a Splitted cap
pub(crate) fn init_machinery_nagel_splitted_cap(
    user: &User,
    portal_conn: &mut PgConnection,
) -> CommercialProduct {
    let splitted_cap = "Machinery Nagel Splitted Cap 1.5ml";
    if let Some(splitted_cap) = CommercialProduct::from_name(splitted_cap, portal_conn).unwrap() {
        return splitted_cap;
    }

    let splitted_cap_trackable = Trackable::from_name(VIAL_1_5ML_CAP_SPLITTED, portal_conn)
        .unwrap()
        .expect("Cap should exist");
    let machinery_nagel = machinery_nagel(user, portal_conn).unwrap();
    CommercialProduct::new()
        .name(Some(splitted_cap.to_owned()))
        .unwrap()
        .description(Some("Machinery Nagel Splitted Cap 1.5ml, used to partially seal Vial 1.5ml and allows mass spectrometry analysis.".to_owned()))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .brand_id(machinery_nagel.id)
        .unwrap()
        .parent_id(splitted_cap_trackable.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap()
}

/// Returns and possibly creates a sealed cap
pub(crate) fn init_avion_interchim_sealed_cap(
    user: &User,
    portal_conn: &mut PgConnection,
) -> CommercialProduct {
    let sealed_cap = "Avion Interchim Sealed Cap 1.5ml";
    if let Some(sealed_cap) = CommercialProduct::from_name(sealed_cap, portal_conn).unwrap() {
        return sealed_cap;
    }

    let sealed_cap_trackable = Trackable::from_name(VIAL_1_5ML_SEALED_CAP, portal_conn)
        .unwrap()
        .expect("Cap should exist");
    let advion_interchim = advion_interchim(user, portal_conn).unwrap();
    CommercialProduct::new()
        .name(Some(sealed_cap.to_owned()))
        .unwrap()
        .description(Some(
            "Avion Interchim Sealed Cap 1.5ml, used to seal Vial 1.5ml for storage.".to_owned(),
        ))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .brand_id(advion_interchim.id)
        .unwrap()
        .parent_id(sealed_cap_trackable.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap()
}

/// Returns and possibly creates an insert
pub(crate) fn init_vici_schweiz_insert(
    user: &User,
    portal_conn: &mut PgConnection,
) -> CommercialProduct {
    let insert = "VICI Schweiz Insert 200μl";
    if let Some(insert) = CommercialProduct::from_name(insert, portal_conn).unwrap() {
        return insert;
    }

    let insert_trackable =
        Trackable::from_name(VIAL_INSERT_200UL, portal_conn).unwrap().expect("Insert should exist");
    let vici_schweiz = machinery_nagel(user, portal_conn).unwrap();
    CommercialProduct::new()
        .name(Some(insert.to_owned()))
        .unwrap()
        .description(Some("VICI Schweiz insert, used to decrease needed amount of extract for mass spectrometry analysis.".to_owned()))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .brand_id(vici_schweiz.id)
        .unwrap()
        .parent_id(insert_trackable.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap()
}
