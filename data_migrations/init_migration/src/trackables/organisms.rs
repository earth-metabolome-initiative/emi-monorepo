//! Submodule to initialize the `instruments` in the database.

use core_structures::{Trackable, User};
use diesel::PgConnection;
use web_common_traits::database::{Insertable, InsertableVariant};

pub(crate) const ORGANISM: &str = "Organism";
pub(crate) const SAMPLE: &str = "Sample";

pub(crate) fn init_organisms(user: &User, conn: &mut PgConnection) -> anyhow::Result<()> {
    let _organisms = Trackable::new()
        .name(ORGANISM.to_owned())?
        .description("Organisms used in laboratory procedures".to_owned())?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let _sample = Trackable::new()
        .name(SAMPLE.to_owned())?
        .description("Sample of an organism".to_owned())?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    Ok(())
}
