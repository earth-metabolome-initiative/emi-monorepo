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

mod formic_acid_aliquoting_step_model;
mod methanol_aliquoting_step_model;

use super::shared_step_models::{
    init_qrcode_step_model, init_rinsing_step_model_95, init_water_aliquoting_step_model,
};

pub(super) async fn init_emi_solvent_procedure_models(
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

    let emi_solvent_procedure_container_category = ProcedureModelContainerCategory::new()
        .procedure_model_id(emi_solvent_procedure.id)?
        .container_category(ContainerCategory::Bottle { liters: 0.5 })?
        .created_by(user.id)?
        .build()?
        .insert(&user.id, portal_conn)
        .await?;

    let emi_solvent_procedure_nameplate_category = ProcedureModelNameplateCategory::new()
        .procedure_model_id(emi_solvent_procedure.id)?
        .nameplate_category(NameplateCategory::SemiPermanent)?
        .created_by(user.id)?
        .build()?
        .insert(&user.id, portal_conn)
        .await?;

    let graduated_cylinder = ProcedureModelToolCategory::new()
        .procedure_model_id(emi_solvent_procedure.id)?
        .tool_category(ToolCategory::GraduatedCylinder)?
        .created_by(user.id)?
        .build()?
        .insert(&user.id, portal_conn)
        .await?;

    let pipette = ProcedureModelToolCategory::new()
        .procedure_model_id(emi_solvent_procedure.id)?
        .tool_category(ToolCategory::Pipette)?
        .created_by(user.id)?
        .build()?
        .insert(&user.id, portal_conn)
        .await?;

    let pipette_tip = ProcedureModelToolCategory::new()
        .procedure_model_id(emi_solvent_procedure.id)?
        .tool_category(ToolCategory::PipetteTip)?
        .created_by(user.id)?
        .build()?
        .insert(&user.id, portal_conn)
        .await?;

    let pipetting_container = ProcedureModelToolCategory::new()
        .procedure_model_id(emi_solvent_procedure.id)?
        .tool_category(ToolCategory::PipettingContainer)?
        .created_by(user.id)?
        .build()?
        .insert(&user.id, portal_conn)
        .await?;

    init_rinsing_step_model_95(
        user,
        &emi_solvent_procedure,
        &emi_solvent_procedure_container_category,
        portal_conn,
    )
    .await?;
    init_qrcode_step_model(
        user,
        &emi_solvent_procedure,
        &emi_solvent_procedure_nameplate_category,
        &emi_solvent_procedure_container_category,
        "EMI Solvent",
        portal_conn,
    )
    .await?;
    methanol_aliquoting_step_model::init_methanol_aliquoting_step_model(
        user,
        &emi_solvent_procedure,
        &emi_solvent_procedure_container_category,
        &[&graduated_cylinder],
        portal_conn,
    )
    .await?;
    init_water_aliquoting_step_model(
        user,
        &emi_solvent_procedure,
        &emi_solvent_procedure_container_category,
        &[&graduated_cylinder],
        0.1,
        "Aliquoting 0.1 liters of Distilled Water to prepare the EMI solvent.",
        portal_conn,
    )
    .await?;
    formic_acid_aliquoting_step_model::init_formic_acid_aliquoting_step_model(
        user,
        &emi_solvent_procedure,
        &emi_solvent_procedure_container_category,
        &[&pipette, &pipette_tip, &pipetting_container],
        portal_conn,
    )
    .await?;

    Ok(())
}
