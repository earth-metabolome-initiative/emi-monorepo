//! Submodule defining the Vinesh plan procedure template.
use web_common_traits::database::Insertable;
use core_structures::{ProcedureTemplate, User};
use core_structures::tables::insertables::ProcedureTemplateSettable;
use web_common_traits::database::InsertableVariant;



/// The name of the Vinesh plan procedure template.
pub const VINESH_PLAN: &str = "Vinesh Plan";

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
pub fn init_vinesh_plan(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<ProcedureTemplate> {
    let vinesh_plan = ProcedureTemplate::new()
        .name(VINESH_PLAN)?
        .description("Vinesh Plan procedure template")?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    Ok(vinesh_plan)
}
