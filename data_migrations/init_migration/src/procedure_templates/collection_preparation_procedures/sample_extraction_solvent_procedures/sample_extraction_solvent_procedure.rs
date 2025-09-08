//! Submodule defining the procedure template to create a standard sample
//! extraction solvent.
use core_structures::{
    ProcedureTemplate, ProcedureTemplateAssetModel, User,
    tables::insertables::{ProcedureTemplateAssetModelSettable, ProcedureTemplateSettable},
};
use diesel::OptionalExtension;
use web_common_traits::database::{Insertable, InsertableVariant};

use crate::asset_models::containers::bottles::bottle_1l;

/// Creates a procedure template for sample extraction solvent preparation.
///
/// # Arguments
///
/// * `user` - The user who is creating the procedure template.
/// * `conn` - The database connection to use for the insertion.
pub(crate) fn sample_extraction_solvent_procedure(
    user: &User,
    conn: &mut diesel::PgConnection,
) -> anyhow::Result<(ProcedureTemplate, ProcedureTemplateAssetModel)> {
    const SAMPLE_EXTRACTION_SOLVENT: &str = "Sample extraction solvent";
    if let Some(procedure) =
        ProcedureTemplate::from_name(SAMPLE_EXTRACTION_SOLVENT, conn).optional()?
    {
        let ptam = ProcedureTemplateAssetModel::from_name_and_procedure_template(
            SAMPLE_EXTRACTION_SOLVENT,
            procedure.procedure_template,
            conn,
        )?;
        return Ok((procedure, ptam));
    }

    let procedure_template = ProcedureTemplate::new()
        .name(SAMPLE_EXTRACTION_SOLVENT)?
        .description(
			"procedure template for sample extraction solvent preparation, used in various laboratory and field procedures.",
        )?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    let ptam = ProcedureTemplateAssetModel::new()
        .procedure_template(procedure_template.procedure_template)?
        .asset_model(bottle_1l(user, conn)?.id)?
        .name(SAMPLE_EXTRACTION_SOLVENT)?
        .created_by(user.id)?
        .insert(user.id, conn)?;

    Ok((procedure_template, ptam))
}
