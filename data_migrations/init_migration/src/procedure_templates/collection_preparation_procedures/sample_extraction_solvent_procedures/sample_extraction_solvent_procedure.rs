//! Submodule defining the procedure template to create a standard sample
//! extraction solvent.
use core_structures::{ProcedureTemplate, User, tables::insertables::ProcedureTemplateBuildable};
use diesel::OptionalExtension;
use web_common_traits::database::{Insertable, InsertableVariant};
/// The name of the sample extraction solvent procedure template.
pub const SAMPLE_EXTRACTION_SOLVENT: &str = "Sample extraction solvent";

/// Creates a procedure template for sample extraction solvent preparation.
///
/// # Arguments
///
/// * `user` - The user who is creating the procedure template.
/// * `conn` - The database connection to use for the insertion.
pub(crate) fn init_sample_extraction_solvent_procedure(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<ProcedureTemplate> {
    if let Some(procedure) =
        ProcedureTemplate::from_name(SAMPLE_EXTRACTION_SOLVENT, conn).optional()?
    {
        return Ok(procedure);
    }

    Ok(ProcedureTemplate::new()
        .name(SAMPLE_EXTRACTION_SOLVENT)?
        .description(
			"procedure template for sample extraction solvent preparation, used in various laboratory and field procedures.",
        )?
        .created_by(user.id)?
        .insert(user.id, conn)?)
}
