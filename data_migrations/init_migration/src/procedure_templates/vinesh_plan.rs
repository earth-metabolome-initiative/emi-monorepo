//! Submodule defining the Vinesh plan procedure template.
use core_structures::{ProcedureTemplate, User, tables::insertables::ProcedureTemplateSettable};
use diesel::OptionalExtension;
use web_common_traits::database::{DispatchableInsertableVariant, Insertable};
/// Initializes the Vinesh plan procedure template in the database.
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
///
/// # Errors
///
/// * If the connection to the database fails.
pub fn vinesh_plan(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<ProcedureTemplate> {
    const VINESH_PLAN: &str = "Vinesh Plan";

    if let Some(procedure) = ProcedureTemplate::from_name(VINESH_PLAN, conn).optional()? {
        return Ok(procedure);
    }

    let vinesh_plan = ProcedureTemplate::new()
        .name(VINESH_PLAN)?
        .description("Vinesh Plan procedure template")?
        .created_by(user)?
        .insert(user.id, conn)?;

    Ok(vinesh_plan)
}
