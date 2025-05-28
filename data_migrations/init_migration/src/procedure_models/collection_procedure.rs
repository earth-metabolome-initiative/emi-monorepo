use container_categories::ContainerCategory;
use core_structures::{ProcedureModel, User};
use diesel_async::AsyncPgConnection;
use nameplate_categories::NameplateCategory;
use tool_categories::ToolCategory;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable},
    prelude::Builder,
};

use super::shared_step_models::init_falcon_qrcode_step_model;

pub(crate) const COLLECTION_PROCEDURE: &str = "Collection procedure";

pub(super) async fn init_collection_procedure_model(
    user: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let dbgi_collection_procedure: ProcedureModel = ProcedureModel::new()
        .name(COLLECTION_PROCEDURE)?
        .description("Collecting samples in the project, using digital organism UUIDs.")?
        .icon("blender")?
        .repeatable(true)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let _organism_nameplate =
        dbgi_collection_procedure.nameplate(NameplateCategory::Digital, user, portal_conn).await?;

    let falcon_nameplate = dbgi_collection_procedure
        .nameplate(NameplateCategory::SemiPermanent, user, portal_conn)
        .await?;

    let falcon = dbgi_collection_procedure
        .container(ContainerCategory::SampleContainer, user, portal_conn)
        .await?;

    let _wrapper = dbgi_collection_procedure.tool(ToolCategory::Wrapper, user, portal_conn).await?;

    let _paper_towel =
        dbgi_collection_procedure.tool(ToolCategory::PaperTowels, user, portal_conn).await?;

    let _cutting_tool =
        dbgi_collection_procedure.tool(ToolCategory::CuttingTool, user, portal_conn).await?;

    let _gloves = dbgi_collection_procedure.tool(ToolCategory::Gloves, user, portal_conn).await?;

    init_falcon_qrcode_step_model(
        user,
        &dbgi_collection_procedure,
        &falcon_nameplate,
        &falcon,
        portal_conn,
    )
    .await?;

    Ok(())
}
