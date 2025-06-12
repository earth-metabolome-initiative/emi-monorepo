//! Submodule to initialize the `reagents` in the database.

use core_structures::{Trackable, User};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

pub mod wet_lab_containers;
pub(crate) use wet_lab_containers::{
    CONICAL_CENTRIFUGAL_TUBE_50ML, SAFELOCK_TUBE_2ML, VIAL_1_5ML, VIAL_1_5ML_CAP_SPLITTED,
    VIAL_1_5ML_SEALED_CAP, VIAL_INSERT_200UL,
};

pub const BOTTLE: &str = "Bottle";
pub const BOX: &str = "Box";
pub const SAMPLE_CONTAINER: &str = "Sample Container";
pub const SPRAYER: &str = "Sprayer";
pub const POLYSTYRENE_BOX: &str = "Polystyrene Box";

pub(crate) fn init_containers(user: &User, portal_conn: &mut PgConnection) {
    let container = core_structures::Trackable::new()
        .name(Some("Container".to_owned()))
        .unwrap()
        .description(Some("Containers used in laboratory procedures".to_owned()))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    let _bottle = Trackable::new()
        .name(Some(BOTTLE.to_owned()))
        .unwrap()
        .description(Some("Bottle, a common container for liquids".to_owned()))
        .unwrap()
        .parent_id(Some(container.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    let sample_container = Trackable::new()
        .name(Some(SAMPLE_CONTAINER.to_owned()))
        .unwrap()
        .description(Some("Sample container, a common container for samples".to_owned()))
        .unwrap()
        .parent_id(Some(container.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    let r#box = Trackable::new()
        .name(Some(BOX.to_owned()))
        .unwrap()
        .description(Some("Box, a common container for samples".to_owned()))
        .unwrap()
        .parent_id(Some(container.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    let _polystyrene_box = Trackable::new()
        .name(Some(POLYSTYRENE_BOX.to_owned()))
        .unwrap()
        .description(Some(
            "Polystyrene box, a container typically used for liquid nitrogen".to_owned(),
        ))
        .unwrap()
        .parent_id(Some(r#box.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    let _sprayer = Trackable::new()
        .name(Some(SPRAYER.to_owned()))
        .unwrap()
        .description(Some(
            "Sprayer, a container for liquids, usually Ethanol, used in laboratory or field procedures".to_owned(),
        ))
        .unwrap()
        .parent_id(Some(container.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    wet_lab_containers::init_wet_lab_containers(user, &container, &sample_container, portal_conn);
}
