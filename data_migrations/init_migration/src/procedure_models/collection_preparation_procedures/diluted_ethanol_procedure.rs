//! Submodule defining the Ethanol 70 percent procedure creation.

use core_structures::{ProcedureModel, User};
use diesel::OptionalExtension;
use web_common_traits::database::{Insertable, InsertableVariant};

/// The name of the DBGI Collection preparation procedure model.
pub const E70_ETHANOL: &str = "Ethanol 70 percent";

/// Initializes the DBGI Collection preparation procedure model in the database.
///
/// # Arguments
///
/// * `user` - The user who is creating the procedure model.
/// * `conn` - The database connection to use for the insertion.
///
/// # Panics
///
/// * If the connection fails to insert the procedure model.
/// * If the procedure model building fails.
pub(crate) fn init_ethanol_70_percent(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<ProcedureModel> {
    if let Some(procedure) = ProcedureModel::from_name(E70_ETHANOL, conn).optional()? {
        return Ok(procedure);
    }

    Ok(ProcedureModel::new()
        .name(E70_ETHANOL)
        ?
        .description(
			"Procedure model for Ethanol 70 percent Solvent preparation, used in various cleaning procedures.",
        )
        ?
        .created_by(user.id)
        ?
        .insert(user.id, conn)
		?)
}
