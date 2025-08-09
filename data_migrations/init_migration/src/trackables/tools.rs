//! Submodule to initialize the `instruments` in the database.

use core_structures::{Trackable, User};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

pub const METAL_BEADS: &str = "Metal Beads";
pub const METAL_BEADS_3MM: &str = "Metal Beads 3mm";

pub(crate) fn init_tools(user: &User, conn: &mut PgConnection) -> anyhow::Result<()> {
    let metal_beads = core_structures::Trackable::new()
        .name(METAL_BEADS.to_owned())?
        .description("Metal beads used in laboratory procedures".to_owned())?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let _metal_beads_3mm = Trackable::new()
        .name(METAL_BEADS_3MM.to_owned())?
        .description("Metal beads 3mm used in laboratory procedures".to_owned())?
        .parent(Some(metal_beads.id))?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    Ok(())
}
