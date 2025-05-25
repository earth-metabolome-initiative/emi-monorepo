use container_categories::ContainerCategory;
use core_structures::{
    ProcedureModel, ProcedureModelContainerCategory, ProcedureModelNameplateCategory, User,
};
use diesel_async::AsyncPgConnection;
use nameplate_categories::NameplateCategory;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable, InsertableVariant},
    prelude::Builder,
};

mod ethanol_aliquoting_step_model;
mod qrcode_step_model;
mod rinsing_step_model_95;
mod water_aliquoting_step_model;

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
        .container_category(ContainerCategory::Bottle)?
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

    rinsing_step_model_95::init_rinsing_step_model_95(
        user,
        &ethanol_70,
        &ethanol_70_container_category,
        portal_conn,
    )
    .await?;
    qrcode_step_model::init_qrcode_step_model(
        user,
        &ethanol_70,
        &ethanol_70_nameplate_category,
        &ethanol_70_container_category,
        portal_conn,
    )
    .await?;
    ethanol_aliquoting_step_model::init_ethanol_aliquoting_step_model(
        user,
        &ethanol_70,
        &ethanol_70_container_category,
        portal_conn,
    )
    .await?;
    water_aliquoting_step_model::init_water_aliquoting_step_model(
        user,
        &ethanol_70,
        &ethanol_70_container_category,
        portal_conn,
    )
    .await?;

    Ok(())
}
