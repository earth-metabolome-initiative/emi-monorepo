//! Submodule to initialize the `instruments` in the database.

use core_structures::{Trackable, User};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

pub const FREEZER: &str = "Freezer";
pub const FREEZE_DRYER: &str = "Freeze Dryer";

pub(crate) fn init_instruments(user: &User, portal_conn: &mut PgConnection) {
    let instrument = core_structures::Trackable::new()
        .name(Some("Instrument".to_owned()))
        .unwrap()
        .description(Some("Instruments used in laboratory procedures".to_owned()))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    let _freezer = Trackable::new()
        .name(Some(FREEZER.to_owned()))
        .unwrap()
        .description(Some("-80°C Freezer".to_owned()))
        .unwrap()
        .parent_id(Some(instrument.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    let _freeze_dryer = Trackable::new()
        .name(Some(FREEZE_DRYER.to_owned()))
        .unwrap()
        .description(Some("Freeze dryer".to_owned()))
        .unwrap()
        .parent_id(Some(instrument.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();
}
