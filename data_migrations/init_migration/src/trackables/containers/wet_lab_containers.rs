//! Submodule defining containers commonly used in wet-lab environments.

use core_structures::{Trackable, User};
use diesel::PgConnection;

pub mod conical_centrifugal_tubes;
pub(crate) use conical_centrifugal_tubes::{
    CONICAL_CENTRIFUGAL_TUBE_50ML, CONICAL_CENTRIFUGAL_TUBE_50ML_RACK,
};
pub mod safelock_tubes;
pub(crate) use safelock_tubes::SAFELOCK_TUBE_2ML;
pub mod vials;
pub(crate) use vials::{
    VIAL_1_5ML, VIAL_1_5ML_CAP_SPLITTED, VIAL_1_5ML_SEALED_CAP, VIAL_INSERT_200UL,
};

pub(super) fn init_wet_lab_containers(
    user: &User,
    container: &Trackable,
    wet_lab_container: &Trackable,
    portal_conn: &mut PgConnection,
) {
    conical_centrifugal_tubes::init_conical_centrifugal_tubes(
        user,
        container,
        wet_lab_container,
        portal_conn,
    );
    safelock_tubes::init_safelock_tubes(user, wet_lab_container, portal_conn);
    vials::init_vials(user, container, wet_lab_container, portal_conn);
}
