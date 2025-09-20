//! Submodule defining the Ethanol 70 percent procedure creation.

use core_structures::{ProcedureTemplate, User, tables::insertables::ProcedureTemplateSettable};
use diesel::OptionalExtension;
use web_common_traits::database::{DispatchableInsertableVariant, Insertable};

/// Initializes the DBGI Collection preparation procedure template in the
/// database.
///
/// # Arguments
///
/// * `user` - The user who is creating the procedure template.
/// * `conn` - The database connection to use for the insertion.
///
/// # Panics
///
/// * If the connection fails to insert the procedure template.
/// * If the procedure template building fails.
pub(crate) fn ethanol_70_percent(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<ProcedureTemplate> {
    const E70_ETHANOL: &str = "Ethanol 70 percent";

    if let Some(procedure) = ProcedureTemplate::from_name(E70_ETHANOL, conn).optional()? {
        return Ok(procedure);
    }

    Ok(ProcedureTemplate::new()
        .name(E70_ETHANOL)?
        .description(
			"procedure template for Ethanol 70 percent Solvent preparation, used in various cleaning procedures.",
        )?
        .created_by(user)?
        .insert(user.id, conn)?)
}
