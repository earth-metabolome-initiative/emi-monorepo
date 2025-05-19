//! Submodule defining the init migrations for the procedure models.

use container_categories::ContainerCategory;
use core_structures::{
    ProcedureModel, ProcedureModelContainerCategory, ProcedureModelToolCategory, User,
};
use diesel_async::AsyncPgConnection;
use tool_categories::ToolCategory;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable},
    prelude::Builder,
};

async fn init_emi_solvent_procedure_models(
    user: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let emi_solvent_procedure: ProcedureModel = ProcedureModel::new()
        .name("EMI Solvent 0.5L")?
        .description("A procedure to create half a liter of EMI Solvent.")?
        .icon("blender")?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    ProcedureModelToolCategory::new()
        .procedure_model_id(emi_solvent_procedure.id)?
        .tool_category(ToolCategory::VolumeMeasuringTool)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    ProcedureModelContainerCategory::new()
        .procedure_model_id(emi_solvent_procedure.id)?
        .container_category(ContainerCategory::Bottle)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    Ok(())
}

pub(crate) async fn init_procedure_models(
    user: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    init_emi_solvent_procedure_models(user, portal_conn).await?;
    Ok(())
}
