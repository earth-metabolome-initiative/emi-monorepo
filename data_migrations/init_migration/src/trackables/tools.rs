//! Submodule to initialize the `instruments` in the database.

use core_structures::{ContainerModel, Trackable, User};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

pub const METAL_BEADS: &str = "Metal Beads";
pub const METAL_BEADS_3MM: &str = "Metal Beads 3mm";

pub(crate) fn init_tools(user: &User, portal_conn: &mut PgConnection) {
    let metal_beads = core_structures::Trackable::new()
        .name(Some(METAL_BEADS.to_owned()))
        .unwrap()
        .description(Some("Metal beads used in laboratory procedures".to_owned()))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();

    let _metal_beads_3mm = Trackable::new()
        .name(Some(METAL_BEADS_3MM.to_owned()))
        .unwrap()
        .description(Some("Metal beads 3mm used in laboratory procedures".to_owned()))
        .unwrap()
        .parent_id(Some(metal_beads.id))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, portal_conn)
        .unwrap();
}
