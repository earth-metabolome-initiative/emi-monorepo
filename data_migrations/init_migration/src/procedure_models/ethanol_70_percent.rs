use container_categories::ContainerCategory;
use core_structures::{
    ProcedureModel, ProcedureModelContainerCategory, ProcedureModelNameplateCategory,
    ProcedureModelToolCategory, User,
};
use diesel_async::AsyncPgConnection;
use nameplate_categories::NameplateCategory;
use tool_categories::ToolCategory;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable, InsertableVariant},
    prelude::Builder,
};

use super::shared_step_models::{
    init_bottle_qrcode_step_model, init_rinsing_step_model_95, init_water_aliquoting_step_model,
};

mod ethanol_aliquoting_step_model;

pub(super) async fn init_ethanol_70_percent(
    user: &User,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let ethanol_70: ProcedureModel = ProcedureModel::new()
        .name("Ethanol 70%")?
        .description("Ethanol mixed with water used to sterilize all sorts of surfaces.")?
        .icon("blender")?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let ethanol_70_container_category = ProcedureModelContainerCategory::new()
        .procedure_model_id(ethanol_70.id)?
        .container_category(ContainerCategory::Bottle { liters: 5.0 })?
        .created_by(user.id)?
        .build()?
        .insert(&user.id, portal_conn)
        .await?;

    let ethanol_70_nameplate_category = ProcedureModelNameplateCategory::new()
        .procedure_model_id(ethanol_70.id)?
        .nameplate_category(NameplateCategory::SemiPermanent)?
        .created_by(user.id)?
        .build()?
        .insert(&user.id, portal_conn)
        .await?;

    let ethanol_70_graduated_cylinder = ProcedureModelToolCategory::new()
        .procedure_model_id(ethanol_70.id)?
        .tool_category(ToolCategory::GraduatedCylinder)?
        .created_by(user.id)?
        .build()?
        .insert(&user.id, portal_conn)
        .await?;

    init_rinsing_step_model_95(user, &ethanol_70, &ethanol_70_container_category, portal_conn)
        .await?;
    init_bottle_qrcode_step_model(
        user,
        &ethanol_70,
        &ethanol_70_nameplate_category,
        &ethanol_70_container_category,
        "ethanol 70%",
        portal_conn,
    )
    .await?;
    ethanol_aliquoting_step_model::init_ethanol_aliquoting_step_model(
        user,
        &ethanol_70,
        &ethanol_70_container_category,
        &[&ethanol_70_graduated_cylinder],
        portal_conn,
    )
    .await?;
    init_water_aliquoting_step_model(
        user,
        &ethanol_70,
        &ethanol_70_container_category,
        &[&ethanol_70_graduated_cylinder],
        1.5,
        "Aliquoting 1.5 liters of Distilled Water to prepare Ethanol 70 percent.",
        portal_conn,
    )
    .await?;

    Ok(())
}
