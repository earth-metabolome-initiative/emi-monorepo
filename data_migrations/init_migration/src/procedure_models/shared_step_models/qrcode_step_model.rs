//! Submodule creating a new labelling step model.

use core_structures::{
    ProcedureModel, ProcedureModelContainerCategory, ProcedureModelNameplateCategory, StepModel,
    StepModelContainerCategory, StepModelNameplateCategory, User,
};
use diesel_async::AsyncPgConnection;
use step_model_categories::StepModelCategory;
use web_common_traits::{
    database::{BackendInsertableVariant, Insertable},
    prelude::Builder,
};

pub(crate) async fn init_bottle_qrcode_step_model(
    user: &User,
    procedure: &ProcedureModel,
    procedure_nameplate_category: &ProcedureModelNameplateCategory,
    procedure_container_category: &ProcedureModelContainerCategory,
    bottle_description: &str,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let qrcode_materials_photograph = core_structures::create_photograph(
        include_bytes!("../../../images/cleaning.jpg"),
        user,
        portal_conn,
    )
    .await?;

    let qrcode_materials_step_model = StepModel::new()
        .procedure_model_id(procedure.id)?
        .name("Labelling step")?
        .description(format!("Apply the qrcode sticker on the new {bottle_description} bottle."))?
        // https://fontawesome.com/icons/qrcode?f=classic&s=solid
        .icon("qrcode")?
        .photograph_id(qrcode_materials_photograph.id)?
        .step_model_category(StepModelCategory::Labelling)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let _step_model_container_category = StepModelContainerCategory::new()
        .step_model_id(qrcode_materials_step_model.id)?
        .procedure_model_container_category_id(procedure_container_category.id)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let _step_model_nameplate_category = StepModelNameplateCategory::new()
        .step_model_id(qrcode_materials_step_model.id)?
        .procedure_model_nameplate_category_id(procedure_nameplate_category.id)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    Ok(())
}

pub(crate) async fn init_falcon_qrcode_step_model(
    user: &User,
    procedure: &ProcedureModel,
    procedure_nameplate_category: &ProcedureModelNameplateCategory,
    procedure_container_category: &ProcedureModelContainerCategory,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let qrcode_materials_photograph = core_structures::create_photograph(
        include_bytes!("../../../images/cleaning.jpg"),
        user,
        portal_conn,
    )
    .await?;

    let qrcode_materials_step_model = StepModel::new()
        .procedure_model_id(procedure.id)?
        .name("Labelling step")?
        .description("Apply a QR code label on the sample container")?
        // https://fontawesome.com/icons/qrcode?f=classic&s=solid
        .icon("qrcode")?
        .snoozable(true)?
        .photograph_id(qrcode_materials_photograph.id)?
        .step_model_category(StepModelCategory::Labelling)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let _step_model_container_category = StepModelContainerCategory::new()
        .step_model_id(qrcode_materials_step_model.id)?
        .procedure_model_container_category_id(procedure_container_category.id)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let _step_model_nameplate_category = StepModelNameplateCategory::new()
        .step_model_id(qrcode_materials_step_model.id)?
        .procedure_model_nameplate_category_id(procedure_nameplate_category.id)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    Ok(())
}

pub(crate) async fn init_box_qrcode_step_model(
    user: &User,
    procedure: &ProcedureModel,
    procedure_nameplate_category: &ProcedureModelNameplateCategory,
    procedure_container_category: &ProcedureModelContainerCategory,
    portal_conn: &mut AsyncPgConnection,
) -> Result<(), crate::error::Error> {
    let qrcode_materials_photograph = core_structures::create_photograph(
        include_bytes!("../../../images/cleaning.jpg"),
        user,
        portal_conn,
    )
    .await?;

    let qrcode_materials_step_model = StepModel::new()
        .procedure_model_id(procedure.id)?
        .name("Labelling step")?
        .description("Apply a QR code label on the box or just use the one already on the box.")?
        // https://fontawesome.com/icons/qrcode?f=classic&s=solid
        .icon("qrcode")?
        .snoozable(true)?
        .photograph_id(qrcode_materials_photograph.id)?
        .step_model_category(StepModelCategory::Labelling)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let _step_model_container_category = StepModelContainerCategory::new()
        .step_model_id(qrcode_materials_step_model.id)?
        .procedure_model_container_category_id(procedure_container_category.id)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    let _step_model_nameplate_category = StepModelNameplateCategory::new()
        .step_model_id(qrcode_materials_step_model.id)?
        .procedure_model_nameplate_category_id(procedure_nameplate_category.id)?
        .created_by(user.id)?
        .build()?
        .backend_insert(portal_conn)
        .await?;

    Ok(())
}
