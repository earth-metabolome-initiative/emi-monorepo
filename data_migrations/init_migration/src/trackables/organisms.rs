//! Submodule to initialize the `instruments` in the database.

use core_structures::{Trackable, User};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

pub(crate) const ORGANISM: &str = "Organism";
pub(crate) const SAMPLE: &str = "Sample";

pub(crate) fn init_organisms(user: &User, conn: &mut PgConnection) {
    let _organisms = Trackable::new()
        .name(Some(ORGANISM.to_owned()))
        .unwrap()
        .description(Some("Organisms used in laboratory procedures".to_owned()))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();

    let _sample = Trackable::new()
        .name(Some(SAMPLE.to_owned()))
        .unwrap()
        .description(Some("Sample of an organism".to_owned()))
        .unwrap()
        .created_by(user.id)
        .unwrap()
        .insert(user.id, conn)
        .unwrap();
}
