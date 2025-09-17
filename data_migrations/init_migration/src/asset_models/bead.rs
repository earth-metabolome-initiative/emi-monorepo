//! Submodule to initialize the `instruments` in the database.

use core_structures::{
    BeadModel, User,
    tables::insertables::{AssetModelSettable, BeadModelAttribute, BeadModelSettable},
};
use diesel::PgConnection;
use web_common_traits::database::{DispatchableInsertableVariant, InsertError, Insertable};

/// Returns the bead model for 3mm metal beads, creating it if it does not
/// exist.
///
/// # Arguments
///
/// * `user` - The user who is creating the bead model.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
/// * If the insertion fails.
pub(crate) fn bead_3mm(
    user: &User,
    conn: &mut PgConnection,
) -> Result<BeadModel, InsertError<BeadModelAttribute>> {
    const METAL_BEADS_3MM: &str = "Metal Bead 3mm";

    if let Ok(bead) = BeadModel::from_name(METAL_BEADS_3MM, conn) {
        return Ok(bead);
    }

    BeadModel::new()
        .name(METAL_BEADS_3MM)?
        .description("Metal bead of 3mm used primarely in ball milling procedures.")?
        .diameter_millimeters(3.0)?
        .created_by(user)?
        .insert(user.id, conn)
}
