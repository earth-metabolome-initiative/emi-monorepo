//! Submodule to initialize the `ppe` in the database.

use core_structures::{
    PersonalProtectiveEquipmentModel, User,
    tables::insertables::{AssetModelSettable, PersonalProtectiveEquipmentModelAttribute},
};
use diesel::PgConnection;
use web_common_traits::database::{DispatchableInsertableVariant, InsertError, Insertable};

/// Returns the PPE model for gloves, creating it if it does not
/// exist.
///
/// # Arguments
///
/// * `user` - The user who is creating the PPE model.
/// * `conn` - The database connection to use for the insertion.
///
/// # Errors
///
/// * If the connection to the database fails.
/// * If the insertion fails.
pub(crate) fn glove_model(
    user: &User,
    conn: &mut PgConnection,
) -> Result<PersonalProtectiveEquipmentModel, InsertError<PersonalProtectiveEquipmentModelAttribute>>
{
    const GLOVE_MODEL: &str = "Glove";

    if let Ok(bead) = PersonalProtectiveEquipmentModel::from_name(GLOVE_MODEL, conn) {
        return Ok(bead);
    }

    PersonalProtectiveEquipmentModel::new()
        .name(GLOVE_MODEL)?
        .description("Latex or nitrile gloves used for personal protection.")?
        .created_by(user)?
        .insert(user.id, conn)
}
