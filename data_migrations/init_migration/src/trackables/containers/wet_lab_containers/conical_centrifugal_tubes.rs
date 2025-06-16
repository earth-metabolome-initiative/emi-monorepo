//! Submodule to initialize the sample containers in the database.

use core_structures::{ContainerModel, StorageRule, Trackable, User, VolumetricContainerModel};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

const CONICAL_CENTRIFUGAL_TUBE: &str = "Conical Tube";
pub const CONICAL_CENTRIFUGAL_TUBE_50ML: &str = "Conical Tube 50ml";
pub const CONICAL_CENTRIFUGAL_TUBE_50ML_RACK: &str = "Conical Tube Rack 50ml";

pub(super) fn init_conical_centrifugal_tubes(
    user: &User,
    container: &Trackable,
    wet_lab_container: &Trackable,
    conn: &mut PgConnection,
) {
    let conical_tube = Trackable::new()
        .name(Some(CONICAL_CENTRIFUGAL_TUBE.to_owned()))
        .unwrap()
        .description(Some("Conical tube, a common container for samples".to_owned()))
        .unwrap()
        .parent_id(Some(wet_lab_container.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let conical_tube_50ml = VolumetricContainerModel::new()
        .name(Some(CONICAL_CENTRIFUGAL_TUBE_50ML.to_owned()))
        .unwrap()
        .description(Some("Conical tube of 50ml, used for sample collection.".to_owned()))
        .unwrap()
        .parent_id(Some(conical_tube.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .liters(0.05)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let conical_tube_rack = ContainerModel::new()
        .name(Some(CONICAL_CENTRIFUGAL_TUBE_50ML_RACK.to_owned()))
        .unwrap()
        .description(Some("Conical tube rack, a common container for conical tubes".to_owned()))
        .unwrap()
        .parent_id(Some(container.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    StorageRule::new()
        .parent_container_id(conical_tube_rack.id)
        .unwrap()
        .child_container_id(conical_tube_50ml.id)
        .unwrap()
        // TODO! ACTUALLY SET THE CORRECT QUANTITY
        .quantity(24i16)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();
}
