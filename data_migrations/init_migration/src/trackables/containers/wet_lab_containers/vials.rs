//! Submodule to initialize the

use core_structures::{
    ContainerModel, Trackable, User, VolumetricContainerModel, traits::CompatibleWith,
};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

pub const VIAL_1_5ML: &str = "Vial 1.5ml";
pub const VIAL_1_5ML_SEALED_CAP: &str = "Vial 1.5ml Cap Sealed";
pub const VIAL_1_5ML_CAP_SPLITTED: &str = "Vial 1.5ml Cap Splitted";
pub const VIAL_INSERT: &str = "Vial Insert";
pub const VIAL_INSERT_200UL: &str = "Vial Insert 200μl";
pub const VIAL_BOX: &str = "Vial Box";

pub(super) fn init_vials(
    user: &User,
    container: &ContainerModel,
    wet_lab_container: &ContainerModel,
    conn: &mut PgConnection,
) {
    let vial = ContainerModel::new()
        .name(Some("Vial".to_owned()))
        .unwrap()
        .description(Some("Vials, used to store extracts".to_owned()))
        .unwrap()
        .parent_id(Some(wet_lab_container.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let vial_1_5ml = VolumetricContainerModel::new()
        .name(Some(VIAL_1_5ML.to_owned()))
        .unwrap()
        .description(Some(
            "Vials of 1.5ml, used for extract library and mass spectrometry analysis.".to_owned(),
        ))
        .unwrap()
        .parent_id(Some(vial.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .liters(0.0015)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let vial_cap = Trackable::new()
        .name(Some("Vial Cap".to_owned()))
        .unwrap()
        .description(Some("Vial cap, used to seal vials".to_owned()))
        .unwrap()
        .parent_id(Some(wet_lab_container.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let splitted_vial_cap = Trackable::new()
        .name(Some("Splitted Vial Cap".to_owned()))
        .unwrap()
        .description(Some("Splitted vial cap, used to partially seal vials".to_owned()))
        .unwrap()
        .parent_id(Some(vial_cap.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let _vial_1_5ml_splitted_cap = VolumetricContainerModel::new()
        .name(Some(VIAL_1_5ML_CAP_SPLITTED.to_owned()))
        .unwrap()
        .description(Some("Splitted vial cap of 1.5ml, used to partially seal vials.".to_owned()))
        .unwrap()
        .parent_id(Some(splitted_vial_cap.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .liters(0.0015)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let sealed_vial_cap = Trackable::new()
        .name(Some("Sealed Vial Cap".to_owned()))
        .unwrap()
        .description(Some("Sealed vial cap, used to seal vials".to_owned()))
        .unwrap()
        .parent_id(Some(vial_cap.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let vial_1_5ml_sealed_cap = VolumetricContainerModel::new()
        .name(Some(VIAL_1_5ML_SEALED_CAP.to_owned()))
        .unwrap()
        .description(Some("Sealed vial cap of 1.5ml, used to seal vials.".to_owned()))
        .unwrap()
        .parent_id(Some(sealed_vial_cap.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .liters(0.0015)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    // We register that the cap can be used with the vial
    vial_1_5ml.compatible_with_quantity(&vial_1_5ml_sealed_cap, 1, user, conn).unwrap();

    let vial_insert = VolumetricContainerModel::new()
        .name(Some(VIAL_INSERT.to_owned()))
        .unwrap()
        .description(Some("Vial insert, used to hold samples in vials".to_owned()))
        .unwrap()
        .parent_id(Some(vial.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        // TODO!: Set the correct volume
        .liters(0.0002)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let _vial_insert_200ul = VolumetricContainerModel::new()
        .name(Some(VIAL_INSERT_200UL.to_owned()))
        .unwrap()
        .description(Some("Vial insert of 200μl, used to hold samples in vials.".to_owned()))
        .unwrap()
        .parent_id(Some(vial_insert.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .liters(0.0002)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let vial_box = ContainerModel::new()
        .name(Some(VIAL_BOX.to_owned()))
        .unwrap()
        .description(Some("Vial box, used to store vials".to_owned()))
        .unwrap()
        .parent_id(Some(container.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    vial_box.compatible_with_quantity(&vial_1_5ml, 24, user, conn).unwrap();
}
