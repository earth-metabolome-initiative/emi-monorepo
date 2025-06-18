//! Submodule to initialize the

use core_structures::{ContainerModel, StorageRule, User, VolumetricContainerModel};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::trackables::containers::CONICAL_CENTRIFUGAL_TUBE_50ML;

pub const COFFEE_FILTER_WRAPPER: &str = "Coffee Filter Wrapper";

pub(super) fn init_wrappers(
    user: &User,
    wet_lab_container: &ContainerModel,
    conn: &mut PgConnection,
) {
    let coffee_filter_wrapper = ContainerModel::new()
        .name(Some(COFFEE_FILTER_WRAPPER.to_owned()))
        .unwrap()
        .description(Some(
            "Coffee filters used to wrap sample in the field prior to storage in Falcon tubes"
                .to_owned(),
        ))
        .unwrap()
        .parent_id(Some(wet_lab_container.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let cct =
        VolumetricContainerModel::from_name(CONICAL_CENTRIFUGAL_TUBE_50ML, conn).unwrap().unwrap();

    StorageRule::new()
        .parent_container_id(cct.id)
        .unwrap()
        .child_container_id(coffee_filter_wrapper.id)
        .unwrap()
        .quantity(1i16)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();
}
