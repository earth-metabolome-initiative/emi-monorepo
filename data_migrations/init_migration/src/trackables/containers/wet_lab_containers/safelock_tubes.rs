//! Submodule to initialize the

use core_structures::{Trackable, User, VolumetricContainerModel};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

const SAFELOCK_TUBE: &str = "Safelock Tube";
pub const SAFELOCK_TUBE_2ML: &str = "Safelock Tube 2ml";

pub(super) fn init_safelock_tubes(
    user: &User,
    wet_lab_container: &Trackable,
    conn: &mut PgConnection,
) {
    let safelock_tube = Trackable::new()
        .name(Some(SAFELOCK_TUBE.to_owned()))
        .unwrap()
        .description(Some("Safelock Tube, used to perform extractions".to_owned()))
        .unwrap()
        .parent_id(Some(wet_lab_container.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let _safelock_tube_2ml = VolumetricContainerModel::new()
        .name(Some(SAFELOCK_TUBE_2ML.to_owned()))
        .unwrap()
        .description(Some("Safelock tube of 2ml, used for sample extraction.".to_owned()))
        .unwrap()
        .parent_id(Some(safelock_tube.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .liters(0.002)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();
}
